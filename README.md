# ParallelChain Client CLI (pchain-client)

`pchain_client` is an easy-to-use, fully-featured CLI for interacting with ParallelChain. 
For a detailed description of all available commands, execute `pchain_client --help`. 

## Usgae 
```sh
ParallelChain Client CLI 0.4.1
<ParallelChain Lab>
ParallelChain client (`pchain_client`) is a command-line tool for you to connect and interact with
the ParallelChain Mainnet/Testnet.

USAGE:
    pchain_client <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    transaction    Construct and submit Transactions to ParallelChain network
    query          Query blockchain and world state information for ParallelChain network
    keys           Locally stores and manage account keypairs you created. (Password required)
    parse          Utilities functions to deserialize return values in CommandReceipt, and
                       compute contract address
    config         Get and set Fullnode RPC url to interact with ParallelChain
    help           Print this message or the help of the given subcommand(s)
```

## Why pchain_client
`pchain_client` allows you to query data from the ParallelChain, submit transaction, and more, all at the comfort of your command line.\
Checkout the examples below for more information or check the full list of commands. The following document walks through the CLI's essential workflows. 

New users can begin either by 
1. [Install and Setup](#install-and-setup) or,
2. [Prepare Environment](#prepare-environment) or,
3. [Setting up new account](#generate-new-keypair)


If you are lost at any step, you can always type `pchain_client --help`.

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->

## Table of Contents
- [Install and Setup](#install-and-setup)
  - [Installation](#installation)
  - [Running pchain_client](#running-pchain_client)
- [Prepare Environment](#prepare-environment)
- [Manage Account](#manage-account)
  - [Generate new keypair](#generate-new-keypair)
  - [Import exsiting keypair](#import-exsiting-keypair)
  - [List Accounts](#list-accounts)
- [Transaction](#transaction)
  - [Prepare Transaction File](#prepare-transaction-file)
    - [Create new Transaction File](#create-new-transaction-file)
    - [Append Command to existing file](#append-command-to-existing-file)
  - [Submit Transaction to Parallelchain](#submit-transaction-to-parallelchain)
- [Query](#query)
  - [Check Account related information](#check-account-related-information)
  - [Get Transaction with receipt](#get-transaction-with-receipt)
  - [Get Deposit and Stake](#get-deposit-and-stake)
- [Smart Contract](#smart-contract)
  - [Retrieve Contract Address](#retrieve-contract-address)
  - [Prepare Contract method arguments file](#prepare-contract-method-arguments-file)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## Install and Setup
### Installation
`pchain_client` is available for Unix/Linux, MacOS and Windows. You need to download only pre-built binary applicable for your platform and install pchain client.

Here are the simple steps to install pchain client.
  - Open a Web browser and go to [https://XXXXXX/downloads/]() (TBC).
  - Follow the link to download pre-built binary available for your platform.
  - Run the downloaded file.

**NOTE:**
If this is your first time using `pchain_client`, you need to setup `$PCHAIN_CLI_HOME` in environment variables to specify the home path. See more [here](https://chlee.co/how-to-setup-environment-variables-for-windows-mac-and-linux/).

### Running pchain_client
First time using `pchain_client` will be required to setup password to protect your account keypairs. Please note that this password is different from the password used in Parallelchain explorer.

Command:
```sh
pchain_client ---version
```
You will require to enter your password twice. If your password is set successfully, you will see return message with pchain_client version on console.

**WARNING:**
The password is not sent and saved in anywhere. You won't be able to recover the password if you lost it. Please keep your password safe. You will require to provide this password to submit transaction and manage keypairs later.

## Prepare Environment
Before you can submit transaction or query information on Parallelchain, you need to setup your own choice of ParallelChain RPC API provider URL.

Command:
```sh
pchain_client config setup --url <URL>
```
This would check the status of your chosen provider. If pchain client cannot connect to your provider, a warning message will be shown and setup is failed. You need to setup another url with the above command again.

## Manage Account
In parallelchain, account is identified by the public key of Ed25519 keypair. You can either generate new keys or import your exsiting Ed25519 keypair to making transaction in pchain_client. Both operations are password required.

### Generate new keypair
This command generates a set of ed25519_dalek compatible keys. Random name will be set if you do not provide a name.
```sh
pchain_client keys create --name <NAME>
```

### Import exsiting keypair
If you already get keys from Parallelchain explorer, you can import your account keypair with this command. Random name will be set if you do not provide a name.
```sh
pchain_client keys add --private <PRIVATE_KEY> --public <PUBLIC_KEY> --name <NAME>

// PRIVATE_KEY and PUBLIC_KEY are Base64encoded Ed25519 keys.
```
### List Accounts
After create or add keypair, you can check it using following command to list out all public keys managed in this tool.
```sh
pchain_client keys list
```

## Transaction 
A transaction is a digitally signed instruction that tells the Parallelchain state machine to execute a sequence of commands. There are different kinds of [Commands](/protocol/Runtime.md) in ParallelChain protocol. 

`pchain_client` accepts transaction in json format. This section will demonstrate how to prepare your transaction file and submit with your account keys.
### Prepare Transaction File
`pchain_client` provides user-friendly way to prepare your transaction file without prior knowledge of JSON (JavaScript Object Notation) format.
The transaction file with 2 parts: `Parameters` and `Subcommand`.

Here are some CLI subcommands to indicate corresponding [Protocal Transaction Command](/protocol/Runtime.md). 

| Subcommand | Action          | Description                                           |
|------------|-----------------|-------------------------------------------------------|
| transfer   |                 | Transfer Balance from transaction signer to recipient |
| deploy     |                 | Deploy smart contract to the state of the blockchain  |
| call       |                 | Trigger method call of a deployed smart contract      |
| deposit    |                 | Deposit some balance into the network account         |
|            | create          | Instantiation of a Deposit of existing Pool           |
|            | top-up          | Increase balance of an existing Deposit               |
|            | withdraw        | Withdraw balance from an existing Deposit             |
|            | update-settings | Update settings of an existing Deposit                |
| stake      |                 | Stake to particular pool                              |
|            | stake           | Increase stakes to an existing Pool                   |
|            | unstake         | Remove stakes from an existing Pool                   |
| pool       |                 | Create amd manage Pool                                |
|            | create          | Instantiation of a Pool in the network account        |
|            | update-settings | Update settings of an existing Pool                   |
|            | delete          | Delete an existing Pool in the network account        |

#### Create new Transaction File
`Transaction` in ParallelChain protocol specifies a set of parameters included in the instruction. You don't need to provide all parameters, some of them would be computed and filled in automatically when you submit the transaction.

```sh
pchain_client transaction create --help
```

First, provide following 4 parameters:
```sh
pchain_client transaction create \
  --nonce <NONCE> \
  --gas-limit <GAS_LIMIT> \
  --max-base-fee-per-gas <MAX_BASE_FEE_PER_GAS> \
  --priority-fee-per-gas <PRIORITY_FEE_PER_GAS> \
...
```
Then, decide the transaction type using the [subcommand](#prepare-transaction-file). Each of them takes different inputs. You can always check help menu with `--help`.

Make sure you provide both `Parameters` and `Subcommand` parts in one command. The output transaction file (tx.json) will be saved in current directory. You can also specify the designated file with flag `--destination`

Examples:
```sh
// Transfer Tokens
pchain_client transaction create \
  --nonce 0 \
  --gas-limit 100000 \
  --max-base-fee-per-gas 8 \
  --priority-fee-per-gas 0 \
  transfer \
    --recipient kRPL8cXI73DNgVSSQz9WfIi-mAAvFvdXKfZ9UPBEv_A \
    --amount 100
```
```sh
// Deploy Contract, save to designated file `deposit-tx.json`
pchain_client transaction create \
  --destination ~/Documents/deposit-tx.json \
  --nonce 0 \
  --gas-limit 100000 \
  --max-base-fee-per-gas 8 \
  --priority-fee-per-gas 0 \
  deploy \
    --contract-code /home/document/code.wasm \
    --cbi-version 0
```

#### Append Command to existing file
As explained in the beginning of [Transaction](#transaction) section, Transaction in ParallelChain protocol accept sequence of commands. But you may find that `transaction create` in previous section only support single Command in Transaction. 

If you want to support multiple Commands, use following command with the [subcommand](#prepare-transaction-file). This appends a `Command` element to the back of the command array in Transaction. Please note that commands in array will be executed in sequential order.

Example:
```sh
pchain_client transaction append \
  --file ~/Documents/deposit-tx.json \
  transfer \
    --recipient kRPL8cXI73DNgVSSQz9WfIi-mAAvFvdXKfZ9UPBEv_A \
    --amount 100
```

### Submit Transaction to Parallelchain
After prepared the transaction json file, you can now submit the transaction with keypair.

Command:
```sh
pchain_client transaction submit \
--file <FILE> \
--keypair-name <KEYPAIR_NAME>
```
You will get following response if the transaction is accepted by your provider:
```sh
{
  "API Response:": "Your Transaction has been received.",
  "Command(s):": [
    {
      "Deploy": {
        "cbi_version": 0,
        "contract": "<contract in 53476 bytes>"
      }
    }
  ],
  "Contract Address:": "EH-0Im5Pb5mZQumIP6AAxyqTU7fBWQsNfLdGfaBh8AE",
  "Signature:": "DdRr2l-f3SwWtQP7M5JKdOUEvIb-th2mBrV1z06dkvB2rpp0qKQZwBBzJBh8czCqplUsmzSlSjPNrvOQbx2jAA",
  "Transaction Hash:": "POikFlLT8sVuVt3RHJvxmzPKP8dfvi55TrME6Muc80I"
}
```


## Query
`pchain_client` allows you to query different data from the ParallelChain, not just Transaction or Account related information, but also Validators and Stake Pool details in Parallelchain network. 

Use `pchain_client query --help` to check the full list avaliable to query.

### Check Account related information
To check Externally Owned Accounts (EOA) information such as balance and nonce, you always need to provide your account address (public key).

Command:
```sh
pchain_client query balance --address <ADDRESS>
pchain_client query nonce --address <ADDRESS>
```

For Contract Account, you can use another command to get all information such as balance, nonce, cbi version and download the contract code binary file(wasm) at once.

Command:
```sh
pchain_client query contract \
  --address <ADDRESS>
  --with-code
```

### Get Transaction with receipt
In [Submit Transaction to Parallelchain](#submit-transaction-to-parallelchain) section, after you successfully make transaction on Parallekchain, you should receive the transaction hash (tx_hash) in response. This hash is the identity of your transaction. You can always retreive the transaction details with/without receipt by the transaction hash.

Command:
```sh
pchain_client query tx 
  --hash <TX_HASH>
  --with-receipt
```

If you just want to get the receipt, you can use following command
```sh
pchain_client query receipt --hash <TX_HASH>
```

### Get Deposit and Stake
You can query deposit or stake amount of an account from specific pool stored in Network Account.

Command:
```sh
pchain_client query deposit --operator <OPERATOR> --owner <OWNER>

pchain_client query stake --operator <OPERATOR> --owner <OWNER>
```

## Smart Contract 
Smart contracts are computer programs that are stored on a blockchain. You need to provide some necessary information such as contract address, method name, and arguments in order to invoke method of the contract.

### Retrieve Contract Address
After you deploy contract in trasnaction, you should receive the contract address together with transaction hash. What if you want to deploy contract and call method in the SAME transaction, it is possible to compute the contract address in advance.

You need to provide the account address and nonce when deploying contract.

Without proof:
Command:
```sh
pchain_client parse contract-address --address <ADDRESS> --nonce <NONCE>
```

### Prepare Contract method arguments file
When you make a contract call that modify or view state, the contract method may expects arguments. You need to provide arguments by JSON file(.json) in `transaction create call` or `query view` commands.

Example:
For a contract method accept 3 arguments (String, Vec<i16> , boolean)
```sh
{
    "arguments": [
        {"argument_type": "String", "argument_value": "Yuru Camp"},
        {"argument_type": "Vec<i16>", "argument_value":"[-1, 20]"},
        {"argument_type": "bool", "argument_value": "true"}
    ]
}
```
Each object in arguments array consists two fields `argument_type` and `argument_value`.
Here are some acceptable types and values.

| Type        | Description                                   | example                              |
|-------------|-----------------------------------------------|--------------------------------------|
| `i8`        | The 8-bit signed integer type.                | "-128"                               |
| `i16`       | The 16-bit signed integer type.               | "-32768"                             |
| `i32`       | The 32-bit signed integer type.               | "-2147483648"                        |
| `i64`       | The 64-bit signed integer type.               | "-9223372036854775808"               |
| `u8`        | The 8-bit unsigned integer type.              | "255"                                |
| `u16`       | The 16-bit unsigned integer type.             | "65535"                              |
| `u32`       | The 32-bit unsigned integer type.             | "4294967295"                         |
| `u64`       | The 64-bit unsigned integer type.             | "18446744073709551615"               |
| `String`    | String                                        | "This is test string"                |
| `bool`      | Boolean                                       | "true" or "false"                    |
| `Vec<TYPE>` | Array with specific type and arbitrary length | "[65535,6535]" , "[true,false,true]" |
| `[5]`       | Array with specific length                    | "[1,2,3,4,5]"                        |
