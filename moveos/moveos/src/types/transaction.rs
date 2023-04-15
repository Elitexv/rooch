// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use move_core_types::{
    account_address::AccountAddress,
    identifier::Identifier,
    language_storage::{ModuleId, TypeTag},
};
use moveos_statedb::HashValue;
use serde::{Deserialize, Serialize};

#[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Script {
    #[serde(with = "serde_bytes")]
    pub code: Vec<u8>,
    pub ty_args: Vec<TypeTag>,
    pub args: Vec<Vec<u8>>,
}

#[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Function {
    pub module: ModuleId,
    pub function: Identifier,
    pub ty_args: Vec<TypeTag>,
    pub args: Vec<Vec<u8>>,
}

#[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum MoveTransaction {
    //Execute a Move script
    Script(Script),
    //Execute a Move function
    Function(Function),
    //Publish Move modules
    ModuleBundle(Vec<Vec<u8>>),
}

impl MoveTransaction {
    pub fn new_module_bundle(modules: Vec<Vec<u8>>) -> Self {
        Self::ModuleBundle(modules)
    }
    pub fn new_function(
        module: ModuleId,
        function: Identifier,
        ty_args: Vec<TypeTag>,
        args: Vec<Vec<u8>>,
    ) -> Self {
        Self::Function(Function {
            module,
            function,
            ty_args,
            args,
        })
    }
    pub fn new_script(code: Vec<u8>, ty_args: Vec<TypeTag>, args: Vec<Vec<u8>>) -> Self {
        Self::Script(Script {
            code,
            ty_args,
            args,
        })
    }
}

pub trait AbstractTransaction {
    fn senders(&self) -> Vec<AccountAddress>;
    fn into_move_transaction(self) -> MoveTransaction;
    fn txn_hash(&self) -> HashValue;
}

#[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct SimpleTransaction {
    pub sender: AccountAddress,
    pub txn: MoveTransaction,
}

impl SimpleTransaction {
    pub fn new(sender: AccountAddress, txn: MoveTransaction) -> Self {
        Self { sender, txn }
    }
}

impl AbstractTransaction for SimpleTransaction {
    fn senders(&self) -> Vec<AccountAddress> {
        vec![self.sender]
    }

    fn into_move_transaction(self) -> MoveTransaction {
        self.txn
    }

    fn txn_hash(&self) -> HashValue {
        HashValue::sha3_256_of(bcs::to_bytes(&self).unwrap().as_slice())
    }
}

// TODO Fix the constructor
#[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PublishPackageTransaction {
    pub sender: AccountAddress,
    pub module_bytes: Vec<u8>,
}

impl PublishPackageTransaction {
    pub fn new(sender: AccountAddress, module_bytes: Vec<u8>) -> Self {
        Self {
            sender,
            module_bytes,
        }
    }
}

impl AbstractTransaction for PublishPackageTransaction {
    fn senders(&self) -> Vec<AccountAddress> {
        vec![self.sender]
    }

    // TODO fix build the ModuleBundle
    fn into_move_transaction(self) -> MoveTransaction {
        MoveTransaction::ModuleBundle(vec![self.module_bytes])
    }

    fn txn_hash(&self) -> HashValue {
        HashValue::sha3_256_of(bcs::to_bytes(&self).unwrap().as_slice())
    }
}

// TODO Fix the constructor
#[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct FunctionTransaction {
    pub sender: AccountAddress,
    pub function_bytes: Vec<u8>,
}

impl FunctionTransaction {
    pub fn new(sender: AccountAddress, function_bytes: Vec<u8>) -> Self {
        Self {
            sender,
            function_bytes,
        }
    }
}

impl AbstractTransaction for FunctionTransaction {
    fn senders(&self) -> Vec<AccountAddress> {
        vec![self.sender]
    }

    // TODO fix build the Function
    fn into_move_transaction(self) -> MoveTransaction {
        // TODO deserialize functoin bytes
        let f: Function = bcs::from_bytes(&self.function_bytes).unwrap();
        MoveTransaction::Function(f)
    }

    fn txn_hash(&self) -> HashValue {
        HashValue::sha3_256_of(bcs::to_bytes(&self).unwrap().as_slice())
    }
}
