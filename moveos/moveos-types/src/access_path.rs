// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use crate::{
    move_string::MoveString,
    object::{NamedTableID, ObjectID},
    state_resolver::{self, module_name_to_key, resource_tag_to_key},
};
use move_core_types::{
    account_address::AccountAddress, identifier::Identifier, language_storage::StructTag,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Path {
    /// Get Object
    Object { object_ids: Vec<ObjectID> },
    /// Get account resources
    Resource {
        account: AccountAddress,
        resource_types: Vec<StructTag>,
    },
    /// Get account modules
    Module {
        account: AccountAddress,
        module_names: Vec<Identifier>,
    },
    /// Get table values by keys
    Table {
        table_handle: ObjectID,
        keys: Vec<Vec<u8>>,
    },
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Path::Object { object_ids } => {
                write!(
                    f,
                    "/object/{}",
                    object_ids
                        .iter()
                        .map(|object_id| object_id.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                )?;
            }
            Path::Resource {
                account,
                resource_types,
            } => {
                write!(
                    f,
                    "/resource/{}/{}",
                    account.to_hex_literal(),
                    resource_types
                        .iter()
                        .map(|struct_tag| struct_tag.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                )?;
            }
            Path::Module {
                account,
                module_names,
            } => {
                write!(
                    f,
                    "/module/{}/{}",
                    account.to_hex_literal(),
                    module_names
                        .iter()
                        .map(|module_name| module_name.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                )?;
            }
            Path::Table { table_handle, keys } => {
                write!(
                    f,
                    "/table/{}/{}",
                    table_handle,
                    keys.iter()
                        .map(|key| {
                            let hex_key = hex::encode(key);
                            format!("0x{hex_key}")
                        })
                        .collect::<Vec<_>>()
                        .join(",")
                )?;
            }
        }
        Ok(())
    }
}

impl FromStr for Path {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches('/');
        let mut iter = s.split('/');
        let path_type = iter.next().ok_or_else(|| anyhow::anyhow!("Invalid path"))?;
        match path_type {
            "object" => {
                let object_ids = iter.next().ok_or_else(|| anyhow::anyhow!("Invalid path"))?;
                let object_ids = object_ids
                    .split(',')
                    .map(ObjectID::from_str)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Path::Object { object_ids })
            }
            "resource" => {
                let account = iter.next().ok_or_else(|| anyhow::anyhow!("Invalid path"))?;
                let account = AccountAddress::from_hex_literal(account)?;
                let resource_types = iter.next().ok_or_else(|| anyhow::anyhow!("Invalid path"))?;
                let resource_types = resource_types
                    .split(',')
                    .map(StructTag::from_str)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Path::Resource {
                    account,
                    resource_types,
                })
            }
            "module" => {
                let account = iter.next().ok_or_else(|| anyhow::anyhow!("Invalid path"))?;
                let account = AccountAddress::from_hex_literal(account)?;
                let module_names = iter.next().ok_or_else(|| anyhow::anyhow!("Invalid path"))?;
                let module_names = module_names
                    .split(',')
                    .map(Identifier::from_str)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Path::Module {
                    account,
                    module_names,
                })
            }
            "table" => {
                let table_handle = iter.next().ok_or_else(|| anyhow::anyhow!("Invalid path"))?;
                let table_handle = ObjectID::from_str(table_handle)?;
                let keys = iter.next().ok_or_else(|| anyhow::anyhow!("Invalid path"))?;
                let keys = keys
                    .split(',')
                    .map(|key| {
                        match key.strip_prefix("0x") {
                            Some(key) => hex::decode(key)
                                .map_err(|_| anyhow::anyhow!("Invalid path key: {}", key)),
                            None => {
                                //if key not start with `0x`, we think it is a utf8 string
                                let move_str = MoveString::from_str(key)?;
                                Ok(bcs::to_bytes(&move_str)?)
                            }
                        }
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Path::Table { table_handle, keys })
            }
            _ => Err(anyhow::anyhow!("Invalid path: {}", s)),
        }
    }
}

/// StateDB query path
///
/// 1. /object/$object_id1[,$object_id2]*
/// 2. /resource/$account_address/$resource_type1[,$resource_type2]*
/// 3. /module/$account_address/$module_name1[,$module_name2]*
/// 4. /table/$table_handle/$key1[,$key2]*
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccessPath(pub Path);

impl AccessPath {
    pub fn object(object_id: ObjectID) -> Self {
        AccessPath(Path::Object {
            object_ids: vec![object_id],
        })
    }

    pub fn objects(object_ids: Vec<ObjectID>) -> Self {
        AccessPath(Path::Object { object_ids })
    }

    pub fn resource(account: AccountAddress, resource_type: StructTag) -> Self {
        AccessPath(Path::Resource {
            account,
            resource_types: vec![resource_type],
        })
    }

    pub fn resources(account: AccountAddress, resource_types: Vec<StructTag>) -> Self {
        AccessPath(Path::Resource {
            account,
            resource_types,
        })
    }

    pub fn module(account: AccountAddress, module_name: Identifier) -> Self {
        AccessPath(Path::Module {
            account,
            module_names: vec![module_name],
        })
    }

    pub fn modules(account: AccountAddress, module_names: Vec<Identifier>) -> Self {
        AccessPath(Path::Module {
            account,
            module_names,
        })
    }

    pub fn table(table_handle: ObjectID, keys: Vec<Vec<u8>>) -> Self {
        AccessPath(Path::Table { table_handle, keys })
    }

    /// Convert AccessPath to TableQuery, return the table handle and keys
    /// All other AccessPath is a shortcut for TableQuery
    pub fn into_table_query(self) -> (ObjectID, Vec<Vec<u8>>) {
        match self.0 {
            Path::Table { table_handle, keys } => (table_handle, keys),
            Path::Object { object_ids } => {
                let table_handle = state_resolver::GLOBAL_OBJECT_STORAGE_HANDLE;
                let keys = object_ids
                    .iter()
                    .map(|object_id| object_id.to_bytes())
                    .collect();
                (table_handle, keys)
            }
            Path::Module {
                account,
                module_names,
            } => {
                let table_handle = NamedTableID::Module(account).to_object_id();
                let keys = module_names
                    .into_iter()
                    .map(|name| module_name_to_key(&name))
                    .collect();
                (table_handle, keys)
            }
            Path::Resource {
                account,
                resource_types,
            } => {
                let resource_table_id = NamedTableID::Resource(account).to_object_id();
                let keys = resource_types
                    .into_iter()
                    .map(|tag| resource_tag_to_key(&tag))
                    .collect();
                (resource_table_id, keys)
            }
        }
    }
}

impl std::fmt::Display for AccessPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

impl FromStr for AccessPath {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = s.parse::<Path>()?;
        Ok(AccessPath(path))
    }
}

// AccessPath always serialize and deserilaize as string
impl Serialize for AccessPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for AccessPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<AccessPath>().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_path_roundtrip(path: &str) {
        let path = path.parse::<Path>().unwrap();
        let path_str = path.to_string();
        let path2 = path_str.parse::<Path>().unwrap();
        assert_eq!(path, path2);
    }

    #[test]
    pub fn test_path() {
        test_path_roundtrip("/object/0x1");
        test_path_roundtrip("/object/0x1,0x2");
        test_path_roundtrip("/resource/0x1/0x2::m::S");
        test_path_roundtrip("/resource/0x1/0x2::m1::S1,0x3::m2::S2");
        test_path_roundtrip("/module/0x2/m1");
        test_path_roundtrip("/module/0x2/m1,m2");
        test_path_roundtrip("/table/0x1/0x12");
        test_path_roundtrip("/table/0x1/0x12,0x13");
        test_path_roundtrip("/table/0x1/key1,key2");
    }
}
