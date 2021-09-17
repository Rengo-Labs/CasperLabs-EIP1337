#![no_main]
use contract::{
    contract_api::{runtime},
};

mod constants;

// All session code must have a `call` entrypoint, 
#[no_mangle]
fn call() {
    casper_contract_eip_1337::install_or_upgrade_contract(
        String::from("casper-contract-eip-1337"),
        runtime::get_named_arg(constants::TO),
        runtime::get_named_arg(constants::TOKEN_AMOUNT),
        runtime::get_named_arg(constants::PERIOD_SECONDS),
        runtime::get_named_arg(constants::ERC20_CONTRACT_HASH),
    );
}
