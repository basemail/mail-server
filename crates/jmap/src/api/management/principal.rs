/*
 * Copyright (c) 2023 Stalwart Labs Ltd.
 *
 * This file is part of Stalwart Mail Server.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of
 * the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 * in the LICENSE file at the top-level directory of this distribution.
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * You can be released from the requirements of the AGPLv3 license by
 * purchasing a commercial license. Please contact licensing@stalw.art
 * for more details.
*/

use std::sync::Arc;

use directory::{
    backend::internal::{
        lookup::DirectoryStore, manage::ManageDirectory, PrincipalField, PrincipalUpdate,
        PrincipalValue,
    },
    DirectoryError, DirectoryInner, ManagementError, Principal, QueryBy, Type,
};

use hyper::{header, Method, StatusCode};
use jmap_proto::error::request::RequestError;
use serde_json::json;
use utils::url_params::UrlParams;

use crate::{
    api::{http::ToHttpResponse, HttpRequest, HttpResponse, JsonResponse},
    auth::AccessToken,
    JMAP,
};

use super::{decode_path_element, ManagementApiError};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PrincipalResponse {
    #[serde(default)]
    pub id: u32,
    #[serde(rename = "type")]
    pub typ: Type,
    #[serde(default)]
    pub quota: u64,
    #[serde(rename = "usedQuota")]
    #[serde(default)]
    pub used_quota: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub emails: Vec<String>,
    #[serde(default)]
    pub secrets: Vec<String>,
    #[serde(rename = "memberOf")]
    #[serde(default)]
    pub member_of: Vec<String>,
    #[serde(default)]
    pub members: Vec<String>,
    #[serde(default)]
    pub description: Option<String>,
}

impl JMAP {
    pub async fn handle_manage_principal(
        &self,
        req: &HttpRequest,
        path: Vec<&str>,
        body: Option<Vec<u8>>,
    ) -> HttpResponse {
        match (path.get(1), req.method()) {
            (None, &Method::POST) => {
                // Make sure the current directory supports updates
                if let Some(response) = self.assert_supported_directory() {
                    return response;
                }

                // Create principal
                match serde_json::from_slice::<PrincipalResponse>(
                    body.as_deref().unwrap_or_default(),
                ) {
                    Ok(principal) => {
                        match self
                            .core
                            .storage
                            .data
                            .create_account(
                                Principal {
                                    id: principal.id,
                                    typ: principal.typ,
                                    quota: principal.quota,
                                    name: principal.name,
                                    secrets: principal.secrets,
                                    emails: principal.emails,
                                    member_of: principal.member_of,
                                    description: principal.description,
                                },
                                principal.members,
                            )
                            .await
                        {
                            Ok(account_id) => JsonResponse::new(json!({
                                "data": account_id,
                            }))
                            .into_http_response(),
                            Err(err) => err.into_http_response(),
                        }
                    }
                    Err(err) => err.into_http_response(),
                }
            }
            (None, &Method::GET) => {
                // List principal ids
                let params = UrlParams::new(req.uri().query());
                let filter = params.get("filter");
                let typ = params.parse("type");
                let page: usize = params.parse("page").unwrap_or(0);
                let limit: usize = params.parse("limit").unwrap_or(0);

                match self.core.storage.data.list_accounts(filter, typ).await {
                    Ok(accounts) => {
                        let (total, accounts) = if limit > 0 {
                            let offset = page.saturating_sub(1) * limit;
                            (
                                accounts.len(),
                                accounts.into_iter().skip(offset).take(limit).collect(),
                            )
                        } else {
                            (accounts.len(), accounts)
                        };

                        JsonResponse::new(json!({
                                "data": {
                                    "items": accounts,
                                    "total": total,
                                },
                        }))
                        .into_http_response()
                    }
                    Err(err) => err.into_http_response(),
                }
            }
            (Some(name), method) => {
                // Fetch, update or delete principal
                let name = decode_path_element(name);
                let account_id = match self.core.storage.data.get_account_id(name.as_ref()).await {
                    Ok(Some(account_id)) => account_id,
                    Ok(None) => {
                        return RequestError::blank(
                            StatusCode::NOT_FOUND.as_u16(),
                            "Not found",
                            "Account not found.",
                        )
                        .into_http_response();
                    }
                    Err(err) => {
                        return err.into_http_response();
                    }
                };

                match *method {
                    Method::GET => {
                        let result = match self
                            .core
                            .storage
                            .data
                            .query(QueryBy::Id(account_id), true)
                            .await
                        {
                            Ok(Some(principal)) => {
                                self.core.storage.data.map_group_ids(principal).await
                            }
                            Ok(None) => {
                                return RequestError::blank(
                                    StatusCode::NOT_FOUND.as_u16(),
                                    "Not found",
                                    "Account not found.",
                                )
                                .into_http_response()
                            }
                            Err(err) => Err(err),
                        };

                        match result {
                            Ok(principal) => {
                                // Obtain quota usage
                                let mut principal = PrincipalResponse::from(principal);
                                principal.used_quota =
                                    self.get_used_quota(account_id).await.unwrap_or_default()
                                        as u64;

                                // Obtain member names
                                for member_id in self
                                    .core
                                    .storage
                                    .data
                                    .get_members(account_id)
                                    .await
                                    .unwrap_or_default()
                                {
                                    if let Ok(Some(member_principal)) = self
                                        .core
                                        .storage
                                        .data
                                        .query(QueryBy::Id(member_id), false)
                                        .await
                                    {
                                        principal.members.push(member_principal.name);
                                    }
                                }

                                JsonResponse::new(json!({
                                        "data": principal,
                                }))
                                .into_http_response()
                            }
                            Err(err) => err.into_http_response(),
                        }
                    }
                    Method::DELETE => {
                        // Remove FTS index
                        if let Err(err) = self.core.storage.fts.remove_all(account_id).await {
                            return err.into_http_response();
                        }

                        // Delete account
                        match self
                            .core
                            .storage
                            .data
                            .delete_account(QueryBy::Id(account_id))
                            .await
                        {
                            Ok(_) => JsonResponse::new(json!({
                                "data": (),
                            }))
                            .into_http_response(),
                            Err(err) => err.into_http_response(),
                        }
                    }
                    Method::PATCH => {
                        match serde_json::from_slice::<Vec<PrincipalUpdate>>(
                            body.as_deref().unwrap_or_default(),
                        ) {
                            Ok(changes) => {
                                // Make sure the current directory supports updates
                                if let Some(response) = self.assert_supported_directory() {
                                    if changes.iter().any(|change| {
                                        !matches!(
                                            change.field,
                                            PrincipalField::Quota | PrincipalField::Description
                                        )
                                    }) {
                                        return response;
                                    }
                                }

                                match self
                                    .core
                                    .storage
                                    .data
                                    .update_account(QueryBy::Id(account_id), changes)
                                    .await
                                {
                                    Ok(_) => JsonResponse::new(json!({
                                        "data": (),
                                    }))
                                    .into_http_response(),
                                    Err(err) => err.into_http_response(),
                                }
                            }
                            Err(err) => err.into_http_response(),
                        }
                    }
                    _ => RequestError::not_found().into_http_response(),
                }
            }

            _ => RequestError::not_found().into_http_response(),
        }
    }

    pub async fn handle_change_password(
        &self,
        req: &HttpRequest,
        access_token: Arc<AccessToken>,
        body: Option<Vec<u8>>,
    ) -> HttpResponse {
        // Make sure the user authenticated using Basic auth
        if req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .map_or(true, |header| !header.to_lowercase().starts_with("basic "))
        {
            return ManagementApiError::Other {
                details: "Password changes only allowed using Basic auth".into(),
            }
            .into_http_response();
        }

        // Obtain new password
        let new_password = match String::from_utf8(body.unwrap_or_default()) {
            Ok(new_password) if !new_password.is_empty() => new_password,
            _ => {
                return ManagementApiError::Other {
                    details: "Invalid change password request".into(),
                }
                .into_http_response()
            }
        };

        // Handle Fallback admin password changes
        if access_token.is_super_user() && access_token.primary_id() == u32::MAX {
            return match self
                .core
                .storage
                .config
                .set([("authentication.fallback-admin.secret", new_password)])
                .await
            {
                Ok(_) => JsonResponse::new(json!({
                    "data": (),
                }))
                .into_http_response(),
                Err(err) => err.into_http_response(),
            };
        }

        // Make sure the current directory supports updates
        if let Some(response) = self.assert_supported_directory() {
            return response;
        }

        // Update password
        match self
            .core
            .storage
            .data
            .update_account(
                QueryBy::Id(access_token.primary_id()),
                vec![PrincipalUpdate::set(
                    PrincipalField::Secrets,
                    PrincipalValue::StringList(vec![new_password]),
                )],
            )
            .await
        {
            Ok(_) => JsonResponse::new(json!({
                "data": (),
            }))
            .into_http_response(),
            Err(err) => err.into_http_response(),
        }
    }

    pub fn assert_supported_directory(&self) -> Option<HttpResponse> {
        ManagementApiError::UnsupportedDirectoryOperation {
            class: match &self.core.storage.directory.store {
                DirectoryInner::Internal(_) => return None,
                DirectoryInner::Ldap(_) => "LDAP",
                DirectoryInner::Sql(_) => "SQL",
                DirectoryInner::Imap(_) => "IMAP",
                DirectoryInner::Smtp(_) => "SMTP",
                DirectoryInner::Memory(_) => "In-Memory",
                DirectoryInner::Basemail(_) => return None,
            }
            .into(),
        }
        .into_http_response()
        .into()
    }
}

impl From<Principal<String>> for PrincipalResponse {
    fn from(principal: Principal<String>) -> Self {
        PrincipalResponse {
            id: principal.id,
            typ: principal.typ,
            quota: principal.quota,
            name: principal.name,
            emails: principal.emails,
            member_of: principal.member_of,
            description: principal.description,
            secrets: principal.secrets,
            used_quota: 0,
            members: Vec::new(),
        }
    }
}

impl ToHttpResponse for DirectoryError {
    fn into_http_response(self) -> HttpResponse {
        match self {
            DirectoryError::Management(err) => {
                let response = match err {
                    ManagementError::MissingField(field) => ManagementApiError::FieldMissing {
                        field: field.to_string().into(),
                    },
                    ManagementError::AlreadyExists { field, value } => {
                        ManagementApiError::FieldAlreadyExists {
                            field: field.to_string().into(),
                            value: value.into(),
                        }
                    }
                    ManagementError::NotFound(details) => ManagementApiError::NotFound {
                        item: details.into(),
                    },
                };
                JsonResponse::new(response).into_http_response()
            }
            DirectoryError::Unsupported => JsonResponse::new(ManagementApiError::Unsupported {
                details: "Requested action is unsupported".into(),
            })
            .into_http_response(),
            err => {
                tracing::warn!(
                    context = "directory",
                    event = "error",
                    reason = ?err,
                    "Directory error"
                );

                RequestError::internal_server_error().into_http_response()
            }
        }
    }
}
