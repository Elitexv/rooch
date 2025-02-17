// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use move_command_line_common::address::NumericalAddress;
use move_core_types::account_address::AccountAddress;
use std::collections::BTreeMap;

pub const ROOCH_FRAMEWORK_ADDRESS_NAME: &str = "rooch_framework";
pub const ROOCH_FRAMEWORK_ADDRESS_LITERAL: &str = "0x3";
pub const ROOCH_FRAMEWORK_ADDRESS: AccountAddress = {
    let mut addr = [0u8; AccountAddress::LENGTH];
    addr[AccountAddress::LENGTH - 1] = 3u8;
    AccountAddress::new(addr)
};

pub static ROOCH_NAMED_ADDRESS_MAPPING: [(&str, &str); 1] = [(
    ROOCH_FRAMEWORK_ADDRESS_NAME,
    ROOCH_FRAMEWORK_ADDRESS_LITERAL,
)];

pub fn rooch_framework_named_addresses() -> BTreeMap<String, NumericalAddress> {
    let mut address_mapping = moveos_stdlib::moveos_stdlib_named_addresses();
    address_mapping.extend(
        ROOCH_NAMED_ADDRESS_MAPPING
            .iter()
            .map(|(name, addr)| (name.to_string(), NumericalAddress::parse_str(addr).unwrap())),
    );
    address_mapping
}
