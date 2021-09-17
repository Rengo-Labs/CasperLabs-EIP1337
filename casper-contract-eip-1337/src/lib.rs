use renvm_sig::hash_message;

use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert 
};

use types::{
    account::AccountHash,
    contracts::{ContractPackageHash, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys}, 
    crypto::{PublicKey, Signature},
    CLValue, CLTyped, CLType, Parameter, RuntimeArgs,runtime_args,ContractHash,U256,ApiError,Key,
};

mod utils;
mod constants;
mod hashes;

use hashes::Hashes;
  
/// Enum for ContractError, It represents codes for different smart contract errors.
#[derive(Debug)]
pub enum ContractError {

    /// 65,536 for (signer verification failed)
    SignerFailed = 20,  
    /// 65,538 for (blocktime less than nextvalidtimestamp)      
    InvalidBlockTime = 22, 
    /// 65,539 for (allowance is less than token_amount)       
    InsufficientAllowance = 23, 
    /// 65,540 for (subcription not active)   
    SubscriptionNotActive = 24,      

    ReadingCallerError = 25,
}

impl From<ContractError> for ApiError {
    fn from(err: ContractError) -> ApiError {
        ApiError::User(err as u16)
    }
}

/// This function is used by external smart contracts to verify on-chain that a
/// particular subscription is "paid" and "active"
/// there must be a small grace period added to allow the publisher
/// or desktop miner to execute.
/// 
/// # Parameters
///
/// * `subcription_hash_` - A string slice that holds the eip-191 standard's subcription_hash of the meta transaction
///
/// * `grace_period_seconds` - An u64 that holds the grace seconds to see if transaction gets active by adding some grace seconds
/// 
#[no_mangle]
pub fn is_subscription_active()
{

    let subscription_hash:String=runtime::get_named_arg(constants::SUBSCRIPTION_HASH);
    let grace_period_seconds:u64=runtime::get_named_arg(constants::GRACE_PERIOD_SECONDS);

    let blocktime:u64 =runtime::get_blocktime().into();
    let next_valid_timestamp_key:String=format!("{}{}", constants::NEXT_VALID_TIMESTAMP,subscription_hash);
    let next_valid_timestamp:u64=utils::get_key(&next_valid_timestamp_key).unwrap_or_revert();

    if blocktime < next_valid_timestamp+(grace_period_seconds*1000)
    {
         // subscription not active
        runtime::revert(ApiError::User(ContractError::SubscriptionNotActive as u16));
    }

}

/// Given the subscription details, generate a message string
/// # Parameters
///
/// * `from` - An Accounthash that holds the account address of the subscriber/signer
///
pub fn get_subscription_data(from:AccountHash) -> String
{
    let to:AccountHash=utils::get_key(constants::TO).unwrap_or_revert();
    let token_amount:U256=utils::get_key(constants::TOKEN_AMOUNT).unwrap_or_revert();
    let period_seconds:u64=utils::get_key(constants::PERIOD_SECONDS).unwrap_or_revert();

    format!("{}_{}_{}_{}",to,from,token_amount,period_seconds)
}

/// Given the subscription details, generate eip-191 standard hash, internal implementation.
/// # Parameters
///
/// * `data` - A string slice that holds the meta transaction data
///
pub fn get_subscription_hash_bytes(data:String) -> [u8; 32]
{
    hash_message(data)
}

/// Given the subscription details, generate eip-191 standard hash, internal implementation.
/// # Parameters
///
/// * `data` - A string slice that holds the meta transaction data
///
pub fn _get_subscription_hash(data:String) -> String
{
    let eip191_standard_hash= get_subscription_hash_bytes(data);
    let eip191_standard_hash_string = hex::encode(eip191_standard_hash);
    
    eip191_standard_hash_string
}

/// Given the subscription details, generate eip-191 standard hash, external interface.
/// # Parameters
///
/// * `from` - An Accounthash that holds the account address of the subscriber/signer
///
#[no_mangle]
pub fn get_subscription_hash()
{
    let from: AccountHash = runtime::get_named_arg(constants::FROM);

    let data: String = get_subscription_data(from);

    let hashes = Hashes::new();

    let opt = hashes.get(from);

    match opt {
        Some(hash) => {
            let blocktime:u64 =runtime::get_blocktime().into();
            let period_seconds:u64=utils::get_key(constants::PERIOD_SECONDS).unwrap_or_revert();
            let next_valid_timestamp:u64=blocktime + 1000 * period_seconds;
            let next_valid_timestamp_key:String=format!("{}{}", constants::NEXT_VALID_TIMESTAMP,hash);
            utils::set_key(&next_valid_timestamp_key,next_valid_timestamp); 
        
            runtime::ret(CLValue::from_t(hash).unwrap_or_revert());
        },
        None => {
            let hash: String = _get_subscription_hash(data);
            hashes.set(from, &hash);

            let blocktime:u64 =runtime::get_blocktime().into();
            let period_seconds:u64=utils::get_key(constants::PERIOD_SECONDS).unwrap_or_revert();
            let next_valid_timestamp:u64=blocktime + 1000 * period_seconds;
            let next_valid_timestamp_key:String=format!("{}{}", constants::NEXT_VALID_TIMESTAMP,hash);
            utils::set_key(&next_valid_timestamp_key,next_valid_timestamp); 
        
            runtime::ret(CLValue::from_t(hash).unwrap_or_revert());
        }
    }
}

/// Given the subscription details, generate eip-191 standard hash, external interface, do not return (use this until text is fixed).
/// # Parameters
///
/// * `from` - An Accounthash that holds the account address of the subscriber/signer
///
#[no_mangle]
pub fn create_subscription_hash()
{
    let from: AccountHash = runtime::get_named_arg(constants::FROM);

    let data: String = get_subscription_data(from);

    let hashes = Hashes::new();

    let opt = hashes.get(from);

    match opt {
        Some(hash) => {
            let blocktime:u64 =runtime::get_blocktime().into();
            let period_seconds:u64=utils::get_key(constants::PERIOD_SECONDS).unwrap_or_revert();
            let next_valid_timestamp:u64=blocktime + 1000 * period_seconds;
            let next_valid_timestamp_key:String=format!("{}{}", constants::NEXT_VALID_TIMESTAMP,hash);
            utils::set_key(&next_valid_timestamp_key,next_valid_timestamp); 
        },
        None => {
            let hash: String = _get_subscription_hash(data);
            hashes.set(from, &hash);

            let blocktime:u64 =runtime::get_blocktime().into();
            let period_seconds:u64=utils::get_key(constants::PERIOD_SECONDS).unwrap_or_revert();
            let next_valid_timestamp:u64=blocktime + 1000 * period_seconds;
            let next_valid_timestamp_key:String=format!("{}{}", constants::NEXT_VALID_TIMESTAMP,hash);
            utils::set_key(&next_valid_timestamp_key,next_valid_timestamp); 
        }
    }
}


/// This function is to get subcription signer and verify if it is equal
/// to the signer public key or not. 
/// 
/// # Parameters
///
/// * `public_key` - A string slice that holds the public key of the meta transaction signer,  Subscriber have to get it from running cryptoxide project externally.
///
/// * `signature` - A string slice that holds the signature of the meta transaction,  Subscriber have to get it from running cryptoxide project externally.
/// 
/// * `get_eip191_standard_hash` - A u8 array that holds the eip-191 standard subcription hash of the meta transaction
/// 
pub fn get_subscription_signer_and_verification(public_key:PublicKey,signature:Signature,eip191_hash_bytes:[u8;32]) -> bool
{
    if let PublicKey::Ed25519(pub_key) = public_key 
    {
        if let Signature::Ed25519(sig) = signature 
        {
            pub_key.verify_strict(&eip191_hash_bytes, &sig).unwrap();
            return true;
        }
    }

    false
}

/// You don't really need this if you are using the approve/transferFrom method
/// because you control the flow of tokens by approving this contract address,
/// but to make the contract an extensible example for later user I'll add this.
///  
/// # Parameters
///
/// * `public` - A string slice that holds the public key of the meta transaction signer,  Subscriber have to get it from running cryptoxide project externally.
///
/// * `signature` - A string slice that holds the signature of the meta transaction,  Subscriber have to get it from running cryptoxide project externally.
/// 
/// * `from` - An Accounthash that holds the account address of the subscriber/signer
#[no_mangle]
pub fn cancel_subscription()
{
    let public_key:PublicKey= runtime::get_named_arg(constants::PUBLIC);
    let signature:String = runtime::get_named_arg(constants::SIGNATURE);
    let from: AccountHash = runtime::get_named_arg(constants::FROM);

    let data:String = get_subscription_data(from);
    let subscription_hash_bytes: [u8;32] = get_subscription_hash_bytes(data);
    let mut sig_bytes = [0u8;64];
    
    hex::decode_to_slice(signature, &mut sig_bytes as &mut [u8]).unwrap();

    let sig = Signature::ed25519(sig_bytes).unwrap();

    let result:bool = get_subscription_signer_and_verification(public_key,sig,subscription_hash_bytes);

    if !result
    {
         //  signature verification failed  
         runtime::revert(ApiError::User(ContractError::SignerFailed as u16));
    }

    //subscription will become valid again Wednesday, November 16, 5138 9:46:39 AM
    //at this point the nextValidTimestamp should be a timestamp that will never
    //be reached during the brief window human existence
    
    let next_valid_timestamp:u64=99999999999*1000;
    let subscription_hash_string:String=hex::encode(subscription_hash_bytes);
    let next_valid_timestamp_key:String=format!("{}{}", constants::NEXT_VALID_TIMESTAMP,subscription_hash_string);
    utils::set_key(&next_valid_timestamp_key,next_valid_timestamp); 
    
}

///Check if a subscription is signed correctly and the timestamp
///is ready for the next execution to happen.
/// 
/// # Parameters
///
/// * `public` - A string slice that holds the public key of the meta transaction signer,  Subscriber have to get it from running cryptoxide project externally.
///
/// * `signature` - A string slice that holds the signature of the meta transaction,  Subscriber have to get it from running cryptoxide project externally.
/// 
/// * `from` - An Accounthash that holds the account address of the subscriber/signer
#[no_mangle]
pub fn is_subscription_ready()
{
    let public_key:PublicKey= runtime::get_named_arg(constants::PUBLIC);
    let signature:String = runtime::get_named_arg(constants::SIGNATURE);
    let from:AccountHash = runtime::get_named_arg(constants::FROM);

    let to:AccountHash=utils::get_key(constants::TO).unwrap_or_revert();
    let token_amount:U256=utils::get_key(constants::TOKEN_AMOUNT).unwrap_or_revert();

    let data:String = get_subscription_data(from);
    let subscription_hash_bytes: [u8;32] = get_subscription_hash_bytes(data);
    let mut sig_bytes = [0u8;64];
    
    hex::decode_to_slice(signature, &mut sig_bytes as &mut [u8]).unwrap();

    let sig = Signature::ed25519(sig_bytes).unwrap();

    let result:bool = get_subscription_signer_and_verification(public_key,sig,subscription_hash_bytes);

    // if signature verification Successfully
    if result 
    {
        let blocktime:u64=runtime::get_blocktime().into();
        let subscription_hash_string:String=hex::encode(subscription_hash_bytes);
        let next_valid_timestamp_key:String=format!("{}{}", constants::NEXT_VALID_TIMESTAMP,subscription_hash_string);
        let next_valid_timestamp:u64 = utils::get_key(&next_valid_timestamp_key).unwrap_or_revert();
        
        if blocktime >= next_valid_timestamp
        {
            let contract_hash_string: String = utils::get_key(constants::ERC20_CONTRACT_HASH).unwrap_or_revert();
            let contract_hash = ContractHash::from_formatted_str(&contract_hash_string).unwrap_or_default();
           
            let allowance_result:U256=runtime::call_contract(
                contract_hash,
                "allowance",
                runtime_args!{
                    "owner" => Key::Account(from),
                    "spender" => Key::Hash(utils::get_key(&"package_hash".to_string()).unwrap_or_revert()),
                }
            );

            if allowance_result < token_amount
            {
                // subscription not ready (allowance is less than token_amount)
                runtime::revert(ApiError::User(ContractError::InsufficientAllowance as u16));
            }
        }
        else
        {
            // subscription not ready (blocktime is less than next_valid_timestamp)
            runtime::revert(ApiError::User(ContractError::InvalidBlockTime as u16));
        }
    }
    else
    {
        // signature verification failed 
        runtime::revert(ApiError::User(ContractError::SignerFailed as u16));
    }

}

///  Execute the transferFrom to pay the publisher from the subscriber, 
///  the subscriber has full control by approving this contract an allowance.
/// 
/// # Parameters
///
/// * `public` - A string slice that holds the public key of the meta transaction signer, Subscriber have to get it from running cryptoxide project externally.
///
/// * `signature` - A string slice that holds the signature of the meta transaction, Subscriber have to get it from running cryptoxide project externally.
/// 
/// * `from` - An Accounthash that holds the account address of the subscriber/signer
#[no_mangle]
pub fn execute_subscription()
{
    let public_key:PublicKey= runtime::get_named_arg(constants::PUBLIC); 
    let signature:String = runtime::get_named_arg(constants::SIGNATURE);
    let from: AccountHash = runtime::get_named_arg(constants::FROM);

    let period_seconds:u64=utils::get_key(constants::PERIOD_SECONDS).unwrap_or_revert();
    let to:AccountHash=utils::get_key(constants::TO).unwrap_or_revert();
    let token_amount:U256=utils::get_key(constants::TOKEN_AMOUNT).unwrap_or_revert();

    let data:String = get_subscription_data(from);
    let subscription_hash_bytes: [u8;32] = get_subscription_hash_bytes(data);
    let mut sig_bytes = [0u8;64];

    hex::decode_to_slice(signature, &mut sig_bytes as &mut [u8]).unwrap();

    let sig = Signature::ed25519(sig_bytes).unwrap();

    let result:bool = get_subscription_signer_and_verification(public_key,sig,subscription_hash_bytes);

     // if signature verification is Successfull
    if result 
    {

        let blocktime:u64 =runtime::get_blocktime().into();
        let subscription_hash_string:String=hex::encode(subscription_hash_bytes);
        let next_valid_timestamp_key:String=format!("{}{}", constants::NEXT_VALID_TIMESTAMP,subscription_hash_string);
        let mut next_valid_timestamp:u64= utils::get_key(&next_valid_timestamp_key).unwrap_or_revert();

        if blocktime >= next_valid_timestamp
        {
            if next_valid_timestamp == 0
            {
                next_valid_timestamp=blocktime;
            }
            next_valid_timestamp=next_valid_timestamp+(period_seconds*1000);
            utils::set_key(&next_valid_timestamp_key, next_valid_timestamp);

            let contract_hash_string: String = utils::get_key(constants::ERC20_CONTRACT_HASH).unwrap_or_revert();
            let contract_hash = ContractHash::from_formatted_str(&contract_hash_string).unwrap();
            
            let transfer_from_result: () = runtime::call_contract(
                contract_hash,
                "transfer_from",
                runtime_args!{
                    "owner" => Key::Account(from),
                    "recipient" => Key::Account(to),
                    "amount" => token_amount
                }
            );
        }
        else
        {
            //blocktime is less than next_valid_timestamp
            runtime::revert(ApiError::User(ContractError::InvalidBlockTime as u16));
        }
    }
    else
    {
        // signature verification failed 
        runtime::revert(ApiError::User(ContractError::SignerFailed as u16));
    }

}

/// Returns the list of the entry points in the contract with added group security.
pub fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        String::from("is_subscription_active"),
        vec![
            Parameter::new(constants::SUBSCRIPTION_HASH, String::cl_type()),
            Parameter::new(constants::GRACE_PERIOD_SECONDS, u64::cl_type())
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("is_subscription_ready"),
        vec![
            Parameter::new(constants::PUBLIC, PublicKey::cl_type()),
            Parameter::new(constants::SIGNATURE,String::cl_type()),
            Parameter::new(constants::FROM, AccountHash::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("cancel_subscription"),
        vec![
            Parameter::new(constants::PUBLIC, PublicKey::cl_type()),
            Parameter::new(constants::SIGNATURE,String::cl_type()),
            Parameter::new(constants::FROM, AccountHash::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("create_subscription_hash"),
        vec![
            Parameter::new(constants::FROM, AccountHash::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("get_subscription_hash"),
        vec![
            Parameter::new(constants::FROM, AccountHash::cl_type()),
        ],
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        String::from("execute_subscription"),
        vec![
            Parameter::new(constants::PUBLIC, PublicKey::cl_type()),
            Parameter::new(constants::SIGNATURE,String::cl_type()),
            Parameter::new(constants::FROM, AccountHash::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}

/// Install or upgrade the contract.
/// # Parameters
///
/// * `name` - Contract name
///
/// * `to` - An Accounthash that holds the account address of the publisher
///
/// * `token_amount` - An U256 that holds the token amount that publisher wants from subscribers
/// 
/// * `period_seconds` - An u64 that holds the interval in seconds between payments
///
/// * `erc20_contract_hash` - A String slice that holds the contracthash of erc20 contract
pub fn install_or_upgrade_contract(
    name: String,
    to: AccountHash,
    token_amount: U256,
    period_seconds: u64,
    erc20_contract_hash: String,
) {
    let mut named_keys: NamedKeys = Default::default();
    let contract_package_hash: ContractPackageHash =
        match runtime::get_key(&format!("{}-package", name)) {
            Some(contract_package_hash) => {
                contract_package_hash.into_hash().unwrap_or_revert().into()
            }
            None => {
                let (contract_package_hash, access_token) =
                    storage::create_contract_package_at_hash();

                runtime::put_key(&format!("{}-package", name), 
                    contract_package_hash.into()
                );

                runtime::put_key(
                    &format!("{}-package-access-uref", name),
                    access_token.into(),
                );
                
                runtime::put_key(
                    &format!("{}-package-hash", name),
                    storage::new_uref(contract_package_hash).into(),
                );                       
                         
                named_keys.insert(constants::TO.to_string(), storage::new_uref(to).into());
                named_keys.insert(constants::TOKEN_AMOUNT.to_string(), storage::new_uref(token_amount).into());
                named_keys.insert(constants::PERIOD_SECONDS.to_string(), storage::new_uref(period_seconds * 1000).into());
                named_keys.insert(constants::ERC20_CONTRACT_HASH.to_string(), storage::new_uref(erc20_contract_hash).into());

                // Add empty dictionary for hashes.
                let hashes_dict = storage::new_dictionary(hashes::HASHES_DICT).unwrap_or_revert();
                named_keys.insert(hashes::HASHES_DICT.to_string(), hashes_dict.into());
                
                // Store package hash.
                named_keys.insert(
                    "package_hash".to_string(),
                    storage::new_uref(contract_package_hash).into(),
                ); 

                contract_package_hash
            }
        };
 
        let entry_points = get_entry_points();
        let (contract_hash, _) =
            storage::add_contract_version(contract_package_hash, entry_points, named_keys);
    
        runtime::put_key(
            &format!("{}-latest-version-contract", name),
            contract_hash.into(),
        );
    
        runtime::put_key(
            &format!("{}-latest-version-contract-hash", name),
            storage::new_uref(contract_hash).into(),
        ); 
}

