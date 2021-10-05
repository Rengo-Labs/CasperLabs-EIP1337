# Casper EIP 1337 Subscription Billing Standard

EIP 1337 subscription billing standard implemented for the Casper Blockchain.  

First, [deploy this contract](#how-to-deploy) by providing an [ERC20](https://github.com/casper-ecosystem/erc20) contract address, your receiving account address, token amount, and subscription period.  

Then, have the sending user permit this contract to transfer tokens on their behalf up to the total amount agreed upon and [generate a subscription hash](#getting-a-subscription-hash) to send you.  

Finally, sign the subscription hash with the private key of your receiving account and call [execute-subscription](#getting-paid) after the allotted subscription period has passed until the total amount of tokens is transferred.

## Setting Up This Contract

### Requirements

1. Install the [rust environment and casper client](https://docs.casperlabs.io/en/latest/.dapp-dev-guide/setup-of-rust-contract-sdk.html)

2. Clone this repo and navigate into the folder.
  ```bash
  $ git clone https://github.com/davidtai/casper-eip-1337.git
  ```

3. The address of the ERC20 compatible contract that you want to use.

4. A receiving Casper account.  An easy way to set one up is using the [Casperlabs Signer](https://docs.cspr.community/docs/user-guides/SignerGuide.html).

### Set up the Rust toolchain
You need the Rust toolchain to develop smart contracts. Make sure `wasm32-unknown-unknown` is installed.
```bash
$ make prepare
```

### Build Smart Contract
```bash
$ make build-contract
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

### Deploy onto Casper Testnet/Mainnet
In this example, we will deploy to testnet.

```bash
casper-client put-deploy 
  --chain-name casper-test \
  --node-address <NODE_ADDRESS> \
  --secret-key contract-keys/secret_key.pem \
  --session-path target/wasm32-unknown-unknown/release/contract.wasm \
  --payment-amount 13500000000 \
  --session-arg="to:account_hash='<YOUR_RECEIVING_ACCOUNT_HASH>'" \
  --session-arg="token_amount:U256='<AMOUNT>'" \
  --session-arg="period_seconds:u64='<PERIOD_SECONDS>'" \
  --session-arg="erc20_contract_hash:string='<ERC20_CONTRACT_ADDRESS>" \
  /
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
$ casper-client get-deploy --chain-name < casper-test> --node-address http://<HOST:PORT> <DEPLOY_HASH>
```


# How to Deploy and call functions of the eip 1337 contract
```bash
Read commandsfordeployment file in the root directory of eip 1337 project 
```

# ExecuteSubscription Flow
```bash

Steps:

Note: You first have to deploy Erc20 to call executesubcription method of eip1337 
(Because, you first have to approve publisher accounthash for funds transfer).  

1) Setup casper from this document (https://docs.google.com/document/d/17bC-iNOZ7sf-oinQxnbPzuQ4P8Dtid-5-WoYfzp6hi4/edit?usp=sharing) 

2) All the important casper commands are in commandsfordeploymenty file.

3) Now that your casper environment is setup and you know how to do deployment and query the contract, Clone erc20 project and make keys in the root directory using casper-client keygen keys.

4) Import your key in casper signer wallet in google chrome. 

5) Get faucet from casper live -> Tools tab -> faucet tab.

6) Deploy erc20 contract using command no 1 in commandsfordeployment file (edit the command for erc20 arguments).

7) Make Keys in the root directory of eip1337 project using casper-client keygen keys.

8) Repeat step no 2 and 3 .

9) Deploy approve method of erc20 and provide accounthash of the spender in argument (spender) and a large value in argument (amount).
Note: You have to get the latest state-root-hash and then erc20 hash see command no 3,4 and 5 in commandsfordeployment file (edith them according to your needs)
 
10) Deploy eip1337 using command no 1 in commandsfordeployment file (edit the command (like argument values) if you want to).
Note:(provide the erc20 contract hash you get in step no 6 in string)

11) You can check the status of deployment using command no 2 in commandsfordeployment file (edit the command with new deploy hash).

11) Provide same data you provided when deploying eip1337, in sign meta transaction rust project.

12) Retrieve Public Key and Signature from doing step no 2 in sign meta transaction rust project.

13) Now you have all three arguments for execute_subscription method (Publickey (pass as string " "), signature (pass as string " ") and from), deploy the method using command no 6 in commandsfordeployment file (edit the command if required).

14) Repeat step 2 for deployment status(edit the command with new deploy hash).

15) Now you can query the state and check the results using command no 5 in commandsfordeployment file (edit the command with new state-root-hash and eip1337 hash).

16) You can query all the keys changing the keys at the end of the command used in step 6. 

```
