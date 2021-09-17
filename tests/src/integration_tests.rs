mod utils;

#[cfg(test)]
mod tests {
    use renvm_sig::hash_message;

    use engine_test_support::{
        Code, Hash, SessionBuilder, TestContext, TestContextBuilder
    };
    
    use types::{
        CLTyped, 
        ContractHash, 
        Key, 
        PublicKey, 
        RuntimeArgs, 
        SecretKey, 
        U256, 
        U512, 
        account::AccountHash, 
        bytesrepr::{
            FromBytes, 
            ToBytes
        }, 
        runtime_args
    };

    use erc20::{
        constants::{
            AMOUNT_RUNTIME_ARG_NAME, 
            OWNER_RUNTIME_ARG_NAME,
            SPENDER_RUNTIME_ARG_NAME,
            RECIPIENT_RUNTIME_ARG_NAME,
        },
        Address, Error, ERC20,
    };

/*     use contract::{
        contract_api::{runtime},
        unwrap_or_revert::UnwrapOrRevert,
    }; */

    use crate::utils::{sign};

    const TOKEN_NAME: &str = "TEST";
    const TOKEN_SYMBOL: &str = "TST";
    const TOKEN_DECIMALS: u8 = 100;
    const TOKEN_TOTAL_SUPPLY: u64 = 1_000_000_000;

    const ARG_NAME: &str = "name";
    const ARG_SYMBOL: &str = "symbol";
    const ARG_DECIMALS: &str = "decimals";
    const ARG_TOTAL_SUPPLY: &str = "total_supply";

    const MINT_ENTRY_POINT_NAME: &str = "mint";
    const TRANSFER_ENTRY_POINT_NAME: &str = "transfer";
    const APPROVE_ENTRY_POINT_NAME: &str = "approve";

    pub const HASHES_DICT: &str = "hashes";
    pub const ALLOWANCES_KEY_NAME: &str = "allowances";
    pub const BALANCES_KEY_NAME: &str = "balances";

    const CONTRACT_NAME: &str = "casper-contract-eip-1337"; //contract name
    const ERC20_CONTRACT_NAME: &str = "erc20_token_contract"; //erc20 contract hash
    const ERC20_CONTRACT_HASH: &str = "erc20_contract_hash"; //erc20 contract hash2

    const TO: &str = "to"; //the publisher
    const FROM: &str = "from_"; //the subscriber
    const TOKEN_AMOUNT: &str = "token_amount"; //the token amount paid to the publisher
    const TOKEN_AMOUNT_VALUE: u64 = 1;
    const PERIOD_SECONDS: &str = "period_seconds"; //the period in seconds between payments
    const GRACE_PERIOD_SECONDS: &str = "grace_period_seconds"; //the grace_period in seconds for is_subscription_active
  
    const PUBLIC: &str = "public"; //the Publickey
    const SIGNATURE: &str = "signature"; //the Signature
    const NEXT_VALID_TIMESTAMP: &str= "next_valid_timestamp"; 

    const PERIOD_SECONDS_VALUE: u64 = 0;

     pub fn generate_eip_1337_secret_key() -> SecretKey {
        SecretKey::ed25519_from_bytes([1u8; 32]).unwrap()
    }

    pub fn get_subscription_data(
        from: AccountHash,
        to: AccountHash,
        token_amount:U256,
        period_seconds:u64,
    ) -> String {
        format!("{}_{}_{}_{}",to,from,token_amount,period_seconds)
    }

    pub fn get_hash_bytes(data:String) -> [u8; 32]
    {
        hash_message(data)
    }

    pub fn get_hex(bytes:[u8;32]) -> String {
        hex::encode(bytes)
    }

    pub struct Subscription {
        context: TestContext,
        pub eip_1337_admin: AccountHash,
        pub eip_1337_admin_pk: PublicKey,
        pub eip_1337_contract_hash: ContractHash,
        pub erc_20_admin: AccountHash,
        pub erc_20_admin_pk: PublicKey,
        pub erc_20_contract_hash: ContractHash,
        pub user_to: AccountHash,
        pub user_to_pk: PublicKey,
        pub user_from: AccountHash,
        pub user_from_pk: PublicKey,
    }

    impl Subscription {

        pub fn deployment() -> Subscription {

            // Create EIP 1337 contract admin.
            let admin_secret = generate_eip_1337_secret_key();
            let admin_key: PublicKey = (&admin_secret).into();
            let admin_addr = AccountHash::from(&admin_key);

            // Create ERC 20 contract admin.
            let erc_20_admin_secret = SecretKey::ed25519_from_bytes([2u8; 32]).unwrap();
            let erc_20_admin_key: PublicKey = (&erc_20_admin_secret).into();
            let erc_20_admin_addr = AccountHash::from(&erc_20_admin_key);

            // Create user.
            let user_secret = SecretKey::ed25519_from_bytes([4u8; 32]).unwrap();
            let user_key: PublicKey = (&user_secret).into();
            let user_addr = AccountHash::from(&user_key);

            // Create user.
            let user_secret_2 = SecretKey::ed25519_from_bytes([4u8; 32]).unwrap();
            let user_key_2: PublicKey = (&user_secret_2).into();
            let user_addr_2 = AccountHash::from(&user_key_2);

            // Create context.
            let mut context = TestContextBuilder::new()
                .with_public_key(admin_key.clone(), U512::from(500_000_000_000_000_000u64))
                .with_public_key(
                    erc_20_admin_key.clone(),
                    U512::from(500_000_000_000_000_000u64),
                )
                .with_public_key(user_key.clone(), U512::from(500_000_000_000_000_000u64))
                .with_public_key(user_key_2.clone(), U512::from(500_000_000_000_000_000u64))
                .build();

            // Deploy the EIP 1337 contract onto the context.
            let erc_20_session_code = Code::from("erc-20.wasm");

            let erc_20_session_args = runtime_args! {
                ARG_NAME => TOKEN_NAME,
                ARG_SYMBOL => TOKEN_SYMBOL,
                ARG_DECIMALS => TOKEN_DECIMALS,
                ARG_TOTAL_SUPPLY => U256::from(TOKEN_TOTAL_SUPPLY),
            };

            let erc_20_session = SessionBuilder::new(erc_20_session_code, erc_20_session_args)
                .with_address(erc_20_admin_addr)
                .with_authorization_keys(&[erc_20_admin_addr])
                .build();

            context.run(erc_20_session);

            let erc_20_contract_hash: ContractHash = context
                .get_account(erc_20_admin_addr)
                .unwrap()
                .named_keys()
                .get(ERC20_CONTRACT_NAME)
                .unwrap()
                .normalize()
                .into_hash()
                .unwrap()
                .into();

            // Deploy the EIP 1337 contract onto the context.
            let session_code = Code::from("casper-contract-eip-1337.wasm");
            
            let session_args = runtime_args! {
                TO => user_addr,
                TOKEN_AMOUNT => U256::from(TOKEN_AMOUNT_VALUE),
                PERIOD_SECONDS => PERIOD_SECONDS_VALUE,
                ERC20_CONTRACT_HASH => erc_20_contract_hash.to_formatted_string(),
            };

            let session = SessionBuilder::new(session_code, session_args)
                .with_address(admin_addr)
                .with_authorization_keys(&[admin_addr])
                .build();

            context.run(session);

            let contract_hash: Hash = context
                .query(
                    admin_addr,
                    &["casper-contract-eip-1337-latest-version-contract-hash".to_string()],
                )
                .unwrap()
                .into_t()
                .unwrap();

            Subscription {
                context,
                eip_1337_admin: admin_addr,
                eip_1337_admin_pk: admin_key.clone(),
                eip_1337_contract_hash: ContractHash::from(contract_hash),
                erc_20_admin: erc_20_admin_addr,
                erc_20_admin_pk: erc_20_admin_key,
                erc_20_contract_hash,
                user_from: user_addr,
                user_from_pk: user_key,
                user_to: user_addr_2,
                user_to_pk: user_key_2,
            }
        }

        fn call(&mut self, caller: &AccountHash, function: &str, args: RuntimeArgs) {
            let code = Code::Hash(self.eip_1337_contract_hash.value(), function.to_string());
            let session = SessionBuilder::new(code, args)
                .with_address(*caller)
                .with_authorization_keys(&[*caller])
                .build();
            self.context.run(session);
        }

        fn call_erc_20(&mut self, caller: &AccountHash, function: &str, args: RuntimeArgs) {
            let code = Code::Hash(self.erc_20_contract_hash.value(), function.to_string());
            let session = SessionBuilder::new(code, args)
                .with_address(*caller)
                .with_authorization_keys(&[*caller])
                .build();
            self.context.run(session);
        }

        fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
            match self.context.query(
                self.eip_1337_admin,
                &["casper-contract-eip-1337-latest-version-contract".to_string(), name.to_string()],
            ) {
                Err(_) => None,
                Ok(maybe_value) => {
                    let value = maybe_value
                        .into_t()
                        .unwrap();
                    Some(value)
                }
            }
        }

        fn query_contract_erc20<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
            match self.context.query(
                self.erc_20_admin,
                &[ERC20_CONTRACT_NAME.to_string(), name.to_string()],
            ) {
                Err(_) => None,
                Ok(maybe_value) => {
                    let value = maybe_value
                        .into_t()
                        .unwrap();
                    Some(value)
                }
            }
        }

        fn query_dictionary_value<T: CLTyped + FromBytes>(
            &self,
            dict_name: &str,
            key: &str,
        ) -> Option<T> {
            match self.context.query_dictionary_item(
                Key::from(self.eip_1337_contract_hash),
                Some(dict_name.to_string()),
                key.to_string(),
            ) {
                Err(_) => None,
                Ok(maybe_value) => {
                    let value: T = maybe_value
                        .into_t()
                        .unwrap();
                    Option::Some(value)
                }
            }
        }

        fn query_dictionary_value_erc20<T: CLTyped + FromBytes>(
            &self,
            dict_name: &str,
            key: &str,
        ) -> Option<T> {
            match self.context.query_dictionary_item(
                Key::from(self.erc_20_contract_hash),
                Some(dict_name.to_string()),
                key.to_string(),
            ) {
                Err(err) => panic!(err),
                Ok(maybe_value) => {
                    let value: T = maybe_value
                        .into_t()
                        .unwrap();
                    Option::Some(value)
                }
            }
        }

        pub fn to(&self) -> AccountHash {
            self.query_contract(TO).unwrap()
        }

        pub fn token_amount(&self) -> U256 {
            self.query_contract(TOKEN_AMOUNT).unwrap()
        }

        pub fn period_seconds(&self) -> u64 {
            self.query_contract(PERIOD_SECONDS).unwrap()
        }

        pub fn from(&self) -> AccountHash {
            self.query_contract(FROM).unwrap()
        }

        pub fn next_valid_timestamp(&self) -> u64 {
            self.query_contract(NEXT_VALID_TIMESTAMP).unwrap()
        }

        pub fn get_subscription_hash(
            &mut self,
            caller: AccountHash,
            from: AccountHash,
        ) -> String {
            
            self.call(
                &caller,
                "create_subscription_hash",
                runtime_args! {
                    "from" => from,
                },
            );

            let subscription_hash: String = self.query_dictionary_value(
                HASHES_DICT, 
                &from.to_string(),
            ).unwrap();
            
            subscription_hash
        }
        
        pub fn execute_subscription(
            &mut self,
            caller: AccountHash,
            public: PublicKey,
            signature: String,
            from: AccountHash,
        ) {
            self.call(
                &caller,
                "execute_subscription",
                runtime_args! {
                    "public" => public,
                    "signature" => signature,
                    "from" => from
                },
            );
        }

        pub fn cancel_subscription(
            &mut self,
            caller: AccountHash,
            public: PublicKey,
            signature: String,
            from: AccountHash,
        ) {
            self.call(
                &caller,
                "cancel_subscription",
                runtime_args! {
                    "public" => public,
                    "signature" => signature,
                    "from" => from
                },
            );
        }

        pub fn is_subscription_ready(
            &mut self,
            caller: AccountHash,
            public: PublicKey,
            signature: String,
            from: AccountHash,
        ) {
            self.call(
                &caller,
                "is_subscription_ready",
                runtime_args! {
                    "public" => public,
                    "signature" => signature,
                    "from" => from
                },
            );
        }

        pub fn is_subscription_active(
            &mut self,
            caller: AccountHash,
            subscription_hash: String,
            grace_period_seconds: u64,
        ) {
            self.call(
                &caller,
                "is_subscription_active",
                runtime_args! {
                    "subscription_hash_" => subscription_hash,
                    "grace_period_seconds" => grace_period_seconds
                },
            );
        }
    }

    #[test]
    fn test_eip1337_deploy() {
        let s = Subscription::deployment();
        assert_eq!(s.to(), s.user_to);
        assert_eq!(s.token_amount(), U256::from(TOKEN_AMOUNT_VALUE));
        assert_eq!(s.period_seconds(), PERIOD_SECONDS_VALUE);

        //panic!("AccountHash: {}",s.user);
    }

    #[test]
    fn test_execute_subscription() {
        let mut s = Subscription::deployment();
        let user_from = s.user_from;
        let user_to = s.user_to;
        let erc_20_admin = s.erc_20_admin;
        let eip_1337_admin = s.eip_1337_admin;
        let eip_1337_admin_pk = s.eip_1337_admin_pk.clone();
        let eip_1337_contract_hash = s.eip_1337_contract_hash.clone();

        // Give the owner 1000 tokens
        s.call_erc_20(
            &erc_20_admin, 
            TRANSFER_ENTRY_POINT_NAME, 
            runtime_args! {
                RECIPIENT_RUNTIME_ARG_NAME => Address::Account(user_from.clone()),
                AMOUNT_RUNTIME_ARG_NAME => U256::from(1000),
            },
        );

        // TODO: Fix this once we have a simpler way of querying balances
        /*let balance_uref: Key = s.query_contract_erc20(BALANCES_KEY_NAME).unwrap();*/

        // Check that hte owner has 1000 tokens
        let bytes_from = erc_20_admin.to_bytes().unwrap();
        let user_from_b64 = base64::encode(&bytes_from);

        println!("DICT {} {}", BALANCES_KEY_NAME.to_string(), user_from_b64);
        let balance_from: U256 = s.query_dictionary_value_erc20(
            &BALANCES_KEY_NAME.to_string(), 
            &user_from_b64,        ).unwrap();
        assert_eq!(balance_from, U256::from(1000));

        // Give the spender contract permission to spend 1000 tokens
        s.call_erc_20(
            &user_from.clone(), 
            APPROVE_ENTRY_POINT_NAME, 
            runtime_args! {
                SPENDER_RUNTIME_ARG_NAME => Address::Contract(eip_1337_contract_hash.clone()),
                AMOUNT_RUNTIME_ARG_NAME => U256::from(1000),
            },
        );

    /*     // Check the approval of the spender contract
        let mut preimage = Vec::new();
        preimage.append(&mut user_from.to_bytes().unwrap_or_revert());
        preimage.append(&mut eip_1337_admin.to_bytes().unwrap_or_revert());

        let key_bytes = runtime::blake2b(&preimage);
        let key_hex = hex::encode(&key_bytes);

        let allowance: U256 = s.query_dictionary_value_erc20(ALLOWANCES_KEY_NAME, &key_hex).unwrap();

        // Check that allowance is 1000 tokens
        println!("ALLOWACE OF OWNER {} TO SPENDER {} IS {}", user_from.to_string(), eip_1337_admin.to_string(), allowance);
        assert_eq!(allowance, U256::from(1000)); */

        // Generate a subscription hash in contract
        let subscription_hash = s.get_subscription_hash(
            eip_1337_admin,
            user_from,
        );

        // Generate a subscription hash in test
        let mut subscription_bytes = [0u8;32];
        hex::decode_to_slice(subscription_hash.clone(), &mut subscription_bytes as &mut [u8]);

        let sub_data = get_subscription_data(user_from, user_to, U256::from(TOKEN_AMOUNT_VALUE), PERIOD_SECONDS_VALUE);
        let sub_bytes = get_hash_bytes(sub_data);
        let sub_hex = get_hex(sub_bytes);

        // Check if the subscription hashes match
        println!("SUB_HASH {} == {}", subscription_hash.clone(), sub_hex);
        assert_eq!(subscription_hash, sub_hex);
        assert_eq!(subscription_bytes, sub_bytes);

        // Sign the subscription hash 
        let signature = sign(
            generate_eip_1337_secret_key(),
            subscription_bytes,
        );

        // TODO: This won't work well unless we can get the contract hash into the contract scope

/*      // Use the signed subscription hash to check if the allowance is >= 1 token and that the transaction can execute
        s.is_subscription_ready(
            eip_1337_admin,
            eip_1337_admin_pk.clone(),
            signature.clone(), 
            user_from,
        );  */

        // Use the signed subscription hash to execute a payment of 1 token
        s.execute_subscription(
            eip_1337_admin,
            eip_1337_admin_pk,
            signature, 
            user_from,
        );
        
        // TODO: Fix this once we have a simpler way of querying balances
        /* // Check final balances
        let bytes_from2 = user_from.to_bytes().unwrap();
        let user_from_b642 = base64::encode(&bytes_from2);
        let balance_from2: U256 = s.query_dictionary_value_erc20(
            BALANCES_KEY_NAME, 
            &user_from_b642,
        ).unwrap();
        
        assert_eq!(balance_from2, U256::from(999));

        let bytes_to = user_to.to_bytes().unwrap();
        let user_to_b64 = base64::encode(&bytes_to);
        let balance_to: U256 = s.query_dictionary_value_erc20(
            BALANCES_KEY_NAME, 
            &user_to_b64,
        ).unwrap(); 
    
        assert_eq!(balance_to, U256::from(1)); */
    }
/*
    #[test]
    fn test_is_subscription_ready() {
        let mut s = Subscription::deployment();
        let from_value:AccountHash=AccountHash::from_formatted_str(FROM_ACCOUNT_HASH).unwrap();
        s.is_subscription_ready(Sender(s.user_to),PUBLIC_VALUE.to_string(),SIGNATURE_VALUE.to_string(),from_value);

    }

    #[test]
    fn test_is_subscription_active() {
        let mut s = Subscription::deployment();
        let grace_period_seconds:u64=300;
        s.is_subscription_active(Sender(s.user_to),SUBSCRIPTION_HASH_VALUE.to_string(),grace_period_seconds);
    }

    #[test]
    fn test_cancel_subscription() {
        let mut s = Subscription::deployment();
        let from_value:AccountHash=AccountHash::from_formatted_str(FROM_ACCOUNT_HASH).unwrap();
        s.cancel_subscription(Sender(s.user_to),PUBLIC_VALUE.to_string(),SIGNATURE_VALUE.to_string(),from_value);
        assert_eq!(s.next_valid_timestamp(),99999999999*1000);

    }*/
}

fn main() {
    panic!("The main should not be used here");
}