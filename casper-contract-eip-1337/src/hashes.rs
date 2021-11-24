use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use types::{
    account::AccountHash,
    crypto::{PublicKey},
    Key, URef,
};

pub const HASHES_DICT: &str = "hashes";
pub const PUBKEYS_DICT: &str = "pubkeys";
pub const PREFIX: &str = "account-hash-";

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
        let key = &account.to_formatted_string().replace(PREFIX, "");

        storage::dictionary_put(self.dict_uref, key, hash);
        storage::dictionary_put(self.pubkeys_dict_uref, key, public_key);
    }

    pub fn delete(&self, account: AccountHash) {
        let key = &account.to_formatted_string().replace(PREFIX, "");

        storage::dictionary_put(self.dict_uref, key, "");
        storage::dictionary_put(self.pubkeys_dict_uref, key, "");
    }

    pub fn get(&self, account: AccountHash) -> (Option<String>, Option<PublicKey>) {
        let key = &account.to_formatted_string().replace(PREFIX, "");

        let hash: Option<String> = storage::dictionary_get(self.dict_uref, key).unwrap_or_revert();
        let public_key: Option<PublicKey> = storage::dictionary_get(self.pubkeys_dict_uref, key).unwrap_or_revert();

        (hash, public_key)
    }

    pub fn get_public_key(&self, account: AccountHash) -> Option<PublicKey> {
        let key = &account.to_formatted_string().replace(PREFIX, "");
        
        storage::dictionary_get(self.pubkeys_dict_uref, key).unwrap_or_revert()
    }
}