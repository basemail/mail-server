pub use basemail_account::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod basemail_account {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("result"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("changeUsername"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("changeUsername"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("oldUsername_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newUsername_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createAccount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createAccount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("username_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deleteAccount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deleteAccount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("accountId_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAccounts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAccounts"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("holder_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getApproved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getApproved"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("result"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getUsernames"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getUsernames"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("holder_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("holderAccounts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("holderAccounts"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("holder"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("accounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("idCounter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("idCounter"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("idToName"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("idToName"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("accountId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("username"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("result"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nameToId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nameToId"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("username"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("accountId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ownerOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ownerOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("result"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("isApproved"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("result"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenURI"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenURI"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferUsername"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferUsername"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "usernameToTransfer_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newUsername_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AccountCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AccountCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("accountId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("username"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AccountDeleted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AccountDeleted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("accountId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("isApproved"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UsernameChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("UsernameChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("accountId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldUsername"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newUsername"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AccountAlreadyExists"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccountAlreadyExists",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AccountBalanceOverflow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccountBalanceOverflow",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AccountDoesNotExist"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccountDoesNotExist",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BalanceQueryForZeroAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BalanceQueryForZeroAddress",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotOwnerNorApproved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NotOwnerNorApproved",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OnlyTokenHolder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("OnlyTokenHolder"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenAlreadyExists"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("TokenAlreadyExists"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenDoesNotExist"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("TokenDoesNotExist"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransferFromIncorrectOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TransferFromIncorrectOwner",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "TransferToNonERC721ReceiverImplementer",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TransferToNonERC721ReceiverImplementer",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransferToZeroAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TransferToZeroAddress",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UsernameInvalid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("UsernameInvalid"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static BASEMAILACCOUNT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x01`\0Ua\x19\xEE\x80a\0%`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x01@W`\x005`\xE0\x1C\x80cB\x84.\x0E\x11a\0\xB6W\x80c\xA2,\xB4e\x11a\0oW\x80c\xA2,\xB4e\x14a\x03\xF4W\x80c\xB8\x8DO\xDE\x14a\x04\x14W\x80c\xC8{V\xDD\x14a\x04'W\x80c\xE0\x1E\xCFK\x14a\x04UW\x80c\xE9\x85\xE9\xC5\x14a\x04uW\x80c\xEB\x08\xAB(\x14a\x04\xABW`\0\x80\xFD[\x80cB\x84.\x0E\x14a\x03&W\x80ccR!\x1E\x14a\x039W\x80ce\xA9\xAF\x16\x14a\x03YW\x80cl\x86\xA40\x14a\x03yW\x80cp\xA0\x821\x14a\x03\xA6W\x80c\x95\xD8\x9BA\x14a\x03\xC6W`\0\x80\xFD[\x80c\x15*\x1C\x04\x11a\x01\x08W\x80c\x15*\x1C\x04\x14a\x02KW\x80c#\xB8r\xDD\x14a\x02kW\x80c&Y\xF3\x94\x14a\x02~W\x80c/\xB1'\x1D\x14a\x02\x9EW\x80c1\0\xCE\xB5\x14a\x02\xBEW\x80c6D\x10\xB3\x14a\x02\xEBW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01EW\x80c\x06\xFD\xDE\x03\x14a\x01\x97W\x80c\x08\x18\x12\xFC\x14a\x01\xD1W\x80c\t^\xA7\xB3\x14a\x02\tW\x80c\x13\x8A\xAE<\x14a\x02\x1EW[`\0\x80\xFD[4\x80\x15a\x01QW`\0\x80\xFD[Pa\x01\x82a\x01`6`\x04a\x14!V[c\x01\xFF\xC9\xA7`\xE0\x91\x90\x91\x1C\x90\x81\x14c\x80\xACX\xCD\x82\x14\x17c[^\x13\x9F\x90\x91\x14\x17\x90V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xA3W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x08\x81Rg\x10\x98\\\xD9[XZ[`\xC2\x1B` \x82\x01R[`@Qa\x01\x8E\x91\x90a\x14\x91V[4\x80\x15a\x01\xDDW`\0\x80\xFD[Pa\x01\xF1a\x01\xEC6`\x04a\x14\xABV[a\x04\xC1V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x8EV[a\x02\x1Ca\x02\x176`\x04a\x14\xDBV[a\x05\0V[\0[4\x80\x15a\x02*W`\0\x80\xFD[Pa\x02>a\x0296`\x04a\x15\x05V[a\x05\x0FV[`@Qa\x01\x8E\x91\x90a\x15 V[4\x80\x15a\x02WW`\0\x80\xFD[Pa\x02\x1Ca\x02f6`\x04a\x15\xCBV[a\x06EV[a\x02\x1Ca\x02y6`\x04a\x167V[a\x06\x92V[4\x80\x15a\x02\x8AW`\0\x80\xFD[Pa\x02\x1Ca\x02\x996`\x04a\x16sV[a\x07\x9EV[4\x80\x15a\x02\xAAW`\0\x80\xFD[Pa\x02\x1Ca\x02\xB96`\x04a\x14\xABV[a\x07\xBEV[4\x80\x15a\x02\xCAW`\0\x80\xFD[Pa\x02\xDEa\x02\xD96`\x04a\x15\x05V[a\x08tV[`@Qa\x01\x8E\x91\x90a\x16\xC6V[4\x80\x15a\x02\xF7W`\0\x80\xFD[Pa\x03\x18a\x03\x066`\x04a\x14\xABV[`\x02` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\x01\x8EV[a\x02\x1Ca\x0346`\x04a\x167V[a\x08\xE0V[4\x80\x15a\x03EW`\0\x80\xFD[Pa\x01\xF1a\x03T6`\x04a\x14\xABV[a\t\rV[4\x80\x15a\x03eW`\0\x80\xFD[Pa\x03\x18a\x03t6`\x04a\x14\xDBV[a\tKV[4\x80\x15a\x03\x85W`\0\x80\xFD[Pa\x03\x18a\x03\x946`\x04a\x14\xABV[`\x01` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x03\xB2W`\0\x80\xFD[Pa\x03\x18a\x03\xC16`\x04a\x15\x05V[a\t|V[4\x80\x15a\x03\xD2W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81Rd\x10\x93PRS`\xDA\x1B` \x82\x01Ra\x01\xC4V[4\x80\x15a\x04\0W`\0\x80\xFD[Pa\x02\x1Ca\x04\x0F6`\x04a\x17\nV[a\t\xB7V[a\x02\x1Ca\x04\"6`\x04a\x17FV[a\n\rV[4\x80\x15a\x043W`\0\x80\xFD[Pa\x01\xC4a\x04B6`\x04a\x14\xABV[P`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[4\x80\x15a\x04aW`\0\x80\xFD[Pa\x02\x1Ca\x04p6`\x04a\x17\xB5V[a\nhV[4\x80\x15a\x04\x81W`\0\x80\xFD[Pa\x01\x82a\x04\x906`\x04a\x18%V[`\x1CRg\nZ.z\0\0\0\0`\x08R`\0R`0`\x0C T\x90V[4\x80\x15a\x04\xB7W`\0\x80\xFD[Pa\x03\x18`\0T\x81V[`\0\x81`\0Rg>\xC4\x12\xA9\x85-\x17=`\xC1\x1B`\x1CR` `\0 \x82\x01\x82\x01\x80T``\x1Ba\x04\xF6Wc\xCE\xEA!\xB6`\0R`\x04`\x1C\xFD[`\x01\x01T\x92\x91PPV[a\x05\x0B3\x83\x83a\n\xC2V[PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94\x93\x83\x01\x82\x82\x80\x15a\x05nW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x05ZW[PPPPP\x90P`\0\x81Q\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\x96Wa\x05\x96a\x18XV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xC9W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x05\xB4W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x06<Wa\x06\x0C`\x02`\0\x86\x84\x81Q\x81\x10a\x05\xF0Wa\x05\xF0a\x18nV[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 Ta\x0BcV[\x82\x82\x81Q\x81\x10a\x06\x1EWa\x06\x1Ea\x18nV[` \x02` \x01\x01\x81\x90RP\x80\x80a\x064\x90a\x18\x9AV[\x91PPa\x05\xCFV[P\x94\x93PPPPV[`\0a\x06Q\x85\x85a\x0CqV[`\0\x81\x81R`\x01` R`@\x90 T\x90\x91Pa\x06l\x81a\x0C\xB7V[`\0a\x06x\x85\x85a\x0C\xEBV[\x90Pa\x06\x89\x82\x84\x89\x89\x85\x8A\x8Aa\x0F\xB7V[PPPPPPPV[a\x06\x9D\x83\x83\x83a\x10*V[`\0\x81\x81Rg>\xC4\x12\xA9\x85-\x17=`\xC1\x1B3\x17`\x1CR` \x90 \x81\x01\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x94\x93\x84\x16\x93\x81\x16\x91\x90\x82\x86\x14\x83\x02a\x06\xEFWg\xCE\xEA!\xB6\xA1\x14\x81\0\x83\x15`\x02\x1BR`\x04`\x1C\xFD[\x85`\0R\x81`\x01\x01T\x92P\x823\x14\x863\x14\x17a\x07\x1DW`0`\x0C Ta\x07\x1DWcKn\x7F\x18`\0R`\x04`\x1C\xFD[\x82\x15a\x07+W`\0\x82`\x01\x01U[\x85\x85\x18\x18\x90UP`\x1C`\x0C\x81\x81 \x80T`\0\x19\x01\x90U`\0\x84\x90R \x80T`\x01\x01c\xFF\xFF\xFF\xFF\x81\x16\x84\x02a\x07nWg\xEAU;4\x013l\xEA\x84\x15`\x02\x1BR`\x04`\x1C\xFD[\x90U\x80\x82\x84\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF`\08\xA4[PPPV[`\0a\x07\xAA\x83\x83a\x0C\xEBV[\x90Pa\x07\xB8\x84\x82\x85\x85a\x11FV[PPPPV[`\0T\x81\x10\x15\x80a\x07\xECWP`\0\x81\x81Rg>\xC4\x12\xA9\x85-\x17=`\xC1\x1B`\x1CR` \x90 \x81\x01\x81\x01T``\x1B\x15[\x15a\x08\nW`@Qc\xE7n\xA8\x7F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\x13\x81a\x0C\xB7V[a\x08\x1C\x81a\x11\xCBV[`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x80T\x90\x84\x90U\x80\x84R`\x01\x83R\x81\x84 \x93\x90\x93UQ\x83\x81R\x7F\xD7\xCDn\xF0\xA0\x98X]\r\xEE\xEB\x14\xD8i0F{\xEC\xD4_\x18\xBA#\xDA\xB2i\x11\x9B\xF9\xD6\xCA\xB9\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x03` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R``\x93\x92\x83\x01\x82\x82\x80\x15a\x08\xD4W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x08\xC0W[PPPPP\x90P\x91\x90PV[a\x08\xEB\x83\x83\x83a\x06\x92V[\x81;\x15a\x07\x99Wa\x07\x99\x83\x83\x83`@Q\x80` \x01`@R\x80`\0\x81RPa\x11\xD6V[`\0\x81\x81Rg>\xC4\x12\xA9\x85-\x17=`\xC1\x1B`\x1CR` \x90 \x81\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16\x80a\tFWc\xCE\xEA!\xB6`\0R`\x04`\x1C\xFD[\x91\x90PV[`\x03` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\tgW`\0\x80\xFD[\x90`\0R` `\0 \x01`\0\x91P\x91PPT\x81V[`\0\x81a\t\x91Wc\x8FN\xB6\x04`\0R`\x04`\x1C\xFD[g>\xC4\x12\xA9\x85-\x17=`\xC1\x1B`\x1CR\x81`\0Rc\xFF\xFF\xFF\xFF`\x1C`\x0C T\x16\x90P\x91\x90PV[\x80\x15\x15\x90P\x81`\x1CRg\nZ.z\0\0\0\0`\x08R3`\0R\x80`0`\x0C U\x80`\0R\x81``\x1B``\x1C3\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1` `\0\xA3PPV[a\n\x18\x85\x85\x85a\x06\x92V[\x83;\x15a\naWa\na\x85\x85\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x11\xD6\x92PPPV[PPPPPV[`\0a\nt\x85\x85a\x0CqV[`\0\x81\x81R`\x01` R`@\x90 T\x90\x91Pa\n\x8F\x81a\x0C\xB7V[`\0a\n\x9B\x85\x85a\x0C\xEBV[\x90Pa\n\xAC\x82\x84\x89\x89\x85\x8A\x8Aa\x0F\xB7V[a\n\xB8\x88\x84\x89\x89a\x11FV[PPPPPPPPV[`\0\x19``\x1C\x82\x81\x16\x92P\x83\x81\x16\x93P\x81`\0R\x83g>\xC4\x12\xA9\x85-\x17=`\xC1\x1B\x17`\x1CR` `\0 \x82\x01\x82\x01\x80T\x82\x16\x91P\x81a\x0B\tWc\xCE\xEA!\xB6`\0R`\x04`\x1C\xFD[\x81\x85\x14\x85\x15\x17a\x0B/W\x81`\0R`0`\x0C Ta\x0B/WcKn\x7F\x18`\0R`\x04`\x1C\xFD[`\x01\x01\x83\x90U\x81\x83\x82\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%`\08\xA4PPPPV[```\0\x82`@Q` \x01a\x0Bz\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0[\x81\x81\x81Q\x81\x10a\x0B\xA0Wa\x0B\xA0a\x18nV[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x15a\x0B\xC6W\x80a\x0B\xBE\x81a\x18\x9AV[\x91PPa\x0B\x8EV[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xE1Wa\x0B\xE1a\x18XV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0C\x0BW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x06<W\x83\x81\x81Q\x81\x10a\x0C+Wa\x0C+a\x18nV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x0CHWa\x0CHa\x18nV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x0Ci\x81a\x18\x9AV[\x91PPa\x0C\x11V[`\0\x80a\x0C~\x83\x85a\x18\xB3V[`\0\x81\x81R`\x01` R`@\x81 T\x91\x92P\x03a\x0C\xAEW`@Qc\xE7n\xA8\x7F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x90P[\x92\x91PPV[3a\x0C\xC1\x82a\t\rV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0C\xE8W`@Qc\x0B\x98O\x17`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`\0\x80\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP\x82Q\x92\x93PP` \x82\x11\x90P\x80a\r9WP`\x03\x81\x10[\x15a\rWW`@Qc\xB9\xDBqs`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x81\x81\x10\x15a\x0FrW`\0\x83\x82\x81Q\x81\x10a\rvWa\rva\x18nV[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90P\x81\x15\x80a\r\x9DWPa\r\x9A`\x01\x84a\x18\xD1V[\x82\x14[\x15a\x0E\\W`\x03`\xFC\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x10\x80\x15\x90a\r\xCEWP`9`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x11\x15[\x15\x80\x15a\x0E\x04WP`A`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x10\x80\x15\x90a\x0E\x02WP`-`\xF9\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x11\x15[\x15[\x80\x15a\x0E9WP`a`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x10\x80\x15\x90a\x0E7WP`=`\xF9\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x11\x15[\x15[\x15a\x0EWW`@Qc\xB9\xDBqs`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F_V[`\x03`\xFC\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x10\x80\x15\x90a\x0E\x88WP`9`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x11\x15[\x15\x80\x15a\x0E\xBEWP`A`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x10\x80\x15\x90a\x0E\xBCWP`-`\xF9\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x11\x15[\x15[\x80\x15a\x0E\xF3WP`a`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x10\x80\x15\x90a\x0E\xF1WP`=`\xF9\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x11\x15[\x15[\x80\x15a\x0F\rWP`\x17`\xF9\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x14\x15[\x80\x15a\x0F'WP`-`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x14\x15[\x80\x15a\x0FAWP`_`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x14\x15[\x15a\x0F_W`@Qc\xB9\xDBqs`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80a\x0Fj\x81a\x18\x9AV[\x91PPa\rZV[P`\0a\x0F~\x83a\x18\xE4V[`\0\x81\x81R`\x01` R`@\x90 T\x90\x91P\x15a\x0F\xAEW`@Qcix=\xB7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x95\x94PPPPPV[`\0\x87\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x90U\x88\x83R`\x01\x90\x91R\x80\x82 \x82\x90U\x84\x82R\x90\x81\x90 \x88\x90UQ\x7F+j\xBB\xFF\x11\x1A\x96\xD9\xFE\xDB\x942\x99%\xDCM\x1D\x80\x9C\x18UNK\xCB\xCC\x1E\x80\xCE\xECe0q\x90a\x10\x19\x90\x89\x90\x88\x90\x88\x90\x87\x90\x87\x90a\x194V[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a\x11\tW`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x90\x91[\x81\x81\x10\x15a\x11\x05W\x83\x83\x82\x81T\x81\x10a\x10qWa\x10qa\x18nV[\x90`\0R` `\0 \x01T\x03a\x10\xF3W\x82a\x10\x8D`\x01\x84a\x18\xD1V[\x81T\x81\x10a\x10\x9DWa\x10\x9Da\x18nV[\x90`\0R` `\0 \x01T\x83\x82\x81T\x81\x10a\x10\xBAWa\x10\xBAa\x18nV[\x90`\0R` `\0 \x01\x81\x90UP\x82\x80T\x80a\x10\xD8Wa\x10\xD8a\x19mV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90Ua\x11\x05V[\x80a\x10\xFD\x81a\x18\x9AV[\x91PPa\x10VV[PPP[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x07\x99W`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\x03` \x90\x81R`@\x82 \x80T`\x01\x81\x01\x82U\x90\x83R\x91 \x01UPV[`\0\x80T\x81\x80a\x11U\x83a\x18\x9AV[\x91\x90PU\x90Pa\x11e\x85\x82a\x12bV[`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x87\x90U\x86\x83R`\x01\x90\x91R\x90\x81\x90 \x82\x90UQ\x7F.$\x89\xADc[^Z2\xF1K\xC4\xE7\xE1u\xAE\x9E\xAF\xB8\x1A\x9Ci\x1F\xB0\x1B\x18\xA8\xB3*\xFA\xC0I\x90a\x11\xBC\x90\x83\x90\x88\x90\x87\x90\x87\x90a\x19\x83V[`@Q\x80\x91\x03\x90\xA1PPPPPV[a\x0C\xE8`\0\x82a\x12|V[`@Qc\x15\x0Bz\x02\x80\x82R3` \x83\x01R\x85``\x1B``\x1C`@\x83\x01R\x83``\x83\x01R`\x80\x80\x83\x01R\x82Q\x80`\xA0\x84\x01R\x80\x15a\x12\x1DW\x80`\xC0\x84\x01\x82` \x87\x01`\x04Z\xFAP[` \x83`\xA4\x83\x01`\x1C\x86\x01`\0\x8AZ\xF1a\x12@W=\x15a\x12@W=`\0\x84>=\x83\xFD[P\x80`\xE0\x1B\x82Q\x14a\x12ZWc\xD1\xA5~\xD6`\0R`\x04`\x1C\xFD[PPPPPPV[a\x05\x0B\x82\x82`@Q\x80` \x01`@R\x80`\0\x81RPa\x13VV[`\0a\x12\x87\x82a\t\rV[\x90Pa\x12\x95\x81`\0\x84a\x10*V[P`\0\x81\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16g>\xC4\x12\xA9\x85-\x17=`\xC1\x1B\x81\x17`\x1CR` \x90\x91 \x82\x01\x82\x01\x80T\x91\x93\x82\x16\x91\x82a\x12\xDAWc\xCE\xEA!\xB6`\0R`\x04`\x1C\xFD[\x82`\0R\x81`\x01\x01T\x80\x86\x14\x84\x87\x14\x17\x86\x15\x17a\x13\tW`0`\x0C Ta\x13\tWcKn\x7F\x18`\0R`\x04`\x1C\xFD[\x80\x15a\x13\x17W`\0\x83`\x01\x01U[P\x82\x18\x90U`\x1C`\x0C \x80T`\0\x19\x01\x90U\x81`\0\x82\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x828\xA4PPPV[a\x13`\x83\x83a\x13tV[\x82;\x15a\x07\x99Wa\x07\x99`\0\x84\x84\x84a\x11\xD6V[a\x13\x80`\0\x83\x83a\x10*V[\x81``\x1B``\x1C\x91P\x80`\0Rg>\xC4\x12\xA9\x85-\x17=`\xC1\x1B`\x1CR` `\0 \x81\x01\x81\x01\x80T\x80``\x1B\x15a\x13\xBEWc\xC9\x91\xCB\xB1`\0R`\x04`\x1C\xFD[\x83\x17\x90U`\0\x82\x90R`\x1C`\x0C \x80T`\x01\x01c\xFF\xFF\xFF\xFF\x81\x16\x84\x02a\x13\xF3Wg\xEAU;4\x013l\xEA\x84\x15`\x02\x1BR`\x04`\x1C\xFD[\x90U\x80\x82`\0\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x818\xA4PPV[`\0` \x82\x84\x03\x12\x15a\x143W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0C\xAEW`\0\x80\xFD[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x14qW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x14UV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\x14\xA4` \x83\x01\x84a\x14KV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x14\xBDW`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\tFW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x14\xEEW`\0\x80\xFD[a\x14\xF7\x83a\x14\xC4V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\x15\x17W`\0\x80\xFD[a\x14\xA4\x82a\x14\xC4V[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a\x15uW`?\x19\x88\x86\x03\x01\x84Ra\x15c\x85\x83Qa\x14KV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x15GV[P\x92\x97\x96PPPPPPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x15\x94W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\xACW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x15\xC4W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\x15\xE1W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x15\xF9W`\0\x80\xFD[a\x16\x05\x88\x83\x89\x01a\x15\x82V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\x16\x1EW`\0\x80\xFD[Pa\x16+\x87\x82\x88\x01a\x15\x82V[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x16LW`\0\x80\xFD[a\x16U\x84a\x14\xC4V[\x92Pa\x16c` \x85\x01a\x14\xC4V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x16\x88W`\0\x80\xFD[a\x16\x91\x84a\x14\xC4V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\xADW`\0\x80\xFD[a\x16\xB9\x86\x82\x87\x01a\x15\x82V[\x94\x97\x90\x96P\x93\x94PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x16\xFEW\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x16\xE2V[P\x90\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x17\x1DW`\0\x80\xFD[a\x17&\x83a\x14\xC4V[\x91P` \x83\x015\x80\x15\x15\x81\x14a\x17;W`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x17^W`\0\x80\xFD[a\x17g\x86a\x14\xC4V[\x94Pa\x17u` \x87\x01a\x14\xC4V[\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\x98W`\0\x80\xFD[a\x17\xA4\x88\x82\x89\x01a\x15\x82V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x17\xCDW`\0\x80\xFD[a\x17\xD6\x86a\x14\xC4V[\x94P` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x17\xF3W`\0\x80\xFD[a\x17\xFF\x89\x83\x8A\x01a\x15\x82V[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15a\x18\x18W`\0\x80\xFD[Pa\x17\xA4\x88\x82\x89\x01a\x15\x82V[`\0\x80`@\x83\x85\x03\x12\x15a\x188W`\0\x80\xFD[a\x18A\x83a\x14\xC4V[\x91Pa\x18O` \x84\x01a\x14\xC4V[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x18\xACWa\x18\xACa\x18\x84V[P`\x01\x01\x90V[\x805` \x83\x10\x15a\x0C\xB1W`\0\x19` \x84\x90\x03`\x03\x1B\x1B\x16\x92\x91PPV[\x81\x81\x03\x81\x81\x11\x15a\x0C\xB1Wa\x0C\xB1a\x18\x84V[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x19\x05W`\0\x19\x81` \x03`\x03\x1B\x1B\x82\x16\x91P[P\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x85\x81R``` \x82\x01R`\0a\x19N``\x83\x01\x86\x88a\x19\x0BV[\x82\x81\x03`@\x84\x01Ra\x19a\x81\x85\x87a\x19\x0BV[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[\x84\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16` \x82\x01R```@\x82\x01\x81\x90R`\0\x90a\x19\xAE\x90\x83\x01\x84\x86a\x19\x0BV[\x96\x95PPPPPPV\xFE\xA2dipfsX\"\x12 \xB7\xEF\x16\xCC\xA7\xB3\xD2\x05\xA9Xq\xB1\xFE0\xB4V\x8A\xDA\xBA7`\xF2\x1D\xC7\xAA\xF1^\xEA\xA9z\x01\x95dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static BASEMAILACCOUNT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01@W`\x005`\xE0\x1C\x80cB\x84.\x0E\x11a\0\xB6W\x80c\xA2,\xB4e\x11a\0oW\x80c\xA2,\xB4e\x14a\x03\xF4W\x80c\xB8\x8DO\xDE\x14a\x04\x14W\x80c\xC8{V\xDD\x14a\x04'W\x80c\xE0\x1E\xCFK\x14a\x04UW\x80c\xE9\x85\xE9\xC5\x14a\x04uW\x80c\xEB\x08\xAB(\x14a\x04\xABW`\0\x80\xFD[\x80cB\x84.\x0E\x14a\x03&W\x80ccR!\x1E\x14a\x039W\x80ce\xA9\xAF\x16\x14a\x03YW\x80cl\x86\xA40\x14a\x03yW\x80cp\xA0\x821\x14a\x03\xA6W\x80c\x95\xD8\x9BA\x14a\x03\xC6W`\0\x80\xFD[\x80c\x15*\x1C\x04\x11a\x01\x08W\x80c\x15*\x1C\x04\x14a\x02KW\x80c#\xB8r\xDD\x14a\x02kW\x80c&Y\xF3\x94\x14a\x02~W\x80c/\xB1'\x1D\x14a\x02\x9EW\x80c1\0\xCE\xB5\x14a\x02\xBEW\x80c6D\x10\xB3\x14a\x02\xEBW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01EW\x80c\x06\xFD\xDE\x03\x14a\x01\x97W\x80c\x08\x18\x12\xFC\x14a\x01\xD1W\x80c\t^\xA7\xB3\x14a\x02\tW\x80c\x13\x8A\xAE<\x14a\x02\x1EW[`\0\x80\xFD[4\x80\x15a\x01QW`\0\x80\xFD[Pa\x01\x82a\x01`6`\x04a\x14!V[c\x01\xFF\xC9\xA7`\xE0\x91\x90\x91\x1C\x90\x81\x14c\x80\xACX\xCD\x82\x14\x17c[^\x13\x9F\x90\x91\x14\x17\x90V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xA3W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x08\x81Rg\x10\x98\\\xD9[XZ[`\xC2\x1B` \x82\x01R[`@Qa\x01\x8E\x91\x90a\x14\x91V[4\x80\x15a\x01\xDDW`\0\x80\xFD[Pa\x01\xF1a\x01\xEC6`\x04a\x14\xABV[a\x04\xC1V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x8EV[a\x02\x1Ca\x02\x176`\x04a\x14\xDBV[a\x05\0V[\0[4\x80\x15a\x02*W`\0\x80\xFD[Pa\x02>a\x0296`\x04a\x15\x05V[a\x05\x0FV[`@Qa\x01\x8E\x91\x90a\x15 V[4\x80\x15a\x02WW`\0\x80\xFD[Pa\x02\x1Ca\x02f6`\x04a\x15\xCBV[a\x06EV[a\x02\x1Ca\x02y6`\x04a\x167V[a\x06\x92V[4\x80\x15a\x02\x8AW`\0\x80\xFD[Pa\x02\x1Ca\x02\x996`\x04a\x16sV[a\x07\x9EV[4\x80\x15a\x02\xAAW`\0\x80\xFD[Pa\x02\x1Ca\x02\xB96`\x04a\x14\xABV[a\x07\xBEV[4\x80\x15a\x02\xCAW`\0\x80\xFD[Pa\x02\xDEa\x02\xD96`\x04a\x15\x05V[a\x08tV[`@Qa\x01\x8E\x91\x90a\x16\xC6V[4\x80\x15a\x02\xF7W`\0\x80\xFD[Pa\x03\x18a\x03\x066`\x04a\x14\xABV[`\x02` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\x01\x8EV[a\x02\x1Ca\x0346`\x04a\x167V[a\x08\xE0V[4\x80\x15a\x03EW`\0\x80\xFD[Pa\x01\xF1a\x03T6`\x04a\x14\xABV[a\t\rV[4\x80\x15a\x03eW`\0\x80\xFD[Pa\x03\x18a\x03t6`\x04a\x14\xDBV[a\tKV[4\x80\x15a\x03\x85W`\0\x80\xFD[Pa\x03\x18a\x03\x946`\x04a\x14\xABV[`\x01` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x03\xB2W`\0\x80\xFD[Pa\x03\x18a\x03\xC16`\x04a\x15\x05V[a\t|V[4\x80\x15a\x03\xD2W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81Rd\x10\x93PRS`\xDA\x1B` \x82\x01Ra\x01\xC4V[4\x80\x15a\x04\0W`\0\x80\xFD[Pa\x02\x1Ca\x04\x0F6`\x04a\x17\nV[a\t\xB7V[a\x02\x1Ca\x04\"6`\x04a\x17FV[a\n\rV[4\x80\x15a\x043W`\0\x80\xFD[Pa\x01\xC4a\x04B6`\x04a\x14\xABV[P`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[4\x80\x15a\x04aW`\0\x80\xFD[Pa\x02\x1Ca\x04p6`\x04a\x17\xB5V[a\nhV[4\x80\x15a\x04\x81W`\0\x80\xFD[Pa\x01\x82a\x04\x906`\x04a\x18%V[`\x1CRg\nZ.z\0\0\0\0`\x08R`\0R`0`\x0C T\x90V[4\x80\x15a\x04\xB7W`\0\x80\xFD[Pa\x03\x18`\0T\x81V[`\0\x81`\0Rg>\xC4\x12\xA9\x85-\x17=`\xC1\x1B`\x1CR` `\0 \x82\x01\x82\x01\x80T``\x1Ba\x04\xF6Wc\xCE\xEA!\xB6`\0R`\x04`\x1C\xFD[`\x01\x01T\x92\x91PPV[a\x05\x0B3\x83\x83a\n\xC2V[PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94\x93\x83\x01\x82\x82\x80\x15a\x05nW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x05ZW[PPPPP\x90P`\0\x81Q\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\x96Wa\x05\x96a\x18XV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xC9W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x05\xB4W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x06<Wa\x06\x0C`\x02`\0\x86\x84\x81Q\x81\x10a\x05\xF0Wa\x05\xF0a\x18nV[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 Ta\x0BcV[\x82\x82\x81Q\x81\x10a\x06\x1EWa\x06\x1Ea\x18nV[` \x02` \x01\x01\x81\x90RP\x80\x80a\x064\x90a\x18\x9AV[\x91PPa\x05\xCFV[P\x94\x93PPPPV[`\0a\x06Q\x85\x85a\x0CqV[`\0\x81\x81R`\x01` R`@\x90 T\x90\x91Pa\x06l\x81a\x0C\xB7V[`\0a\x06x\x85\x85a\x0C\xEBV[\x90Pa\x06\x89\x82\x84\x89\x89\x85\x8A\x8Aa\x0F\xB7V[PPPPPPPV[a\x06\x9D\x83\x83\x83a\x10*V[`\0\x81\x81Rg>\xC4\x12\xA9\x85-\x17=`\xC1\x1B3\x17`\x1CR` \x90 \x81\x01\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x94\x93\x84\x16\x93\x81\x16\x91\x90\x82\x86\x14\x83\x02a\x06\xEFWg\xCE\xEA!\xB6\xA1\x14\x81\0\x83\x15`\x02\x1BR`\x04`\x1C\xFD[\x85`\0R\x81`\x01\x01T\x92P\x823\x14\x863\x14\x17a\x07\x1DW`0`\x0C Ta\x07\x1DWcKn\x7F\x18`\0R`\x04`\x1C\xFD[\x82\x15a\x07+W`\0\x82`\x01\x01U[\x85\x85\x18\x18\x90UP`\x1C`\x0C\x81\x81 \x80T`\0\x19\x01\x90U`\0\x84\x90R \x80T`\x01\x01c\xFF\xFF\xFF\xFF\x81\x16\x84\x02a\x07nWg\xEAU;4\x013l\xEA\x84\x15`\x02\x1BR`\x04`\x1C\xFD[\x90U\x80\x82\x84\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF`\08\xA4[PPPV[`\0a\x07\xAA\x83\x83a\x0C\xEBV[\x90Pa\x07\xB8\x84\x82\x85\x85a\x11FV[PPPPV[`\0T\x81\x10\x15\x80a\x07\xECWP`\0\x81\x81Rg>\xC4\x12\xA9\x85-\x17=`\xC1\x1B`\x1CR` \x90 \x81\x01\x81\x01T``\x1B\x15[\x15a\x08\nW`@Qc\xE7n\xA8\x7F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\x13\x81a\x0C\xB7V[a\x08\x1C\x81a\x11\xCBV[`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x80T\x90\x84\x90U\x80\x84R`\x01\x83R\x81\x84 \x93\x90\x93UQ\x83\x81R\x7F\xD7\xCDn\xF0\xA0\x98X]\r\xEE\xEB\x14\xD8i0F{\xEC\xD4_\x18\xBA#\xDA\xB2i\x11\x9B\xF9\xD6\xCA\xB9\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x03` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R``\x93\x92\x83\x01\x82\x82\x80\x15a\x08\xD4W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x08\xC0W[PPPPP\x90P\x91\x90PV[a\x08\xEB\x83\x83\x83a\x06\x92V[\x81;\x15a\x07\x99Wa\x07\x99\x83\x83\x83`@Q\x80` \x01`@R\x80`\0\x81RPa\x11\xD6V[`\0\x81\x81Rg>\xC4\x12\xA9\x85-\x17=`\xC1\x1B`\x1CR` \x90 \x81\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16\x80a\tFWc\xCE\xEA!\xB6`\0R`\x04`\x1C\xFD[\x91\x90PV[`\x03` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\tgW`\0\x80\xFD[\x90`\0R` `\0 \x01`\0\x91P\x91PPT\x81V[`\0\x81a\t\x91Wc\x8FN\xB6\x04`\0R`\x04`\x1C\xFD[g>\xC4\x12\xA9\x85-\x17=`\xC1\x1B`\x1CR\x81`\0Rc\xFF\xFF\xFF\xFF`\x1C`\x0C T\x16\x90P\x91\x90PV[\x80\x15\x15\x90P\x81`\x1CRg\nZ.z\0\0\0\0`\x08R3`\0R\x80`0`\x0C U\x80`\0R\x81``\x1B``\x1C3\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1` `\0\xA3PPV[a\n\x18\x85\x85\x85a\x06\x92V[\x83;\x15a\naWa\na\x85\x85\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x11\xD6\x92PPPV[PPPPPV[`\0a\nt\x85\x85a\x0CqV[`\0\x81\x81R`\x01` R`@\x90 T\x90\x91Pa\n\x8F\x81a\x0C\xB7V[`\0a\n\x9B\x85\x85a\x0C\xEBV[\x90Pa\n\xAC\x82\x84\x89\x89\x85\x8A\x8Aa\x0F\xB7V[a\n\xB8\x88\x84\x89\x89a\x11FV[PPPPPPPPV[`\0\x19``\x1C\x82\x81\x16\x92P\x83\x81\x16\x93P\x81`\0R\x83g>\xC4\x12\xA9\x85-\x17=`\xC1\x1B\x17`\x1CR` `\0 \x82\x01\x82\x01\x80T\x82\x16\x91P\x81a\x0B\tWc\xCE\xEA!\xB6`\0R`\x04`\x1C\xFD[\x81\x85\x14\x85\x15\x17a\x0B/W\x81`\0R`0`\x0C Ta\x0B/WcKn\x7F\x18`\0R`\x04`\x1C\xFD[`\x01\x01\x83\x90U\x81\x83\x82\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%`\08\xA4PPPPV[```\0\x82`@Q` \x01a\x0Bz\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0[\x81\x81\x81Q\x81\x10a\x0B\xA0Wa\x0B\xA0a\x18nV[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x15a\x0B\xC6W\x80a\x0B\xBE\x81a\x18\x9AV[\x91PPa\x0B\x8EV[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xE1Wa\x0B\xE1a\x18XV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0C\x0BW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x06<W\x83\x81\x81Q\x81\x10a\x0C+Wa\x0C+a\x18nV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x0CHWa\x0CHa\x18nV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x0Ci\x81a\x18\x9AV[\x91PPa\x0C\x11V[`\0\x80a\x0C~\x83\x85a\x18\xB3V[`\0\x81\x81R`\x01` R`@\x81 T\x91\x92P\x03a\x0C\xAEW`@Qc\xE7n\xA8\x7F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x90P[\x92\x91PPV[3a\x0C\xC1\x82a\t\rV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0C\xE8W`@Qc\x0B\x98O\x17`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`\0\x80\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP\x82Q\x92\x93PP` \x82\x11\x90P\x80a\r9WP`\x03\x81\x10[\x15a\rWW`@Qc\xB9\xDBqs`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x81\x81\x10\x15a\x0FrW`\0\x83\x82\x81Q\x81\x10a\rvWa\rva\x18nV[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90P\x81\x15\x80a\r\x9DWPa\r\x9A`\x01\x84a\x18\xD1V[\x82\x14[\x15a\x0E\\W`\x03`\xFC\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x10\x80\x15\x90a\r\xCEWP`9`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x11\x15[\x15\x80\x15a\x0E\x04WP`A`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x10\x80\x15\x90a\x0E\x02WP`-`\xF9\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x11\x15[\x15[\x80\x15a\x0E9WP`a`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x10\x80\x15\x90a\x0E7WP`=`\xF9\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x11\x15[\x15[\x15a\x0EWW`@Qc\xB9\xDBqs`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F_V[`\x03`\xFC\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x10\x80\x15\x90a\x0E\x88WP`9`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x11\x15[\x15\x80\x15a\x0E\xBEWP`A`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x10\x80\x15\x90a\x0E\xBCWP`-`\xF9\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x11\x15[\x15[\x80\x15a\x0E\xF3WP`a`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x10\x80\x15\x90a\x0E\xF1WP`=`\xF9\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x11\x15[\x15[\x80\x15a\x0F\rWP`\x17`\xF9\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x14\x15[\x80\x15a\x0F'WP`-`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x14\x15[\x80\x15a\x0FAWP`_`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x82\x16\x14\x15[\x15a\x0F_W`@Qc\xB9\xDBqs`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80a\x0Fj\x81a\x18\x9AV[\x91PPa\rZV[P`\0a\x0F~\x83a\x18\xE4V[`\0\x81\x81R`\x01` R`@\x90 T\x90\x91P\x15a\x0F\xAEW`@Qcix=\xB7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x95\x94PPPPPV[`\0\x87\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x90U\x88\x83R`\x01\x90\x91R\x80\x82 \x82\x90U\x84\x82R\x90\x81\x90 \x88\x90UQ\x7F+j\xBB\xFF\x11\x1A\x96\xD9\xFE\xDB\x942\x99%\xDCM\x1D\x80\x9C\x18UNK\xCB\xCC\x1E\x80\xCE\xECe0q\x90a\x10\x19\x90\x89\x90\x88\x90\x88\x90\x87\x90\x87\x90a\x194V[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a\x11\tW`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x90\x91[\x81\x81\x10\x15a\x11\x05W\x83\x83\x82\x81T\x81\x10a\x10qWa\x10qa\x18nV[\x90`\0R` `\0 \x01T\x03a\x10\xF3W\x82a\x10\x8D`\x01\x84a\x18\xD1V[\x81T\x81\x10a\x10\x9DWa\x10\x9Da\x18nV[\x90`\0R` `\0 \x01T\x83\x82\x81T\x81\x10a\x10\xBAWa\x10\xBAa\x18nV[\x90`\0R` `\0 \x01\x81\x90UP\x82\x80T\x80a\x10\xD8Wa\x10\xD8a\x19mV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90Ua\x11\x05V[\x80a\x10\xFD\x81a\x18\x9AV[\x91PPa\x10VV[PPP[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x07\x99W`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\x03` \x90\x81R`@\x82 \x80T`\x01\x81\x01\x82U\x90\x83R\x91 \x01UPV[`\0\x80T\x81\x80a\x11U\x83a\x18\x9AV[\x91\x90PU\x90Pa\x11e\x85\x82a\x12bV[`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x87\x90U\x86\x83R`\x01\x90\x91R\x90\x81\x90 \x82\x90UQ\x7F.$\x89\xADc[^Z2\xF1K\xC4\xE7\xE1u\xAE\x9E\xAF\xB8\x1A\x9Ci\x1F\xB0\x1B\x18\xA8\xB3*\xFA\xC0I\x90a\x11\xBC\x90\x83\x90\x88\x90\x87\x90\x87\x90a\x19\x83V[`@Q\x80\x91\x03\x90\xA1PPPPPV[a\x0C\xE8`\0\x82a\x12|V[`@Qc\x15\x0Bz\x02\x80\x82R3` \x83\x01R\x85``\x1B``\x1C`@\x83\x01R\x83``\x83\x01R`\x80\x80\x83\x01R\x82Q\x80`\xA0\x84\x01R\x80\x15a\x12\x1DW\x80`\xC0\x84\x01\x82` \x87\x01`\x04Z\xFAP[` \x83`\xA4\x83\x01`\x1C\x86\x01`\0\x8AZ\xF1a\x12@W=\x15a\x12@W=`\0\x84>=\x83\xFD[P\x80`\xE0\x1B\x82Q\x14a\x12ZWc\xD1\xA5~\xD6`\0R`\x04`\x1C\xFD[PPPPPPV[a\x05\x0B\x82\x82`@Q\x80` \x01`@R\x80`\0\x81RPa\x13VV[`\0a\x12\x87\x82a\t\rV[\x90Pa\x12\x95\x81`\0\x84a\x10*V[P`\0\x81\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16g>\xC4\x12\xA9\x85-\x17=`\xC1\x1B\x81\x17`\x1CR` \x90\x91 \x82\x01\x82\x01\x80T\x91\x93\x82\x16\x91\x82a\x12\xDAWc\xCE\xEA!\xB6`\0R`\x04`\x1C\xFD[\x82`\0R\x81`\x01\x01T\x80\x86\x14\x84\x87\x14\x17\x86\x15\x17a\x13\tW`0`\x0C Ta\x13\tWcKn\x7F\x18`\0R`\x04`\x1C\xFD[\x80\x15a\x13\x17W`\0\x83`\x01\x01U[P\x82\x18\x90U`\x1C`\x0C \x80T`\0\x19\x01\x90U\x81`\0\x82\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x828\xA4PPPV[a\x13`\x83\x83a\x13tV[\x82;\x15a\x07\x99Wa\x07\x99`\0\x84\x84\x84a\x11\xD6V[a\x13\x80`\0\x83\x83a\x10*V[\x81``\x1B``\x1C\x91P\x80`\0Rg>\xC4\x12\xA9\x85-\x17=`\xC1\x1B`\x1CR` `\0 \x81\x01\x81\x01\x80T\x80``\x1B\x15a\x13\xBEWc\xC9\x91\xCB\xB1`\0R`\x04`\x1C\xFD[\x83\x17\x90U`\0\x82\x90R`\x1C`\x0C \x80T`\x01\x01c\xFF\xFF\xFF\xFF\x81\x16\x84\x02a\x13\xF3Wg\xEAU;4\x013l\xEA\x84\x15`\x02\x1BR`\x04`\x1C\xFD[\x90U\x80\x82`\0\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x818\xA4PPV[`\0` \x82\x84\x03\x12\x15a\x143W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0C\xAEW`\0\x80\xFD[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x14qW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x14UV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\x14\xA4` \x83\x01\x84a\x14KV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x14\xBDW`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\tFW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x14\xEEW`\0\x80\xFD[a\x14\xF7\x83a\x14\xC4V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\x15\x17W`\0\x80\xFD[a\x14\xA4\x82a\x14\xC4V[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a\x15uW`?\x19\x88\x86\x03\x01\x84Ra\x15c\x85\x83Qa\x14KV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x15GV[P\x92\x97\x96PPPPPPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x15\x94W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\xACW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x15\xC4W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\x15\xE1W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x15\xF9W`\0\x80\xFD[a\x16\x05\x88\x83\x89\x01a\x15\x82V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\x16\x1EW`\0\x80\xFD[Pa\x16+\x87\x82\x88\x01a\x15\x82V[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x16LW`\0\x80\xFD[a\x16U\x84a\x14\xC4V[\x92Pa\x16c` \x85\x01a\x14\xC4V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x16\x88W`\0\x80\xFD[a\x16\x91\x84a\x14\xC4V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\xADW`\0\x80\xFD[a\x16\xB9\x86\x82\x87\x01a\x15\x82V[\x94\x97\x90\x96P\x93\x94PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x16\xFEW\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x16\xE2V[P\x90\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x17\x1DW`\0\x80\xFD[a\x17&\x83a\x14\xC4V[\x91P` \x83\x015\x80\x15\x15\x81\x14a\x17;W`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x17^W`\0\x80\xFD[a\x17g\x86a\x14\xC4V[\x94Pa\x17u` \x87\x01a\x14\xC4V[\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\x98W`\0\x80\xFD[a\x17\xA4\x88\x82\x89\x01a\x15\x82V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x17\xCDW`\0\x80\xFD[a\x17\xD6\x86a\x14\xC4V[\x94P` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x17\xF3W`\0\x80\xFD[a\x17\xFF\x89\x83\x8A\x01a\x15\x82V[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15a\x18\x18W`\0\x80\xFD[Pa\x17\xA4\x88\x82\x89\x01a\x15\x82V[`\0\x80`@\x83\x85\x03\x12\x15a\x188W`\0\x80\xFD[a\x18A\x83a\x14\xC4V[\x91Pa\x18O` \x84\x01a\x14\xC4V[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x18\xACWa\x18\xACa\x18\x84V[P`\x01\x01\x90V[\x805` \x83\x10\x15a\x0C\xB1W`\0\x19` \x84\x90\x03`\x03\x1B\x1B\x16\x92\x91PPV[\x81\x81\x03\x81\x81\x11\x15a\x0C\xB1Wa\x0C\xB1a\x18\x84V[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x19\x05W`\0\x19\x81` \x03`\x03\x1B\x1B\x82\x16\x91P[P\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x85\x81R``` \x82\x01R`\0a\x19N``\x83\x01\x86\x88a\x19\x0BV[\x82\x81\x03`@\x84\x01Ra\x19a\x81\x85\x87a\x19\x0BV[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[\x84\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16` \x82\x01R```@\x82\x01\x81\x90R`\0\x90a\x19\xAE\x90\x83\x01\x84\x86a\x19\x0BV[\x96\x95PPPPPPV\xFE\xA2dipfsX\"\x12 \xB7\xEF\x16\xCC\xA7\xB3\xD2\x05\xA9Xq\xB1\xFE0\xB4V\x8A\xDA\xBA7`\xF2\x1D\xC7\xAA\xF1^\xEA\xA9z\x01\x95dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static BASEMAILACCOUNT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct BasemailAccount<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BasemailAccount<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BasemailAccount<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BasemailAccount<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BasemailAccount<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BasemailAccount))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BasemailAccount<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BASEMAILACCOUNT_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                BASEMAILACCOUNT_ABI.clone(),
                BASEMAILACCOUNT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            account: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 94, 167, 179], (account, id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeUsername` (0x152a1c04) function
        pub fn change_username(
            &self,
            old_username: ::std::string::String,
            new_username: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 42, 28, 4], (old_username, new_username))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createAccount` (0x2659f394) function
        pub fn create_account(
            &self,
            to: ::ethers::core::types::Address,
            username: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([38, 89, 243, 148], (to, username))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deleteAccount` (0x2fb1271d) function
        pub fn delete_account(
            &self,
            account_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 177, 39, 29], account_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAccounts` (0x3100ceb5) function
        pub fn get_accounts(
            &self,
            holder: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([49, 0, 206, 181], holder)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getApproved` (0x081812fc) function
        pub fn get_approved(
            &self,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([8, 24, 18, 252], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUsernames` (0x138aae3c) function
        pub fn get_usernames(
            &self,
            holder: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::string::String>,
        > {
            self.0
                .method_hash([19, 138, 174, 60], holder)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `holderAccounts` (0x65a9af16) function
        pub fn holder_accounts(
            &self,
            holder: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([101, 169, 175, 22], (holder, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `idCounter` (0xeb08ab28) function
        pub fn id_counter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([235, 8, 171, 40], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `idToName` (0x364410b3) function
        pub fn id_to_name(
            &self,
            account_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 16, 179], account_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isApprovedForAll` (0xe985e9c5) function
        pub fn is_approved_for_all(
            &self,
            owner: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (owner, operator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nameToId` (0x6c86a430) function
        pub fn name_to_id(
            &self,
            username: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([108, 134, 164, 48], username)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ownerOf` (0x6352211e) function
        pub fn owner_of(
            &self,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([99, 82, 33, 30], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom` (0x42842e0e) function
        pub fn safe_transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 132, 46, 14], (from, to, id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom` (0xb88d4fde) function
        pub fn safe_transfer_from_with_from_and_to_and_data(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 141, 79, 222], (from, to, id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setApprovalForAll` (0xa22cb465) function
        pub fn set_approval_for_all(
            &self,
            operator: ::ethers::core::types::Address,
            is_approved: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, is_approved))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenURI` (0xc87b56dd) function
        pub fn token_uri(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([200, 123, 86, 221], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferUsername` (0xe01ecf4b) function
        pub fn transfer_username(
            &self,
            to: ::ethers::core::types::Address,
            username_to_transfer: ::std::string::String,
            new_username: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [224, 30, 207, 75],
                    (to, username_to_transfer, new_username),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AccountCreated` event
        pub fn account_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AccountCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AccountDeleted` event
        pub fn account_deleted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AccountDeletedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ApprovalForAll` event
        pub fn approval_for_all_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalForAllFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `UsernameChanged` event
        pub fn username_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UsernameChangedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BasemailAccountEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for BasemailAccount<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AccountAlreadyExists` with signature `AccountAlreadyExists()` and selector `0x69783db7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "AccountAlreadyExists", abi = "AccountAlreadyExists()")]
    pub struct AccountAlreadyExists;
    ///Custom Error type `AccountBalanceOverflow` with signature `AccountBalanceOverflow()` and selector `0x01336cea`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "AccountBalanceOverflow", abi = "AccountBalanceOverflow()")]
    pub struct AccountBalanceOverflow;
    ///Custom Error type `AccountDoesNotExist` with signature `AccountDoesNotExist()` and selector `0xe76ea87f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "AccountDoesNotExist", abi = "AccountDoesNotExist()")]
    pub struct AccountDoesNotExist;
    ///Custom Error type `BalanceQueryForZeroAddress` with signature `BalanceQueryForZeroAddress()` and selector `0x8f4eb604`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "BalanceQueryForZeroAddress",
        abi = "BalanceQueryForZeroAddress()"
    )]
    pub struct BalanceQueryForZeroAddress;
    ///Custom Error type `NotOwnerNorApproved` with signature `NotOwnerNorApproved()` and selector `0x4b6e7f18`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NotOwnerNorApproved", abi = "NotOwnerNorApproved()")]
    pub struct NotOwnerNorApproved;
    ///Custom Error type `OnlyTokenHolder` with signature `OnlyTokenHolder()` and selector `0x2e613c5c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OnlyTokenHolder", abi = "OnlyTokenHolder()")]
    pub struct OnlyTokenHolder;
    ///Custom Error type `TokenAlreadyExists` with signature `TokenAlreadyExists()` and selector `0xc991cbb1`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "TokenAlreadyExists", abi = "TokenAlreadyExists()")]
    pub struct TokenAlreadyExists;
    ///Custom Error type `TokenDoesNotExist` with signature `TokenDoesNotExist()` and selector `0xceea21b6`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "TokenDoesNotExist", abi = "TokenDoesNotExist()")]
    pub struct TokenDoesNotExist;
    ///Custom Error type `TransferFromIncorrectOwner` with signature `TransferFromIncorrectOwner()` and selector `0xa1148100`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "TransferFromIncorrectOwner",
        abi = "TransferFromIncorrectOwner()"
    )]
    pub struct TransferFromIncorrectOwner;
    ///Custom Error type `TransferToNonERC721ReceiverImplementer` with signature `TransferToNonERC721ReceiverImplementer()` and selector `0xd1a57ed6`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "TransferToNonERC721ReceiverImplementer",
        abi = "TransferToNonERC721ReceiverImplementer()"
    )]
    pub struct TransferToNonERC721ReceiverImplementer;
    ///Custom Error type `TransferToZeroAddress` with signature `TransferToZeroAddress()` and selector `0xea553b34`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "TransferToZeroAddress", abi = "TransferToZeroAddress()")]
    pub struct TransferToZeroAddress;
    ///Custom Error type `UsernameInvalid` with signature `UsernameInvalid()` and selector `0xb9db7173`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "UsernameInvalid", abi = "UsernameInvalid()")]
    pub struct UsernameInvalid;
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum BasemailAccountErrors {
        AccountAlreadyExists(AccountAlreadyExists),
        AccountBalanceOverflow(AccountBalanceOverflow),
        AccountDoesNotExist(AccountDoesNotExist),
        BalanceQueryForZeroAddress(BalanceQueryForZeroAddress),
        NotOwnerNorApproved(NotOwnerNorApproved),
        OnlyTokenHolder(OnlyTokenHolder),
        TokenAlreadyExists(TokenAlreadyExists),
        TokenDoesNotExist(TokenDoesNotExist),
        TransferFromIncorrectOwner(TransferFromIncorrectOwner),
        TransferToNonERC721ReceiverImplementer(TransferToNonERC721ReceiverImplementer),
        TransferToZeroAddress(TransferToZeroAddress),
        UsernameInvalid(UsernameInvalid),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for BasemailAccountErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AccountAlreadyExists as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccountAlreadyExists(decoded));
            }
            if let Ok(decoded) = <AccountBalanceOverflow as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccountBalanceOverflow(decoded));
            }
            if let Ok(decoded) = <AccountDoesNotExist as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccountDoesNotExist(decoded));
            }
            if let Ok(decoded) = <BalanceQueryForZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceQueryForZeroAddress(decoded));
            }
            if let Ok(decoded) = <NotOwnerNorApproved as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotOwnerNorApproved(decoded));
            }
            if let Ok(decoded) = <OnlyTokenHolder as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnlyTokenHolder(decoded));
            }
            if let Ok(decoded) = <TokenAlreadyExists as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenAlreadyExists(decoded));
            }
            if let Ok(decoded) = <TokenDoesNotExist as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenDoesNotExist(decoded));
            }
            if let Ok(decoded) = <TransferFromIncorrectOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferFromIncorrectOwner(decoded));
            }
            if let Ok(decoded) = <TransferToNonERC721ReceiverImplementer as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferToNonERC721ReceiverImplementer(decoded));
            }
            if let Ok(decoded) = <TransferToZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferToZeroAddress(decoded));
            }
            if let Ok(decoded) = <UsernameInvalid as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UsernameInvalid(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BasemailAccountErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AccountAlreadyExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AccountBalanceOverflow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AccountDoesNotExist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceQueryForZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotOwnerNorApproved(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnlyTokenHolder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenAlreadyExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenDoesNotExist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFromIncorrectOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferToNonERC721ReceiverImplementer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferToZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UsernameInvalid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for BasemailAccountErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AccountAlreadyExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AccountBalanceOverflow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AccountDoesNotExist as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BalanceQueryForZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotOwnerNorApproved as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OnlyTokenHolder as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TokenAlreadyExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TokenDoesNotExist as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TransferFromIncorrectOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TransferToNonERC721ReceiverImplementer as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TransferToZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UsernameInvalid as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for BasemailAccountErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccountAlreadyExists(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccountBalanceOverflow(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccountDoesNotExist(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BalanceQueryForZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotOwnerNorApproved(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnlyTokenHolder(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenAlreadyExists(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenDoesNotExist(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFromIncorrectOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferToNonERC721ReceiverImplementer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferToZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UsernameInvalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for BasemailAccountErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AccountAlreadyExists> for BasemailAccountErrors {
        fn from(value: AccountAlreadyExists) -> Self {
            Self::AccountAlreadyExists(value)
        }
    }
    impl ::core::convert::From<AccountBalanceOverflow> for BasemailAccountErrors {
        fn from(value: AccountBalanceOverflow) -> Self {
            Self::AccountBalanceOverflow(value)
        }
    }
    impl ::core::convert::From<AccountDoesNotExist> for BasemailAccountErrors {
        fn from(value: AccountDoesNotExist) -> Self {
            Self::AccountDoesNotExist(value)
        }
    }
    impl ::core::convert::From<BalanceQueryForZeroAddress> for BasemailAccountErrors {
        fn from(value: BalanceQueryForZeroAddress) -> Self {
            Self::BalanceQueryForZeroAddress(value)
        }
    }
    impl ::core::convert::From<NotOwnerNorApproved> for BasemailAccountErrors {
        fn from(value: NotOwnerNorApproved) -> Self {
            Self::NotOwnerNorApproved(value)
        }
    }
    impl ::core::convert::From<OnlyTokenHolder> for BasemailAccountErrors {
        fn from(value: OnlyTokenHolder) -> Self {
            Self::OnlyTokenHolder(value)
        }
    }
    impl ::core::convert::From<TokenAlreadyExists> for BasemailAccountErrors {
        fn from(value: TokenAlreadyExists) -> Self {
            Self::TokenAlreadyExists(value)
        }
    }
    impl ::core::convert::From<TokenDoesNotExist> for BasemailAccountErrors {
        fn from(value: TokenDoesNotExist) -> Self {
            Self::TokenDoesNotExist(value)
        }
    }
    impl ::core::convert::From<TransferFromIncorrectOwner> for BasemailAccountErrors {
        fn from(value: TransferFromIncorrectOwner) -> Self {
            Self::TransferFromIncorrectOwner(value)
        }
    }
    impl ::core::convert::From<TransferToNonERC721ReceiverImplementer>
    for BasemailAccountErrors {
        fn from(value: TransferToNonERC721ReceiverImplementer) -> Self {
            Self::TransferToNonERC721ReceiverImplementer(value)
        }
    }
    impl ::core::convert::From<TransferToZeroAddress> for BasemailAccountErrors {
        fn from(value: TransferToZeroAddress) -> Self {
            Self::TransferToZeroAddress(value)
        }
    }
    impl ::core::convert::From<UsernameInvalid> for BasemailAccountErrors {
        fn from(value: UsernameInvalid) -> Self {
            Self::UsernameInvalid(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "AccountCreated", abi = "AccountCreated(uint256,address,string)")]
    pub struct AccountCreatedFilter {
        pub account_id: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub username: ::std::string::String,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "AccountDeleted", abi = "AccountDeleted(uint256)")]
    pub struct AccountDeletedFilter {
        pub account_id: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub id: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub is_approved: bool,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub id: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "UsernameChanged", abi = "UsernameChanged(uint256,string,string)")]
    pub struct UsernameChangedFilter {
        pub account_id: ::ethers::core::types::U256,
        pub old_username: ::std::string::String,
        pub new_username: ::std::string::String,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum BasemailAccountEvents {
        AccountCreatedFilter(AccountCreatedFilter),
        AccountDeletedFilter(AccountDeletedFilter),
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        TransferFilter(TransferFilter),
        UsernameChangedFilter(UsernameChangedFilter),
    }
    impl ::ethers::contract::EthLogDecode for BasemailAccountEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AccountCreatedFilter::decode_log(log) {
                return Ok(BasemailAccountEvents::AccountCreatedFilter(decoded));
            }
            if let Ok(decoded) = AccountDeletedFilter::decode_log(log) {
                return Ok(BasemailAccountEvents::AccountDeletedFilter(decoded));
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(BasemailAccountEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(BasemailAccountEvents::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(BasemailAccountEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = UsernameChangedFilter::decode_log(log) {
                return Ok(BasemailAccountEvents::UsernameChangedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for BasemailAccountEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccountCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccountDeletedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovalForAllFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UsernameChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AccountCreatedFilter> for BasemailAccountEvents {
        fn from(value: AccountCreatedFilter) -> Self {
            Self::AccountCreatedFilter(value)
        }
    }
    impl ::core::convert::From<AccountDeletedFilter> for BasemailAccountEvents {
        fn from(value: AccountDeletedFilter) -> Self {
            Self::AccountDeletedFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalFilter> for BasemailAccountEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalForAllFilter> for BasemailAccountEvents {
        fn from(value: ApprovalForAllFilter) -> Self {
            Self::ApprovalForAllFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for BasemailAccountEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    impl ::core::convert::From<UsernameChangedFilter> for BasemailAccountEvents {
        fn from(value: UsernameChangedFilter) -> Self {
            Self::UsernameChangedFilter(value)
        }
    }
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub account: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `changeUsername` function with signature `changeUsername(string,string)` and selector `0x152a1c04`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "changeUsername", abi = "changeUsername(string,string)")]
    pub struct ChangeUsernameCall {
        pub old_username: ::std::string::String,
        pub new_username: ::std::string::String,
    }
    ///Container type for all input parameters for the `createAccount` function with signature `createAccount(address,string)` and selector `0x2659f394`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "createAccount", abi = "createAccount(address,string)")]
    pub struct CreateAccountCall {
        pub to: ::ethers::core::types::Address,
        pub username: ::std::string::String,
    }
    ///Container type for all input parameters for the `deleteAccount` function with signature `deleteAccount(uint256)` and selector `0x2fb1271d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "deleteAccount", abi = "deleteAccount(uint256)")]
    pub struct DeleteAccountCall {
        pub account_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getAccounts` function with signature `getAccounts(address)` and selector `0x3100ceb5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getAccounts", abi = "getAccounts(address)")]
    pub struct GetAccountsCall {
        pub holder: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getApproved` function with signature `getApproved(uint256)` and selector `0x081812fc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getApproved", abi = "getApproved(uint256)")]
    pub struct GetApprovedCall {
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getUsernames` function with signature `getUsernames(address)` and selector `0x138aae3c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getUsernames", abi = "getUsernames(address)")]
    pub struct GetUsernamesCall {
        pub holder: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `holderAccounts` function with signature `holderAccounts(address,uint256)` and selector `0x65a9af16`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "holderAccounts", abi = "holderAccounts(address,uint256)")]
    pub struct HolderAccountsCall {
        pub holder: ::ethers::core::types::Address,
        pub p1: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `idCounter` function with signature `idCounter()` and selector `0xeb08ab28`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "idCounter", abi = "idCounter()")]
    pub struct IdCounterCall;
    ///Container type for all input parameters for the `idToName` function with signature `idToName(uint256)` and selector `0x364410b3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "idToName", abi = "idToName(uint256)")]
    pub struct IdToNameCall {
        pub account_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall {
        pub owner: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `nameToId` function with signature `nameToId(bytes32)` and selector `0x6c86a430`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "nameToId", abi = "nameToId(bytes32)")]
    pub struct NameToIdCall {
        pub username: [u8; 32],
    }
    ///Container type for all input parameters for the `ownerOf` function with signature `ownerOf(uint256)` and selector `0x6352211e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ownerOf", abi = "ownerOf(uint256)")]
    pub struct OwnerOfCall {
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256)` and selector `0x42842e0e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256)"
    )]
    pub struct SafeTransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256,bytes)` and selector `0xb88d4fde`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,bytes)"
    )]
    pub struct SafeTransferFromWithFromAndToAndDataCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setApprovalForAll` function with signature `setApprovalForAll(address,bool)` and selector `0xa22cb465`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ::ethers::core::types::Address,
        pub is_approved: bool,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `tokenURI` function with signature `tokenURI(uint256)` and selector `0xc87b56dd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "tokenURI", abi = "tokenURI(uint256)")]
    pub struct TokenURICall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferUsername` function with signature `transferUsername(address,string,string)` and selector `0xe01ecf4b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "transferUsername",
        abi = "transferUsername(address,string,string)"
    )]
    pub struct TransferUsernameCall {
        pub to: ::ethers::core::types::Address,
        pub username_to_transfer: ::std::string::String,
        pub new_username: ::std::string::String,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum BasemailAccountCalls {
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        ChangeUsername(ChangeUsernameCall),
        CreateAccount(CreateAccountCall),
        DeleteAccount(DeleteAccountCall),
        GetAccounts(GetAccountsCall),
        GetApproved(GetApprovedCall),
        GetUsernames(GetUsernamesCall),
        HolderAccounts(HolderAccountsCall),
        IdCounter(IdCounterCall),
        IdToName(IdToNameCall),
        IsApprovedForAll(IsApprovedForAllCall),
        Name(NameCall),
        NameToId(NameToIdCall),
        OwnerOf(OwnerOfCall),
        SafeTransferFrom(SafeTransferFromCall),
        SafeTransferFromWithFromAndToAndData(SafeTransferFromWithFromAndToAndDataCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        TokenURI(TokenURICall),
        TransferFrom(TransferFromCall),
        TransferUsername(TransferUsernameCall),
    }
    impl ::ethers::core::abi::AbiDecode for BasemailAccountCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <ChangeUsernameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChangeUsername(decoded));
            }
            if let Ok(decoded) = <CreateAccountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateAccount(decoded));
            }
            if let Ok(decoded) = <DeleteAccountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeleteAccount(decoded));
            }
            if let Ok(decoded) = <GetAccountsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAccounts(decoded));
            }
            if let Ok(decoded) = <GetApprovedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetApproved(decoded));
            }
            if let Ok(decoded) = <GetUsernamesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetUsernames(decoded));
            }
            if let Ok(decoded) = <HolderAccountsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HolderAccounts(decoded));
            }
            if let Ok(decoded) = <IdCounterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IdCounter(decoded));
            }
            if let Ok(decoded) = <IdToNameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IdToName(decoded));
            }
            if let Ok(decoded) = <IsApprovedForAllCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsApprovedForAll(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <NameToIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NameToId(decoded));
            }
            if let Ok(decoded) = <OwnerOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnerOf(decoded));
            }
            if let Ok(decoded) = <SafeTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeTransferFrom(decoded));
            }
            if let Ok(decoded) = <SafeTransferFromWithFromAndToAndDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeTransferFromWithFromAndToAndData(decoded));
            }
            if let Ok(decoded) = <SetApprovalForAllCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetApprovalForAll(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) = <TokenURICall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenURI(decoded));
            }
            if let Ok(decoded) = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded) = <TransferUsernameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferUsername(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BasemailAccountCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeUsername(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeleteAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAccounts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetApproved(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUsernames(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HolderAccounts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IdCounter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IdToName(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsApprovedForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NameToId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnerOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SafeTransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetApprovalForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenURI(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferUsername(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for BasemailAccountCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeUsername(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateAccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeleteAccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAccounts(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetApproved(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetUsernames(element) => ::core::fmt::Display::fmt(element, f),
                Self::HolderAccounts(element) => ::core::fmt::Display::fmt(element, f),
                Self::IdCounter(element) => ::core::fmt::Display::fmt(element, f),
                Self::IdToName(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsApprovedForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::NameToId(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnerOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetApprovalForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenURI(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferUsername(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApproveCall> for BasemailAccountCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for BasemailAccountCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<ChangeUsernameCall> for BasemailAccountCalls {
        fn from(value: ChangeUsernameCall) -> Self {
            Self::ChangeUsername(value)
        }
    }
    impl ::core::convert::From<CreateAccountCall> for BasemailAccountCalls {
        fn from(value: CreateAccountCall) -> Self {
            Self::CreateAccount(value)
        }
    }
    impl ::core::convert::From<DeleteAccountCall> for BasemailAccountCalls {
        fn from(value: DeleteAccountCall) -> Self {
            Self::DeleteAccount(value)
        }
    }
    impl ::core::convert::From<GetAccountsCall> for BasemailAccountCalls {
        fn from(value: GetAccountsCall) -> Self {
            Self::GetAccounts(value)
        }
    }
    impl ::core::convert::From<GetApprovedCall> for BasemailAccountCalls {
        fn from(value: GetApprovedCall) -> Self {
            Self::GetApproved(value)
        }
    }
    impl ::core::convert::From<GetUsernamesCall> for BasemailAccountCalls {
        fn from(value: GetUsernamesCall) -> Self {
            Self::GetUsernames(value)
        }
    }
    impl ::core::convert::From<HolderAccountsCall> for BasemailAccountCalls {
        fn from(value: HolderAccountsCall) -> Self {
            Self::HolderAccounts(value)
        }
    }
    impl ::core::convert::From<IdCounterCall> for BasemailAccountCalls {
        fn from(value: IdCounterCall) -> Self {
            Self::IdCounter(value)
        }
    }
    impl ::core::convert::From<IdToNameCall> for BasemailAccountCalls {
        fn from(value: IdToNameCall) -> Self {
            Self::IdToName(value)
        }
    }
    impl ::core::convert::From<IsApprovedForAllCall> for BasemailAccountCalls {
        fn from(value: IsApprovedForAllCall) -> Self {
            Self::IsApprovedForAll(value)
        }
    }
    impl ::core::convert::From<NameCall> for BasemailAccountCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NameToIdCall> for BasemailAccountCalls {
        fn from(value: NameToIdCall) -> Self {
            Self::NameToId(value)
        }
    }
    impl ::core::convert::From<OwnerOfCall> for BasemailAccountCalls {
        fn from(value: OwnerOfCall) -> Self {
            Self::OwnerOf(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromCall> for BasemailAccountCalls {
        fn from(value: SafeTransferFromCall) -> Self {
            Self::SafeTransferFrom(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromWithFromAndToAndDataCall>
    for BasemailAccountCalls {
        fn from(value: SafeTransferFromWithFromAndToAndDataCall) -> Self {
            Self::SafeTransferFromWithFromAndToAndData(value)
        }
    }
    impl ::core::convert::From<SetApprovalForAllCall> for BasemailAccountCalls {
        fn from(value: SetApprovalForAllCall) -> Self {
            Self::SetApprovalForAll(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for BasemailAccountCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for BasemailAccountCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TokenURICall> for BasemailAccountCalls {
        fn from(value: TokenURICall) -> Self {
            Self::TokenURI(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for BasemailAccountCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TransferUsernameCall> for BasemailAccountCalls {
        fn from(value: TransferUsernameCall) -> Self {
            Self::TransferUsername(value)
        }
    }
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BalanceOfReturn {
        pub result: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getAccounts` function with signature `getAccounts(address)` and selector `0x3100ceb5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetAccountsReturn(pub ::std::vec::Vec<::ethers::core::types::U256>);
    ///Container type for all return fields from the `getApproved` function with signature `getApproved(uint256)` and selector `0x081812fc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetApprovedReturn {
        pub result: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getUsernames` function with signature `getUsernames(address)` and selector `0x138aae3c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetUsernamesReturn(pub ::std::vec::Vec<::std::string::String>);
    ///Container type for all return fields from the `holderAccounts` function with signature `holderAccounts(address,uint256)` and selector `0x65a9af16`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct HolderAccountsReturn {
        pub accounts: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `idCounter` function with signature `idCounter()` and selector `0xeb08ab28`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IdCounterReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `idToName` function with signature `idToName(uint256)` and selector `0x364410b3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IdToNameReturn {
        pub username: [u8; 32],
    }
    ///Container type for all return fields from the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsApprovedForAllReturn {
        pub result: bool,
    }
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `nameToId` function with signature `nameToId(bytes32)` and selector `0x6c86a430`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct NameToIdReturn {
        pub account_id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `ownerOf` function with signature `ownerOf(uint256)` and selector `0x6352211e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerOfReturn {
        pub result: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SupportsInterfaceReturn {
        pub result: bool,
    }
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `tokenURI` function with signature `tokenURI(uint256)` and selector `0xc87b56dd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TokenURIReturn(pub ::std::string::String);
}
