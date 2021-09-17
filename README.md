# CasperLabs EIP 1337

Token Subscription for the CasperLabs platform.

## Install
Make sure `wasm32-unknown-unknown` is installed.
```bash
$ make prepare
```

## Build Smart Contract
```bash
$ make build-contract
```

## Test
Test logic and smart contract.
```bash
$ make test
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
