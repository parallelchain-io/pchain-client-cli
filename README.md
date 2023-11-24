# ParallelChain Client CLI (pchain-client)

`pchain_client` is an easy-to-use, fully-featured CLI for interacting with ParallelChain. 
For a detailed description of all available commands, execute `pchain_client --help`. 

## Usage 
```sh
ParallelChain Client CLI 0.4.4
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
`pchain_client` allows you to query data from the ParallelChain, submit transactions, and more, all at the comfort of your command line.\
Check out the examples below for more information or see the full list of commands. The following document walks through the CLI's essential workflows. 

New users can begin either by 
1. [Install and Setup](#install-and-setup) or,
2. [Prepare Environment](#prepare-environment) or,
3. [Setting up New Account](#generate-new-keypair)


If you are lost at any step, you can always type `pchain_client --help`.

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->

## Common Use Cases
- [Install and Setup](#install-and-setup)
  - [Installation](#installation)
  - [Running pchain_client](#running-pchain_client)
- [Prepare Environment](#prepare-environment)
- [Manage Account](#manage-account)
  - [Generate new keypair](#generate-new-keypair)
  - [Import existing keypair](#import-existing-keypair)
  - [List accounts](#list-accounts)
- [Transaction](#transaction)
  - [Prepare Transaction file](#prepare-transaction-file)
    - [Create new Transaction file](#create-new-transaction-file)
    - [Append Command to existing file](#append-command-to-existing-file)
  - [Submit Transaction to ParallelChain](#submit-transaction-to-parallelchain)
- [Query](#query)
  - [Check Account related information](#check-account-related-information)
  - [Get Transaction with receipt](#get-transaction-with-receipt)
  - [Get Deposit and Stake](#get-deposit-and-stake)
- [Smart Contract](#smart-contract)
  - [Retrieve contract address](#retrieve-contract-address)
  - [Prepare contract method arguments file](#prepare-contract-method-arguments-file)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## Install and Setup
### Installation
`pchain_client` is an available tool for users on Unix/Linux, MacOS, and Windows operating systems. Simply download the pre-built binary corresponding to your platform and install the `pchain_client`.

Here are the simple steps to install `pchain_client`:
  - Open a web browser and go to [release page](https://github.com/parallelchain-io/pchain-client-cli/releases).
  - Follow the link to download pre-built binary available for your platform.
  - Run the downloaded file.

**NOTE:**
If this is your first time using `pchain_client`, you need to setup `$PCHAIN_CLI_HOME` in environment variables to specify the home path. See more [here](https://chlee.co/how-to-setup-environment-variables-for-windows-mac-and-linux/).

### Running pchain_client
Upon first use of `pchain_client`, you will be prompted to set up a password to protect your account keypairs. Please note that this password can be different from the password you used in ParallelChain Explorer. Alternatively, you can skip the password protection by simply pressing Enter.

Command:
```sh
pchain_client --version
```
You will be required to enter your password twice. If your password is set successfully, you will see a return message with `pchain_client` version shown on console.

**WARNING:**
The password is not sent and saved in anywhere. You won't be able to recover the password if you lost it. Please keep your password safe. You will be required to provide this password to submit transactions and manage keypairs later.

## Prepare Environment
Before you can submit transactions or query information on ParallelChain, you need to setup your own choice of ParallelChain RPC API provider URL.

Command:
```sh
pchain_client config setup --url <URL>
```
This would check the status of your chosen provider. If `pchain_client` cannot connect to your provider, a warning message will be shown and setup is failed. You need to setup another url with the above command again.

## Manage Account
In ParallelChain, an account is identified by the public key of Ed25519 keypair. You can either generate new keys or import your existing Ed25519 keypair to make transactions in `pchain_client`. Both operations require password (if you setup before).

### Generate New Keypair
This command generates a set of ed25519_dalek compatible keys. Random name will be set if you do not provide a name.
```sh
pchain_client keys create --name <NAME>
```

### Import Existing Keypair
If you have already got keys from ParallelChain Explorer, you can import your account keypair with this command. Random name will be set if you do not provide a name.
```sh
pchain_client keys import --private <PRIVATE_KEY> --public <PUBLIC_KEY> --name <NAME>

// PRIVATE_KEY and PUBLIC_KEY are Base64url encoded Ed25519 keys.
```
### List Accounts
After creating or adding keypair, you can check it using the following command to list out all public keys managed in this tool.
```sh
pchain_client keys list
```

## Transaction 
A transaction is a digitally signed instruction that tells the ParallelChain state machine to execute a sequence of commands. There are different kinds of [Commands](https://docs.rs/pchain-types/0.4.3/pchain_types/blockchain/enum.Command.html) in ParallelChain protocol. 

`pchain_client` accepts transaction in json format. This section will demonstrate how to prepare your transaction file and submit it with your account keys.
### Prepare Transaction File
`pchain_client` provides user-friendly way to prepare your transaction file without prior knowledge of JSON (JavaScript Object Notation) format.
The transaction file consists of 2 parts: `Parameters` and `Subcommand`.

Here are some CLI subcommands to indicate corresponding [Protocol Transaction Command](https://docs.rs/pchain-types/0.4.3/pchain_types/blockchain/enum.Command.html). 

| Subcommand | Action          | Description                                           |
|------------|-----------------|-------------------------------------------------------|
| transfer   |                 | Transfer balance from transaction signer to recipient |
| deploy     |                 | Deploy smart contract to the state of the blockchain  |
| call       |                 | Trigger method call of a deployed smart contract      |
| deposit    |                 | Deposit some balance into the network account         |
|            | create          | Instantiation of a Deposit of an existing Pool        |
|            | top-up          | Increase balance of an existing Deposit               |
|            | withdraw        | Withdraw balance from an existing Deposit             |
|            | update-settings | Update settings of an existing Deposit                |
| stake      |                 | Stake to a particular pool                            |
|            | stake           | Increase stakes to an existing Pool                   |
|            | unstake         | Remove stakes from an existing Pool                   |
| pool       |                 | Create and manage Pool                                |
|            | create          | Instantiation of a Pool in the network account        |
|            | update-settings | Update settings of an existing Pool                   |
|            | delete          | Delete an existing Pool in the network account        |

#### Create New Transaction File
`Transaction` in ParallelChain protocol specifies a set of parameters included in the instruction. You don't need to provide all parameters, some of them would be computed and filled in automatically when you submit the transaction.

```sh
pchain_client transaction create --help
```

First, provide the following 4 parameters:
```sh
pchain_client transaction create \
  --nonce <NONCE> \
  --gas-limit <GAS_LIMIT> \
  --max-base-fee-per-gas <MAX_BASE_FEE_PER_GAS> \
  --priority-fee-per-gas <PRIORITY_FEE_PER_GAS> \
...
```
Then, decide the transaction type using the [CLI subcommand](#prepare-transaction-file). Each of them takes different inputs. You can always check help menu using `--help`.

Make sure you provide both `Parameters` and `Subcommand` parts in one command. The output transaction file (tx.json) will be saved in the current directory. You can also specify the designated file with the flag `--destination`

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

#### Append Command to Existing File
As explained in the beginning of [Transaction](#transaction) section, Transaction in ParallelChain protocol accepts sequence of commands. But you may find that `transaction create` in previous section only support a single Command in Transaction. 

If you want to support multiple Commands, use following command with the [subcommand](#prepare-transaction-file). This appends a `Command` element to the back of the command array in Transaction. Please note that commands in array will be executed in sequential order.

Example:
```sh
pchain_client transaction append \
  --file ~/Documents/deposit-tx.json \
  transfer \
    --recipient kRPL8cXI73DNgVSSQz9WfIi-mAAvFvdXKfZ9UPBEv_A \
    --amount 100
```

### Submit Transaction to ParallelChain
After preparing the transaction json file, you can now submit the transaction with keypair.

Command:
```sh
pchain_client transaction submit \
--file <FILE> \
--keypair-name <KEYPAIR_NAME>
```
You will get the following response if the transaction is accepted by your provider:
```json
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
`pchain_client` allows you to query different data from the ParallelChain, not just Transaction or Account related information, but also details of Validators and Stake Pool in ParallelChain network. 

Use `pchain_client query --help` to check the full list available to query.

### Check Account Related Information
To check Externally Owned Accounts (EOA) information such as balance and nonce, your account address (public key) is always needed.

Command:
```sh
pchain_client query balance --address <ADDRESS>
pchain_client query nonce --address <ADDRESS>
```

For Contract Account, you can use another command to download the contract code binary file (wasm).

Command:
```sh
pchain_client query contract --address <ADDRESS>
```

### Get Transaction with Receipt
In [Submit Transaction to ParallelChain](#submit-transaction-to-parallelchain) section, after you successfully make transaction on ParallelChain, you should receive the transaction hash (tx_hash) in the response. This hash is the identity of your transaction. You can always retrieve the transaction details with receipt by the transaction hash.

Command:
```sh
pchain_client query tx --hash <TX_HASH>
```

If you just want to get the receipt, you can use following command
```sh
pchain_client query receipt --hash <TX_HASH>
```

### Get Deposit and Stake
You can query deposit or stake amount of an account from a specific pool stored in Network Account.

Command:
```sh
pchain_client query deposit --operator <OPERATOR> --owner <OWNER>

pchain_client query stake --operator <OPERATOR> --owner <OWNER>
```

## Smart Contract 
Smart contracts are computer programs that are stored on a blockchain. You need to provide some necessary information such as contract address, method name, and arguments in order to invoke method of the contract.

### Retrieve Contract Address
After you deploy contract in a transaction, you should receive the contract address together with transaction hash. What if you want to deploy contract and call method in the SAME transaction, it is possible to compute the contract address in advance.

You need to provide the account address and nonce when deploying contract.

Command:
```sh
pchain_client parse contract-address --address <ADDRESS> --nonce <NONCE>
```

### Prepare Contract Method Arguments File
When you make a contract call that modify or view state, the contract method may expect arguments. You need to provide arguments by JSON file(.json) with `transaction create call` or `query view` commands.

Example:
For a contract method that accepts 3 arguments (String, Vec<i16> , boolean)
```json
{
    "arguments": [
        {"argument_type": "String", "argument_value": "\"Yuru Camp\""},
        {"argument_type": "Vec<i16>", "argument_value":"[-1, 20]"},
        {"argument_type": "bool", "argument_value": "true"}
    ]
}
```
Each object in arguments array consists of two fields, `argument_type` and `argument_value`.
Here are some acceptable types and values.

| Type        | Description                                   | Example value                        |
|-------------|-----------------------------------------------|--------------------------------------|
| `i8`        | The 8-bit signed integer type.                | "-128"                               |
| `i16`       | The 16-bit signed integer type.               | "-32768"                             |
| `i32`       | The 32-bit signed integer type.               | "-2147483648"                        |
| `i64`       | The 64-bit signed integer type.               | "-9223372036854775808"               |
| `u8`        | The 8-bit unsigned integer type.              | "255"                                |
| `u16`       | The 16-bit unsigned integer type.             | "65535"                              |
| `u32`       | The 32-bit unsigned integer type.             | "4294967295"                         |
| `u64`       | The 64-bit unsigned integer type.             | "18446744073709551615"               |
| `String`    | String                                        | "\\"This is test string\\""          |
| `bool`      | Boolean                                       | "true" or "false"                    |

***More complicated types can be found in "example/arguments.json"***

## Versioning

The version of this library reflects the version of the ParallelChain Protocol which it implements. For example, the current version is 0.4.3, and this implements protocol version 0.4. Patch version increases are not guaranteed to be non-breaking.

## Opening an issue

Open an issue in GitHub if you:
1. Have a feature request / feature idea,
2. Have any questions (particularly software related questions),
3. Think you may have discovered a bug.

Please try to label your issues appropriately.