/*
 * Copyright (c) 2023 Stalwart Labs Ltd.
 *
 * This file is part of the Stalwart Mail Server.
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

pub mod assign_id;
pub mod blob;
pub mod ops;
pub mod query;

use std::io::Read;

use store::{config::ConfigStore, FtsStore};
use utils::config::Config;

pub struct TempDir {
    pub path: std::path::PathBuf,
}

const CONFIG: &str = r#"
[store."s3"]
type = "s3"
access-key = "minioadmin"
secret-key = "minioadmin"
region = "eu-central-1"
endpoint = "http://localhost:9000"
bucket = "tmp"

[store."fs"]
type = "fs"
path = "{TMP}"

[store."rocksdb"]
type = "rocksdb"
path = "{TMP}/rocksdb"

[store."foundationdb"]
type = "foundationdb"

[store."sqlite"]
type = "sqlite"
path = "{TMP}/sqlite.db"

[store."postgresql"]
type = "postgresql"
host = "localhost"
port = 5432
database = "stalwart"
user = "postgres"
password = "mysecretpassword"

[store."mysql"]
type = "mysql"
host = "localhost"
port = 3307
database = "stalwart"
user = "root"
password = "password"
"#;

#[tokio::test]
pub async fn store_tests() {
    let insert = true;
    let temp_dir = TempDir::new("store_tests", insert);
    let config = Config::new(&CONFIG.replace("{TMP}", &temp_dir.path.to_string_lossy())).unwrap();
    let stores = config.parse_stores().await.unwrap();

    for (store_id, store) in stores.stores {
        println!("Testing store {}...", store_id);
        if insert {
            store.destroy().await;
        }
        ops::test(store.clone()).await;
        query::test(store.clone(), FtsStore::Store(store.clone()), insert).await;
        assign_id::test(store).await;
    }

    if insert {
        temp_dir.delete();
    }
}

pub fn deflate_artwork_data() -> Vec<u8> {
    let mut csv_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    csv_path.push("resources");
    csv_path.push("artwork_data.csv.gz");

    let mut decoder = flate2::bufread::GzDecoder::new(std::io::BufReader::new(
        std::fs::File::open(csv_path).unwrap(),
    ));
    let mut result = Vec::new();
    decoder.read_to_end(&mut result).unwrap();
    result
}

impl TempDir {
    pub fn new(name: &str, delete_if_exists: bool) -> Self {
        let mut path = std::env::temp_dir();
        path.push(name);
        if delete_if_exists && path.exists() {
            std::fs::remove_dir_all(&path).unwrap();
        }
        std::fs::create_dir_all(&path).unwrap();
        Self { path }
    }

    pub fn delete(&self) {
        std::fs::remove_dir_all(&self.path).unwrap();
    }
}
