use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use types::{account::AccountHash, Key, URef};

pub const HASHES_DICT: &str = "hashes";

pub struct Hashes {
    dict_uref: URef,
}

impl Hashes {
    pub fn new() -> Hashes {
        let dict_key: Key = runtime::get_key(HASHES_DICT).unwrap_or_revert();
        let dict_uref: &URef = dict_key.as_uref().unwrap_or_revert();
        Hashes {
            dict_uref: *dict_uref,
        }
    }

    pub fn set(&self, account: AccountHash, hash: &str) {
        storage::dictionary_put(self.dict_uref, &account.to_string(), hash);
    }

    pub fn delete(&self, account: AccountHash) {
        storage::dictionary_put(self.dict_uref, &account.to_string(), "");
    }

    pub fn get(&self, account: AccountHash) -> Option<String> {
        storage::dictionary_get(self.dict_uref, &account.to_string()).unwrap_or_revert()
    }
}