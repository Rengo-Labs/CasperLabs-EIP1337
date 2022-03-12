# Casper EIP 1337 Subscription Billing Standard

EIP 1337 subscription billing standard implemented for the Casper Blockchain.  

First, [deploy this contract](#deploy-onto-casper) by providing an [ERC20](https://github.com/casper-ecosystem/erc20) contract address, your receiving account address, token amount, and subscription period.  

Then, have the sending user permit this contract to transfer tokens on their behalf up to the total amount agreed upon and [generate a subscription hash](#getting-a-subscription-hash) to send you.  

Finally, sign the subscription hash with the private key of your receiving account and call [execute-subscription](#getting-paid) after the allotted subscription period has passed until the total amount of tokens is transferred.

## Setting Up This Contract

### Requirements

1. Install the [rust environment and casper client](https://docs.casperlabs.io/en/latest/.dapp-dev-guide/setup-of-rust-contract-sdk.html)

2. Install [wasm-strip](https://command-not-found.com/wasm-strip)

3. Clone this repo and navigate into the folder.
  ```bash
  $ git clone https://github.com/davidtai/casper-eip-1337.git
  ```

4. The address of the ERC20 compatible contract that you want to use. Use the example one [here](https://github.com/casper-ecosystem/erc20/blob/master/example/erc20-token/src/main.rs) for testing.

5. A receiving Casper account.  An easy way to set one up is using the [Casperlabs Signer](https://docs.cspr.community/docs/user-guides/SignerGuide.html).

### Set up the Rust toolchain
You need the Rust toolchain to develop smart contracts. Make sure `wasm32-unknown-unknown` is installed.
```bash
$ make prepare
```

### Build Smart Contract
```bash
$ make build-contract
```

### Build The Signer Utility
```bash
$ make build-signer
```

### Test
Test logic and smart contract.
```bash
$ make test
```

### Generate Contract Private Keys

```bash
$ casper-client keygen contract-keys 
```

### Deploy onto Casper
In this example, we will deploy to testnet.

```bash
casper-client put-deploy \
  --chain-name casper-test \
  --node-address <HOST:PORT> \
  --secret-key <CONTRACT_SECRET_KEY_FILE> \
  --session-path target/wasm32-unknown-unknown/release/contract.wasm \
  --payment-amount 400000000000 \
  --session-arg="to:account_hash='<YOUR_RECEIVING_ACCOUNT_HASH>'" \
  --session-arg="token_amount:u256='<SUBSCRIPTION_AMOUNT>'" \
  --session-arg="period_seconds:u64='<PERIOD_SECONDS>'" \
  --session-arg="erc20_contract_hash:key='<ERC20_CONTRACT_HASH>" \
```

A successful response will look like:
```json
{
  "api_version":"1.0.0",
  "deploy_hash":"8c3068850354c2788c1664ac6a275ee575c8823676b4308851b7b3e1fe4e3dcc"
}
```

Once the network has received the deployment, it will queue up in the system before being listed in a block for execution. Sending a transaction (deployment) to the network does not mean that the transaction processed successfully. Therefore, it’s important to check to see that the contract executed properly, and that the block was finalized.

```bash
$ casper-client get-deploy --node-address http://<HOST:PORT> <DEPLOY_HASH>
```

Depending on your preference, it may be more convenient to just go to the cspr.live block explorer though after a minute or so:

```
https://testnet.cspr.live/deploy/<DEPLOY_HASH>
```

### Allow the Contract Access to the Sender's ERC20 Balance

First get the latest state hash.

```bash
casper-client get-state-root-hash --node-address <HOST:PORT> | jq -r
```

The return value will look something like this.

```bash
dde389e00bc1b533bad9ae1d70fc50f8fc7b76670a7fb4b8f0ff47b9218bd1ad
```

Second, get the account ERC20 account's information.

```bash
casper-client query-state --node-address <HOST:PORT> -k <ERC_CONTRACT_PUBLIC_KEY_HEX> -s <STATE_HASH>
```

Third, get the ERC-20 contract session-hash which looks like this.

```json
  ...
    "named_keys": [
      {
        "key": "hash-696e6e9f03320606ebd62003145eb74d7683e245cc62b94fabe1174c10671abe",
        "name": "erc20_token_contract"
      }
    ]
  ...
```

Set up the allowance so that the EIP-1337 subscription contract can spend on behalf of the sender.

```bash
casper-client put-deploy \
  --chain-name casper-test \
  --node-address <HOST:PORT> \
  --secret-key <SENDER_SECRET_KEY_FILE> \
  --payment-amount 100000000000 \
  --session-hash=<ERC20_CONTRACT_HASH> \
  --session-entry-point="approve" \
  --session-arg="spender:key='<EIP_1337_CONTRACT_PACKAGE_HASH>'" \
  --session-arg="amount:u256='<SUBSCRIPTION_AMOUNT>'" \
```

This authorizes the contract-package-hash (contract-hash changes when upgrades, the contract-package-hash does not) to spend some amount on behalf of the sender. After the deploy successfully deploys, get the latest state-root-hash and check the balance to make sure the account has something to send.

```bash
casper-client get-state-root-hash --node-address <HOST:PORT> | jq -r
```

Get the balance key using the script.

```bash
./scripts/base64_key.sh <SENDER_PUBLIC_KEY>
```

Query to double check that the balances is set correctly.

```bash
casper-client get-dictionary-item -s <STATE_HASH>
 --dictionary-name balances --contract-hash <ERC20_CONTRACT_HASH> --node-address <HOST:PORT> --dictionary-item-key <BASE64_KEY>
```

### Generate the subscription hash as the sender

First, get the sender account hash.

```bash
casper-client account-address <SENDER_PUBLIC_KEY_FILE>
```

Second, create the subscription hash.

```bash
casper-client put-deploy \
  --chain-name casper-test \
  --node-address <HOST:PORT> \
  --secret-key <SENDER_SECRET_KEY_FILE> \
  --payment-amount 10000000000 \
  --session-hash="<EIP_1337_CONTRACT_HASH>" \
  --session-entry-point="create_subscription_hash" \
  --session-arg="public:public_key='<SENDER_PUBLIC_KEY_HEX>'" \
  --session-arg="from:account_hash='<SENDER_ACCOUNT_ADDRESS>'" \
```

Third, after the deploy is completed, get the latest state hash.

```bash
casper-client get-state-root-hash --node-address <HOST:PORT> | jq -r
```

Fourth, get the hash from the EIP-1337 `hashes` dictionary.

```bash
casper-client get-dictionary-item -s <STATE_HASH> \
  --contract-hash <EIP_1337_CONTRACT_HASH> \
  --node-address <HOST:PORT> \
  --dictionary-name hashes \
  --dictionary-item-key <SENDER_ACCOUNT_ADDRESS (minus account-hash)> \
```

### Sign the subscription hash as the sender

Build and run the signer utility on the subscription hash.

```bash
./bin/subscription_hash_signer <SENDER_SECRET_KEY_FILE> <SUBSCRIPTION_HASH>
```

Issue this signed subscription hash to the entity that is in charge of causing the subscription (usually the receiver).  It will be checked against the internally registered public key to issue a payment.

### Execute the subscription payment

Execute the subscription using the signed subscription hash.

```
casper-client put-deploy \
  --chain-name casper-test \
  --node-address <HOST:PORT> \
  --secret-key <RECEIVER_SECRET_KEY_FILE> \
  --payment-amount 10000000000 \
  --session-hash="<EIP_1337_CONTRACT_HASH>" \
  --session-entry-point="execute_subscription" \
  --session-arg="signature:string='<SIGNED_SUBSCRIPTION_HASH>'" \
  --session-arg="from:account_hash='<SENDER_ACCOUNT_HASH>'" \

```

Once a payment is executed, the timestamp will be internally updated to restrict payments until that time if there are still funds left to be transferred.  If an insufficient approved amount exists in the referenced ERC-20, then this function will fail.

### Cancel the subscription

Cancel the subscription using the signed subscription hash.

```
casper-client put-deploy \
  --chain-name casper-test \
  --node-address <HOST:PORT> \
  --secret-key <RECEIVER_SECRET_KEY_FILE> \
  --payment-amount 10000000000 \
  --session-hash="<EIP_1337_CONTRACT_HASH>" \
  --session-entry-point="cancel_subscription" \
  --session-arg="signature:string='<SIGNED_SUBSCRIPTION_HASH>'" \
  --session-arg="from:account_hash='<SENDER_ACCOUNT_HASH>'" \

```

## Entry Point methods 

Following are the EIP-1337 entry point methods.

- #### is_subscription_active 

This function is used by external smart contracts to verify on-chain that a particular subscription is "paid" and "active" there must be a small grace period added to allow the publisher or desktop miner to execute.

Following is the table of parameters.

Parameter Name | Type
---|---
subcription_hash | string 
grace_period_seconds | u64


This method **returns** nothing.

- #### get_subscription_hash 
Given the subscription details, generate blake2b standard hash or get it if it has been generated, external interface. This function stores the hash and public key into the the `hashes` and `pubkey` dictionaries under the `from` account hash (`account-hash` prefix stripped).

Following is the table of parameters.

Parameter Name | Type
---|---
public | PublicKey
from | AccountHash

This method **returns** blake2b standard hash.

- #### create_subscription_hash 

Given the subscription details, generate blake2b standard hash, external interface, do not return.  This function stores the hash and public key into the the `hashes` and `pubkey` dictionaries under the `from` account hash (`account-hash` prefix stripped).

Following is the table of parameters.

Parameter Name | Type
---|---
public | PublicKey
from | AccountHash

This method **returns** nothing.  `get_subscription_hash` is the version that returns the created hash (or stored hash if it exists) and should be used when calling from another contract. 


- #### cancel_subscription 

You don't really need this if you are using the approve/transferFrom method
because you control the flow of tokens by approving this contract address, but use this to cancel the subscription using just the contract.

Following is the table of parameters.

Parameter Name | Type
---|---
signature | string 
from | AccountHash

This method **returns** nothing.


- #### is_subscription_ready

Check if a subscription is signed correctly and the timestamp
is ready for the next execution to happen.

Following is the table of parameters.

Parameter Name | Type
---|---
signature | string 
from | AccountHash


This method **returns** nothing.


- #### execute_subscription 

Execute the transferFrom to pay the publisher from the subscriber, 
the subscriber has full control by approving this contract-package-hash an allowance.

Following is the table of parameters.

Parameter Name | Type
---|---
signature | string 
from | AccountHash

This method **returns** nothing.

