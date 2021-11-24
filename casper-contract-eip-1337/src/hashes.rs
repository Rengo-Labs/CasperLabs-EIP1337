use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use types::{
    account::AccountHash,
    crypto::{PublicKey, Signature},
    Key, URef,
};

pub const HASHES_DICT: &str = "hashes";
pub const PUBKEYS_DICT: &str = "pubkeys";

pub struct Hashes {
    dict_uref: URef,
    pubkeys_dict_uref: URef,
}

impl Hashes {
    pub fn new() -> Hashes {
        let dict_key: Key = runtime::get_key(HASHES_DICT).unwrap_or_revert();
        let dict_uref: &URef = dict_key.as_uref().unwrap_or_revert();

        let pubkeys_dict_key: Key = runtime::get_key(PUBKEYS_DICT).unwrap_or_revert();
        let pubkeys_dict_uref: &URef = pubkeys_dict_key.as_uref().unwrap_or_revert();

        Hashes {
            dict_uref: *dict_uref,
            pubkeys_dict_uref: *pubkeys_dict_uref,
        }
    }

    pub fn set(&self, account: AccountHash, hash: &str, public_key: PublicKey) {
        storage::dictionary_put(self.dict_uref, &account.to_string(), hash);
        storage::dictionary_put(self.pubkeys_dict_uref, &account.to_string(), public_key);
    }

    pub fn delete(&self, account: AccountHash) {
        storage::dictionary_put(self.dict_uref, &account.to_string(), "");
    }

    pub fn get(&self, account: AccountHash) -> (Option<String>, Option<PublicKey>) {
        let hash: Option<String> = storage::dictionary_get(self.dict_uref, &account.to_string()).unwrap_or_revert();
        let public_key: Option<PublicKey> = storage::dictionary_get(self.pubkeys_dict_uref, &account.to_string()).unwrap_or_revert();

        (hash, public_key)
    }

    pub fn get_public_key(&self, account: AccountHash) -> Option<PublicKey> {
        storage::dictionary_get(self.pubkeys_dict_uref, &account.to_string()).unwrap_or_revert()
    }
}