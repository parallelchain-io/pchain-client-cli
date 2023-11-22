use std::{fmt, path::PathBuf};
use pchain_types::rpc::SubmitTransactionErrorV1;

use crate::command::{Base64Address, Base64Hash, Base64String};

pub type IdentityName = String;
pub type FileName = String;
pub type CLIArgs = String;
pub type ErrorMsg = String;
pub type URL = String;

#[derive(Debug)]
pub enum DisplayMsg {
    ///////////////////////////
    // Argument Decode Error //
    ///////////////////////////
    FailToDecodeBase64Address(IdentityName,  Base64Address, ErrorMsg),
    FailToDecodeBase64Hash(IdentityName,  Base64Hash, ErrorMsg),
    FailToDecodeBase64String(IdentityName,  Base64String, ErrorMsg),

    //////////////////////////////
    // File Encode/Decode Error //
    //////////////////////////////
    FailToUTF8DecodeFile(FileName, PathBuf, ErrorMsg),
    InvalidTOMLFormat(FileName, PathBuf, ErrorMsg),
    FailToTOMLEncode(FileName, PathBuf, ErrorMsg),
    FailToEncodeJson(FileName, PathBuf, ErrorMsg),
    FailToDecodeJson(FileName, PathBuf, ErrorMsg),

    ////////////////////////
    // Cli argument error //
    ///////////////////////
    IncorrectCombinationOfIdentifiers(CLIArgs),
    IncorrectFormatForSuppliedArgument(ErrorMsg),

    ////////////////
    // Query Msg //
    ///////////////
    CannotFindLatestBlock,
    CannotFindRelevantBlock,
    CannotFindRelevantBlockHeader,
    CannotFindRelevantTransaction,
    CannotFindRelevantReceipt,
    CannotFindRelevantState,
    CannotFindOperator,
    CannotFindOperatorOwnerPair,
    CannotFindValidatorSet,
    CannotFindRelevantContractCode,

    /////////////////////
    // Transaction Msg //
    /////////////////////
    SuccessSubmitTx,
    FailSubmitTx(SubmitTransactionErrorV1),
    FailToParseCallArguments(ErrorMsg),
    FailToParseCallResult(ErrorMsg),
    InvalidTxCommand(ErrorMsg),

    ////////////////
    // Config Msg //
    ////////////////
    PChainCliHomeNotSet(URL),
    InavtiveRPCProvider(URL),
    ActiveRPCProvider(URL),
    ListRPCProvider(URL),
    NotYetSetRPCProvider,

    /////////////////
    // keypair msg //
    /////////////////
    SuccessCreateKey(IdentityName, Base64Address),
    SuccessAddKey(IdentityName),
    KeypairAlreadyExists(IdentityName),
    KeypairNotFound(IdentityName),
    InvalidEd25519Keypair(ErrorMsg),
    FailToSignMessage(ErrorMsg),
    ParseKeypairFailure(serde_json::Error),

    /////////////////
    // File IO Msg //
    /////////////////
    FailToOpenOrReadFile(FileName, PathBuf, ErrorMsg),
    FailToWriteFile(FileName, PathBuf, ErrorMsg),
    FailToCreateDir(IdentityName, PathBuf, ErrorMsg),
    FailToCreateFile(FileName, PathBuf, ErrorMsg),
    IncorrectFilePath(FileName, PathBuf, ErrorMsg),
    SuccessCreateFile(FileName, PathBuf),
    SuccessUpdateFile(FileName, PathBuf),

    ////////////////////
    // HTTP Error Msg //
    ////////////////////
    RespnoseWithHTTPError(ErrorMsg),

    //////////////////
    // Password Msg //
    //////////////////
    WrongPassword,
    PasswordFilesContaminated,
    SuccessSetupPassword,
    PasswordNotMatch,
    FailToSetupPassword(ErrorMsg),
    FailtoEncrypt(ErrorMsg),
    FailtoDecrypt(ErrorMsg),

    //////////////////
    /// Parser Msg  //
    //////////////////
    InvalidJson(serde_json::Error),
    MissingFieldinJson(ErrorMsg),
    FailToBase64DecodeKeypair,
    FailToConvertReturnDataToTargetType(ErrorMsg),
    FailToSerializeCallArgument(ErrorMsg),
    InvalidBase64Encoding(IdentityName),
    IncorrectBase64urlLength,
}

impl fmt::Display for DisplayMsg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ///////////////////////////
            // Argument Decode Error //
            ///////////////////////////
            DisplayMsg::FailToDecodeBase64Address(identity, address, error) =>
                write!(f, "Error: Fail to decode \"{identity}\" address \"{address}\" from a Base64URL string. {error}"),
            DisplayMsg::FailToDecodeBase64Hash(identity, hash, error) =>
                write!(f, "Error: Fail to decode \"{identity}\" hash \"{hash}\" from a Base64URL string. {error}"),
            DisplayMsg::FailToDecodeBase64String(identity, base64_string, error) =>
                write!(f, "Error: Fail to decode \"{identity}\" \"{base64_string}\" from a Base64URL string. {error}"),

            //////////////////////////////
            // File Encode/Decode Error //
            //////////////////////////////
            DisplayMsg::FailToUTF8DecodeFile(file_name, path, error) => 
                write!(f, "Error: Provided {file_name} file at {:?} is not utf8 decodable. {error}", path),
            DisplayMsg::InvalidTOMLFormat(file_name, path, error) => 
                write!(f, "Error: Provided {file_name} file at {:?} is not correct toml format. {error}", path),
            DisplayMsg::FailToTOMLEncode(file_name, path, error) => 
                write!(f, "Error: Cannot encode {file_name} file at {:?} to toml format. {error}", path),
            DisplayMsg::FailToEncodeJson(file_name, path, error) => 
                write!(f, "Error: Cannot encode {file_name} file at {:?} to json format. {error}", path),
            DisplayMsg::FailToDecodeJson(file_name, path, error) => 
                write!(f, "Error: Cannot decode provided {file_name} json file at {:?} to desired shape. {error}", path),

            ////////////////////////
            // Cli argument error //
            ///////////////////////
            DisplayMsg::IncorrectCombinationOfIdentifiers(identifiers) => 
                write!(f, "Error: Invalid combination of input. Please specify a correct identifier (\"{}\" ).", identifiers),
            DisplayMsg::IncorrectFormatForSuppliedArgument(error) => 
            write!(f, "Error: Supplied Argument is of incorrect format. It should be in form of (\"{}\" ).", error),

            ////////////////
            // Query Msg //
            ///////////////
            DisplayMsg::CannotFindLatestBlock => 
                write!(f, "Error: Cannot find find latest block."),
            DisplayMsg::CannotFindRelevantBlock =>
                write!(f, "Cannot find relevant block."),
            DisplayMsg::CannotFindRelevantBlockHeader =>
                write!(f, "Cannot find relevant block header"),
            DisplayMsg::CannotFindRelevantTransaction => 
                write!(f, "Cannot find relevant transaction."),
            DisplayMsg::CannotFindRelevantReceipt => 
                write!(f, "Cannot find relevant receipt."),
            DisplayMsg::CannotFindRelevantState => 
                write!(f, "Cannot find relevant state."),
            DisplayMsg::CannotFindOperator => 
                write!(f, "Cannot find relevant operator."),
            DisplayMsg::CannotFindOperatorOwnerPair => 
                write!(f, "Cannot find relevant operator owner pair."),
            DisplayMsg::CannotFindValidatorSet =>
                write!(f, "No validator set exists at the requested time frame."),
            DisplayMsg::CannotFindRelevantContractCode =>
                write!(f, "No contract code is associated with this address."),

            /////////////////////
            // Transaction Msg //
            /////////////////////
            DisplayMsg::SuccessSubmitTx =>
                write!(f, "Transaction is submitted to ParallelChain but not completely get through yet. Check explorer or wallet for updated status."),
            DisplayMsg::FailSubmitTx(error) => {
                match error {
                    SubmitTransactionErrorV1::MempoolFull => write!(f, "Error: Submit Transation Fail. Mempool is full."),
                    SubmitTransactionErrorV1::UnacceptableNonce => write!(f, "Error: Submit Transation Fail. Nonce is not within acceptable range."),
                    SubmitTransactionErrorV1::Other => write!(f, "Error: Submit Transation Fail. Please ensure gas limit, base fee or transaction size is within range."),
                }
            },
            DisplayMsg::FailToParseCallArguments(e) =>
                write!(f, "Error: Cannot parse contract call arguments of the transaction. {}", e),
            DisplayMsg::FailToParseCallResult(e) =>
                write!(f, "Error: Cannot parse call result. {}", e),
            DisplayMsg::InvalidTxCommand(error) =>
                write!(f, "Error: Invalid transaction command. {}", error),

            ////////////////
            // Config Msg //
            ////////////////
            DisplayMsg::PChainCliHomeNotSet(home) => 
                write!(f, "enviroment variable ${home} isn't set. Please specify the home folder of ParallelChain Client CLI"),
            DisplayMsg::InavtiveRPCProvider(url) => 
                write!(f, "Warning: The chosen provider <{}> is currently not active. Please switch to another active provider by `setup` command.", url),
            DisplayMsg::ActiveRPCProvider(url) => 
                write!(f, "Provider <{url}> is Active"),
            DisplayMsg::ListRPCProvider(url) => 
                write!(f, "Fullnode RPC Provider is <{url}>"),
            DisplayMsg::NotYetSetRPCProvider =>
                write!(f, "Warning: Fullnode RPC url is not setup. \nPlease use command `./pchain_client config setup --url <URL>` to specify the node to connect."),

            /////////////////
            // keypair msg //
            /////////////////
            DisplayMsg::SuccessCreateKey(keypair_name, pk) =>
                write!(f, "Successfully create {keypair_name} with public key <{pk}>" ),
            DisplayMsg::SuccessAddKey(keypair_name) =>
                write!(f, "Successfully add keypair with name {keypair_name}." ),
            DisplayMsg::KeypairAlreadyExists(keypair_name) =>
                write!(f, "Error: Keypair with name {keypair_name} already exists."), 
            DisplayMsg::KeypairNotFound(keypair_name) =>
                write!(f, "Error: Keypair name {keypair_name} provided does not exist. Please generate a keypair by `./pchain_client keys create --name <NAME>`"),
            DisplayMsg::InvalidEd25519Keypair(error) =>
                write!(f, "Error: Invalid Ed25519 keypair. {error}"),
            DisplayMsg::ParseKeypairFailure(serde_json::Error{ .. }) => 
                write!(f, "Error: keypair json is corrupted. Please backup the keypair.json and use command) 
            `./pchain_client keys add --private-key <PRIVATE_KEY> --public-key <PUBLIC_KEY> --name <NAME>` to re-import your keys"),
            DisplayMsg::FailToSignMessage(error) => 
                write!(f, "Fail to sign message by provided keypair. {error}"),
       
                
            /////////////////
            // File IO Msg //
            /////////////////
            DisplayMsg::IncorrectFilePath(file_name, path, error) => 
                write!(f, "Error: Invalid path. Cannot retrieve designated {file_name} file from the designated path at <{:?}>. {:#?}", path, error),
            DisplayMsg::FailToOpenOrReadFile(file_name, path, error) => 
                write!(f, "Error: Failed to read {file_name} file at <{:?}> although file is found. {:#?}", path, error),
            DisplayMsg::FailToWriteFile(file_name, path, error) => 
                write!(f, "Error: Failed to write {file_name} file at <{:?}> although file is found. {:#?}", path, error),
            DisplayMsg::FailToCreateDir(file_name, path, error) => 
                write!(f, "Error: Fail to create necessary directory for {file_name} file at <{:?}>. {:#?}", path, error),
            DisplayMsg::FailToCreateFile(file_name, path, error) => 
                write!(f, "Error: Fail to create {file_name} file at <{:?}>. {:#?}", path, error),
            DisplayMsg::SuccessCreateFile(file_name, path) => 
                write!(f, "Successfully create {file_name} file at <{:?}>.", path),
            DisplayMsg::SuccessUpdateFile(file_name, path) => 
                write!(f, "Successfully update {file_name} file at <{:?}>.", path),

            ////////////////////
            // HTTP Error Msg //
            ////////////////////
            DisplayMsg::RespnoseWithHTTPError(error) => 
                write!(f, "{error}"),


            //////////////////
            // Password Msg //
            //////////////////
            DisplayMsg::WrongPassword =>
                write!(f, "Wrong password. Fail to login."),
            DisplayMsg::PasswordFilesContaminated =>
                write!(f, "Irrecoverable error. Password files contaminted."),
            DisplayMsg::PasswordNotMatch =>
                write!(f, "Password not match"),
            DisplayMsg::SuccessSetupPassword =>
                write!(f, "Password is set. Please keep your password safe. You will require to provide this password to submit transaction and manage keypairs later."),
            DisplayMsg::FailToSetupPassword(error) =>
                write!(f, "Fail to setup your password. {:#?}", error),
            DisplayMsg::FailtoEncrypt(error) =>
                write!(f, "Fail to encrypt data. {:#?}", error),
            DisplayMsg::FailtoDecrypt(error) =>
                write!(f, "Fail to decrypt data. {:#?}", error),

            /////////////////
            // Parser Msg  //
            /////////////////
            DisplayMsg::InvalidJson(e) => 
                write!(f, "Provided json is not valid. {e}"),
            DisplayMsg::MissingFieldinJson(field_name) => 
                write!(f, "Provided json does not contain field with name : {field_name}"),
            DisplayMsg::FailToBase64DecodeKeypair => 
                write!(f, "Fail to base64 decode keypair."),
            DisplayMsg::FailToConvertReturnDataToTargetType(e) => 
                write!(f, "Fail to convert to target data type. {e}"),
            DisplayMsg::FailToSerializeCallArgument(e) =>
                write!(f, "Fail to serialize call argument. {e}"),
            DisplayMsg::InvalidBase64Encoding(identity) => 
                write!(f, "Provided {identity} has invalid base64 encoding"),
            DisplayMsg::IncorrectBase64urlLength => 
                write!(f, "Incorrect length. Correct length should be 32 bytes long."),
            
        }
    }    
}



