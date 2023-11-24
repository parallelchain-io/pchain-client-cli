/*
    Copyright © 2023, ParallelChain Lab
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Implementation of utility methods related to parsing pchain_types::CallData.

use borsh::{BorshDeserialize, BorshSerialize};
use regex::Regex;
use serde_big_array::Array;
use serde_json::Value;
use std::{collections::VecDeque, convert::TryInto, fmt::Debug, ops::Deref};

use crate::display_msg::DisplayMsg;

/// Decodes a Base64URL string into pchain_types::cryptography::PublicAddress.
/// Throws error if decode fails.
/// # Arguments
/// * `base64url` - the string argument which is to be decoded
pub fn base64url_to_public_address(
    base64url: &str,
) -> Result<pchain_types::cryptography::PublicAddress, DisplayMsg> {
    base64url::decode(&base64url)
        .map_err(|_| DisplayMsg::IncorrectBase64urlLength)?
        .try_into()
        .map_err(|_| DisplayMsg::InvalidBase64Encoding(String::from("")))
}

/// Read from a string in json and deserialize it to call arguments.
///
/// The expected json value is in format:
/// ```json
/// {
///     "arguments": [
///         { "argument_type": xxx, "argument_value": yyy },
///         ...
///     ]
/// }
/// ```
pub fn call_arguments_from_json_value(json_val: &Value) -> Result<Vec<Vec<u8>>, DisplayMsg> {
    let json_args = parse_json_arguments(json_val)?;
    call_arguments_from_json_array(&json_args)
}

/// Read from a string in json and deserialize it to call arguments.
///
/// The expected json array is in format:
/// ```json
/// [
///     { "argument_type": xxx, "argument_value": yyy },
///     ...
/// ]
/// ```
pub fn call_arguments_from_json_array(json_array: &Vec<Value>) -> Result<Vec<Vec<u8>>, DisplayMsg> {
    let mut arguments = vec![];
    for json_arg in json_array {
        let args = parse_json_argument_type_value(json_arg)?;
        arguments.push(args);
    }
    Ok(arguments)
}

/// The expected json array is in format:
/// ```json
/// { "arguments": [ ... ] }
/// ```
pub fn parse_json_arguments(json_val: &Value) -> Result<Vec<Value>, DisplayMsg> {
    json_val["arguments"]
        .as_array()
        .map(|args| args.to_vec())
        .ok_or_else(|| DisplayMsg::MissingFieldinJson(String::from("arguments")))
}

/// The expected input is in format:
/// ```json
/// { "argument_type": xxx, "argument_value": yyy }
/// ```
fn parse_json_argument_type_value(json_arg: &Value) -> Result<Vec<u8>, DisplayMsg> {
    let j_type: &str = json_arg["argument_type"]
        .as_str()
        .ok_or_else(|| DisplayMsg::MissingFieldinJson(String::from("argument_type")))?;
    serialize_argument_value(j_type, &json_arg["argument_value"])
}

/// Parse the content in "argument_value" into borsh serialized Vec<u8>. It involves recursive calls
/// for parsing nested Custom Value.
fn serialize_argument_value(data_type: &str, value: &Value) -> Result<Vec<u8>, DisplayMsg> {
    let args = match value {
        Value::String(value_str) => serialize_primitive_argument_value(value_str, data_type)?
            .ok_or_else(|| {
                DisplayMsg::FailToParseCallArguments(format!(
                    "Fail to parse value of data type {}",
                    data_type
                ))
            })?,
        // Custom Serializable Object as an array of fields
        Value::Array(value_arr) => {
            let data_type = data_type.replace(' ', "");
            let is_custom = match data_type.as_str() {
                "Custom" => true,
                "Vec<Custom>" => false,
                _ => return Err(DisplayMsg::FailToParseCallArguments(
                    "Json array value must be with argument types either Custom, or Vec<Custom>"
                        .to_string(),
                )),
            };
            let mut args = vec![];
            for v in value_arr {
                let child_data_type = v["argument_type"]
                    .as_str()
                    .ok_or_else(|| DisplayMsg::MissingFieldinJson("argument_type".to_string()))?;
                let child_data_value =
                    serialize_argument_value(child_data_type, &v["argument_value"])?;
                args.push(child_data_value);
            }
            if is_custom {
                // Custom
                // Borsh serialization concats the serialized field in a struct
                args.concat()
            } else {
                // Vec<Custom>
                // Borsh serialization concats the serialized items in an array, with heading 4 bytes as length
                [
                    (value_arr.len() as u32).to_le_bytes().to_vec(),
                    args.concat(),
                ]
                .concat()
            }
        }
        _ => {
            return Err(DisplayMsg::FailToParseCallArguments(
                "Unknown Json Value".to_string(),
            ))
        }
    };

    Ok(args)
}

/// Serialize call arguments to bytes. Throws error if decode fails.
fn serialize_primitive_argument_value(
    value: &str,
    data_type: &str,
) -> Result<Option<Vec<u8>>, DisplayMsg> {
    let dt_no_space = sanitize_argument_type(data_type);

    macro_rules! serialize_call_args {
        ($($t:ty,)*) => {
            $(
                if dt_no_space == stringify!($t).replace(' ', "") {
                    let data: $t = match serde_json::from_str(&value){
                        Ok(d) => d,
                        Err(e) => return Err(DisplayMsg::FailToSerializeCallArgument(e.to_string())),
                    };

                    match data.try_to_vec() {
                        Ok(data) => {
                            return Ok(Some(data));
                        },
                        Err(e) => {
                            return Err(DisplayMsg::FailToSerializeCallArgument(e.to_string()));
                        }
                    }
                }
            )*
            return Ok(None)
        };
    }

    serialize_call_args!(
        i8, i16, i32, i64, i128,
        u8, u16, u32, u64, u128,
        bool, String,

        Vec<i8>, Vec<i16>, Vec<i32>, Vec<i64>, Vec<i128>,
        Vec<u8>, Vec<u16>, Vec<u32>, Vec<u64>, Vec<u128>,
        Vec<bool>, Vec<String>,

        Option<i8>, Option<i16>, Option<i32>, Option<i64>, Option<i128>,
        Option<u8>, Option<u16>, Option<u32>, Option<u64>, Option<u128>,
        Option<bool>, Option<String>,

        Vec<Vec<i8>>, Vec<Vec<i16>>, Vec<Vec<i32>>, Vec<Vec<i64>>, Vec<Vec<i128>>,
        Vec<Vec<u8>>, Vec<Vec<u16>>, Vec<Vec<u32>>, Vec<Vec<u64>>, Vec<Vec<u128>>,
        Vec<Vec<bool>>, Vec<Vec<String>>,

        Option<Vec<i8>>, Option<Vec<i16>>, Option<Vec<i32>>, Option<Vec<i64>>, Option<Vec<i128>>,
        Option<Vec<u8>>, Option<Vec<u16>>, Option<Vec<u32>>, Option<Vec<u64>>, Option<Vec<u128>>,
        Option<Vec<bool>>, Option<Vec<String>>,

        Vec<Option<i8>>, Vec<Option<i16>>, Vec<Option<i32>>, Vec<Option<i64>>, Vec<Option<i128>>,
        Vec<Option<u8>>, Vec<Option<u16>>, Vec<Option<u32>>, Vec<Option<u64>>, Vec<Option<u128>>,
        Vec<Option<bool>>, Vec<Option<String>>,

        Array<u8, 32>, Array<u8, 64>, OptionArray<u8, 32>, OptionArray<u8, 64>,
    );
}

/// Deserialize the data from a pre-defined format.
pub fn parse_call_result_from_schema(
    serialized_data: &Vec<u8>,
    schema: &Value,
) -> Result<Vec<(String, String)>, DisplayMsg> {
    struct NamedValue<'a> {
        name: String,
        idx: usize,
        value: &'a Value,
    }

    let mut data_types = Vec::new();
    let mut values = VecDeque::new();
    if let Value::Array(j_values) = &schema {
        for (idx, j_value) in j_values.iter().enumerate() {
            values.push_back(NamedValue {
                name: "".to_string(),
                idx,
                value: j_value,
            });
        }
    } else {
        values.push_back(NamedValue {
            name: "".to_string(),
            idx: 0,
            value: schema,
        });
    }

    while let Some(NamedValue { name, idx, value }) = values.pop_front() {
        let argument_name = value["argument_name"].as_str().unwrap_or("");
        let val_name = match (name.as_str(), argument_name) {
            ("", "") => format!("[{idx}]"),
            ("", val_name) => val_name.to_string(),
            (name, "") => format!("{name}[{idx}]"),
            (name, val_name) => name.to_string() + "." + val_name,
        };

        match &value["argument_type"] {
            Value::String(j_type) => {
                data_types.push((val_name, j_type.to_string()));
            }
            Value::Array(j_val_array) => {
                for (j_val_idx, j_val) in j_val_array.iter().enumerate() {
                    values.push_back(NamedValue {
                        name: val_name.clone(),
                        idx: j_val_idx,
                        value: j_val,
                    });
                }
            }
            _ => return Err(DisplayMsg::FailToParseCallArguments("".to_string())),
        }
    }

    let mut result = Vec::new();
    let serialized_data = serialized_data.as_slice();
    let mut pos = 0;
    for (name, data_type) in data_types {
        if let Some(deserialized) =
            deserialize_primitive_argument_value(&serialized_data[pos..], &mut pos, &data_type)?
        {
            result.push((name, deserialized));
        }
    }

    Ok(result)
}

pub fn parse_call_result_from_data_type(
    vec: &Vec<u8>,
    data_type: String,
) -> Result<String, DisplayMsg> {
    let buf = vec.as_slice();
    let mut pos = 0;
    match deserialize_primitive_argument_value(buf, &mut pos, &data_type) {
        Ok(Some(result)) => Ok(result),
        Ok(None) => Err(DisplayMsg::FailToSerializeCallArgument(data_type)),
        Err(e) => Err(DisplayMsg::FailToSerializeCallArgument(e.to_string())),
    }
}

/// Serialize call arguments from bytes. Throws error if decode fails.
fn deserialize_primitive_argument_value(
    buf: &[u8],
    pos: &mut usize,
    data_type: &str,
) -> Result<Option<String>, DisplayMsg> {
    let dt_no_space = sanitize_argument_type(data_type);

    macro_rules! deserialize_call_args {
        ($($t:ty,)*) => {
            $(
                if dt_no_space == stringify!($t).replace(' ', "") {
                    let data: $t = deserialize_from_buf(buf, pos)?;
                    return Ok(Some(format!("{:?}", data)));
                }
            )*
        }
    }

    deserialize_call_args!(
        
        i8, i16, i32, i64, i128,
        u8, u16, u32, u64, u128,
        bool, String,

        Vec<i8>, Vec<i16>, Vec<i32>, Vec<i64>, Vec<i128>,
        Vec<u8>, Vec<u16>, Vec<u32>, Vec<u64>, Vec<u128>,
        Vec<bool>, Vec<String>,

        Option<i8>, Option<i16>, Option<i32>, Option<i64>, Option<i128>,
        Option<u8>, Option<u16>, Option<u32>, Option<u64>, Option<u128>,
        Option<bool>, Option<String>,

        Vec<Option<i8>>, Vec<Option<i16>>, Vec<Option<i32>>, Vec<Option<i64>>, Vec<Option<i128>>,
        Vec<Option<u8>>, Vec<Option<u16>>, Vec<Option<u32>>, Vec<Option<u64>>, Vec<Option<u128>>,
        Vec<Option<bool>>, Vec<Option<String>>,
    );

    macro_rules! deserialize_call_args_explicitly {
        ($($t:ty => $q:ty,)*) => {
            $(
                if dt_no_space == stringify!($t).replace(' ', "") {
                    let data: $q = deserialize_from_buf(buf, pos)?;
                    return Ok(Some(format!("{:?}", data)));
                }
            )*
        }
    }

    deserialize_call_args_explicitly!(
        Array<u8, 32> => [u8; 32],
        Array<u8, 64> => [u8; 64],
        OptionArray<u8, 32> => Option<[u8; 32]>,
        OptionArray<u8, 64> => Option<[u8; 64]>,
    );

    Ok(None)
}

/// Deserialized from a buffer. Update the deserialized length to the variable `pos`.
fn deserialize_from_buf<T: BorshDeserialize>(buf: &[u8], pos: &mut usize) -> Result<T, DisplayMsg> {
    let mut temp = buf;
    let org_len = temp.len();
    let deserialized = T::deserialize(&mut temp)
        .map_err(|e| DisplayMsg::FailToSerializeCallArgument(e.to_string()))?;
    let new_len = temp.len();
    *pos += org_len - new_len;
    Ok(deserialized)
}

/// Convert the argument type in string to a valid format
fn sanitize_argument_type(data_type: &str) -> String {
    let mut dt_no_space: String = data_type.replace(' ', "");

    // if input type string is a slice of number type with length 32 or 64
    let re_option = Regex::new(r"^(Option<)?\[[ui](8|16|32|64|128);(32|64)](>)?$").unwrap();
    if re_option.is_match(&dt_no_space) {
        if dt_no_space.starts_with('O') {
            //turn option slice into option of serde json big array type
            dt_no_space = dt_no_space
                .replace("Option<", "Option")
                .replace('[', "Array<")
                .replace(';', ",")
                .replace("]>", ">");
        } else {
            // turn slice into serde json big array type
            dt_no_space = dt_no_space
                .replace('[', "Array<")
                .replace(';', ",")
                .replace(']', ">");
        }
    }

    dt_no_space
}

/// [OptionArray] wraps Option type of serde json big array
/// to make it serde serializable/deserializable
/// which make Option type of byte slice as argument type acceptable
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
pub struct OptionArray<T, const N: usize>(pub Option<Array<T, N>>);

impl<'de, T: serde::de::Deserialize<'de>, const N: usize> serde::de::Deserialize<'de>
    for OptionArray<T, N>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let option_array: Option<Array<T, N>> =
            <Option<_> as serde::de::Deserialize>::deserialize(deserializer)?;
        Ok(OptionArray(option_array))
    }
}

impl<T: serde::ser::Serialize, const N: usize> serde::ser::Serialize for OptionArray<T, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self.0.as_ref() {
            Some(inner) => {
                <[T; N] as serde_big_array::BigArray<T>>::serialize(&inner.0, serializer)
            }
            None => serializer.serialize_none(),
        }
    }
}

impl<T, const N: usize> Deref for OptionArray<T, N> {
    type Target = Option<[T; N]>;

    fn deref(&self) -> &Self::Target {
        unsafe {
            // Safety: ensure the size and alignment of `BigArray<T, N>` and `[T; N]` are the same
            std::mem::transmute(&self.0)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{parse_call_result_from_schema, serialize_primitive_argument_value};
    use borsh::{BorshDeserialize, BorshSerialize};
    use serde_json::Value;

    #[test]
    fn test_serialize_primitive_argument_value() {
        match serialize_primitive_argument_value("[[[true]]]", "Vec<Vec<Vec<bool>>>") {
            Ok(None) => {}
            _ => panic!("Expect error for unsupported type"),
        }

        assert_eq!(
            serialize_primitive_argument_value("false", "bool")
                .unwrap()
                .unwrap(),
            { false.try_to_vec().unwrap() }
        );

        assert_eq!(
            serialize_primitive_argument_value("[false]", "Vec<bool>")
                .unwrap()
                .unwrap(),
            { vec![false].try_to_vec().unwrap() }
        );

        assert_eq!(
            serialize_primitive_argument_value("[-2, -3, 5, 6]", "Vec< i32 >")
                .unwrap()
                .unwrap(),
            vec![-2i32, -3, 5, 6].try_to_vec().unwrap()
        );

        assert_eq!(
            serialize_primitive_argument_value(
                "[2, 2, 2, 2, 2, 2, 2, 2, 
                2, 2, 2, 2, 2, 2, 2, 2, 
                2, 2, 2, 2, 2, 2, 2, 2, 
                2, 2, 2, 2, 2, 2, 2, 2, 
                2, 2, 2, 2, 2, 2, 2, 2, 
                2, 2, 2, 2, 2, 2, 2, 2, 
                2, 2, 2, 2, 2, 2, 2, 2, 
                2, 2, 2, 2, 2, 2, 2, 2]",
                "[ u8 ; 64 ]"
            )
            .unwrap()
            .unwrap(),
            [
                2u8, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2, 2, 2, 2, 2
            ]
            .try_to_vec()
            .unwrap()
        );

        assert_eq!(
            serialize_primitive_argument_value(
                "[1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2]",
                "Option<[ u8 ; 32 ]>"
            )
            .unwrap()
            .unwrap(),
            Some([
                1u8, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7,
                8, 9, 0, 1, 2,
            ])
            .try_to_vec()
            .unwrap()
        );

        assert_eq!(
            serialize_primitive_argument_value("null", "Option<[ u8 ; 64 ]>")
                .unwrap()
                .unwrap(),
            {
                let v: Option<[u8; 64]> = None;
                v.try_to_vec().unwrap()
            }
        );

        assert_eq!(
            serialize_primitive_argument_value("\"Fa\\\"l\\\"se\"", "String")
                .unwrap()
                .unwrap(),
            {
                let v = r#"Fa"l"se"#;
                v.try_to_vec().unwrap()
            }
        );

        assert_eq!(
            serialize_primitive_argument_value("\"Fa\\\"l\\\"se\"", "String")
                .unwrap()
                .unwrap(),
            {
                let v = "Fa\"l\"se";
                v.try_to_vec().unwrap()
            }
        );

        assert_eq!(
            serialize_primitive_argument_value("[\"sss\", \"ddd\"]", "Vec< String >")
                .unwrap()
                .unwrap(),
            vec![String::from("sss"), String::from("ddd")]
                .try_to_vec()
                .unwrap()
        );

        assert_eq!(
            serialize_primitive_argument_value("[\"sss\", \"ddd\"]", "Vec< Option < String > >")
                .unwrap()
                .unwrap(),
            vec![Some(String::from("sss")), Some(String::from("ddd"))]
                .try_to_vec()
                .unwrap()
        );

        assert_eq!(
            serialize_primitive_argument_value("[\"sss\", null]", "Vec< Option < String > >")
                .unwrap()
                .unwrap(),
            vec![Some(String::from("sss")), None].try_to_vec().unwrap()
        );

        assert_eq!(
            serialize_primitive_argument_value("[\"sss\", \"ddd\"]", "Option< Vec < String > >")
                .unwrap()
                .unwrap(),
            Some(vec![String::from("sss"), String::from("ddd")])
                .try_to_vec()
                .unwrap()
        );

        assert_eq!(
            serialize_primitive_argument_value("null", "Option< Vec < String > >")
                .unwrap()
                .unwrap(),
            {
                let v: Option<Vec<String>> = None;
                v.try_to_vec().unwrap()
            }
        );

        assert_eq!(
            serialize_primitive_argument_value("[[\"sss\"], [\"ddd\"]]", "Vec< Vec < String > >")
                .unwrap()
                .unwrap(),
            vec![vec![String::from("sss")], vec![String::from("ddd")]]
                .try_to_vec()
                .unwrap()
        );
    }

    #[test]
    fn test_parse_arguments() {
        let json_val: Value = serde_json::from_str(r#" 
            {
                "arguments": [
                    {"argument_type": "i8", "argument_value":"-1"},
                    {"argument_type": "Vec<i8>", "argument_value":"[-1, -3, -5]"},
                    {"argument_type": "Vec<Vec<i8>>", "argument_value":"[[-1, -3], [-5]]"},
                    {"argument_type": "Vec<Option<i8>>", "argument_value":"[-1, null, -5]"},
                    {"argument_type": "Option<Vec<i8>>", "argument_value":"[-1, -3, -5]"},
                    {"argument_type": "Option<Vec<i8>>", "argument_value":"null"},
            
                    {"argument_type": "i16", "argument_value":"-30000"},
                    {"argument_type": "Vec<i16>", "argument_value":"[-10001, -30001, -5001]"},
                    {"argument_type": "Vec<Vec<i16>>", "argument_value":"[[-10001, -30001], [-5001]]"},
                    {"argument_type": "Vec<Option<i16>>", "argument_value":"[-10001, null, -5001]"},
                    {"argument_type": "Option<Vec<i16>>", "argument_value":"[-10001, -30001, -5001]"},
                    {"argument_type": "Option<Vec<i16>>", "argument_value":"null"},
                    
                    {"argument_type": "i32", "argument_value":"-1094967295"},
                    {"argument_type": "Vec<i32>", "argument_value":"[-1,0,1]"},
            
                    {"argument_type": "i64", "argument_value":"-9046744073709551615"},
                    {"argument_type": "Vec<i64>", "argument_value":"[-1,0,1,65656565]"},
            
                    {"argument_type": "i128", "argument_value":"-9046744073709551615"},
                    {"argument_type": "Vec<i128>", "argument_value":"[-1,0,1,-65656565,0]"},
            
                    {"argument_type": "u8", "argument_value":"255"},
                    {"argument_type": "Vec<u8>", "argument_value":"[0]"},
            
                    {"argument_type": "u16", "argument_value":"65535"},
                    {"argument_type": "Vec<u16>", "argument_value":"[65535,6535]"},
            
                    {"argument_type": "u32", "argument_value":"4294967295"},
                    {"argument_type": "Vec<u32>", "argument_value":"[65535,6535,1919]"},
            
                    {"argument_type": "u64", "argument_value":"18446744073709551615"},
                    {"argument_type": "Vec<u64>", "argument_value":"[65535,6535,1919112123223]"},
            
                    {"argument_type": "u128", "argument_value":"18446744073709551616"},
                    {"argument_type": "Vec<u128>", "argument_value":"[65535,6535,1919112123223,123123,124124,125152]"},
            
                    {"argument_type": "bool", "argument_value": "true"},
                    {"argument_type": "Vec<bool>", "argument_value": "[true,false,true]"},
                    {"argument_type": "Vec<Vec<bool>>", "argument_value":"[[true, false], [true]]"},
                    {"argument_type": "Vec<Option<bool>>", "argument_value":"[true,false,null]"},
                    {"argument_type": "Option<Vec<bool>>", "argument_value":"[true,false,true]"},
                    {"argument_type": "Option<Vec<bool>>", "argument_value":"null"},
            
                    {"argument_type": "String", "argument_value": "\"string data\""},
                    {"argument_type": "Vec<String>", "argument_value": "[\"string data\", \"asdaf\", \"1d1 as2\"]"},
                    {"argument_type": "Vec<Vec<String>>", "argument_value":"[[\"string data\"], [\"asdaf\", \"1d1 as2\"]]"},
                    {"argument_type": "Vec<Option<String>>", "argument_value":"[\"string data\", null, \"1d1 as2\"]"},
                    {"argument_type": "Option<Vec<String>>", "argument_value":"[\"string data\", \"asdaf\", \"1d1 as2\"]"},
                    {"argument_type": "Option<Vec<String>>", "argument_value":"null"},

                    {"argument_type": "[u8; 32]", "argument_value": "[1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2]"},
                    {"argument_type": "[u8; 64]", "argument_value": "[1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2]"},
                    {"argument_type": "Option<[u8; 32]>", "argument_value": "[1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2]"},
                    {"argument_type": "Option<[u8; 64]>", "argument_value": "null"},

                    {"argument_type": "Vec<Custom>", "argument_value":
                        [
                            {"argument_type": "Custom", "argument_value":
                                [
                                    {"argument_type": "bool", "argument_value":"true"},
                                    {"argument_type": "String", "argument_value":"\"my name\""}
                                ]
                            }
                        ]
                    }
                ]
            }
            
        "#).unwrap();

        let result = crate::parser::call_arguments_from_json_value(&json_val);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.len(), 45);

        // Check in details:
        // - result[44] is "Vec<Custom>"
        assert_eq!(result[44].len(), 16); // 4 (length) + 1 (bool) + 4+7 (String)
    }

    #[test]
    fn test_parse_call_result() {
        println!("{}", base64url::encode([1u8]));

        #[derive(BorshSerialize, BorshDeserialize)]
        struct Account {
            name: String,
            balance: u64,
            friends: Friends,
        }
        #[derive(BorshSerialize, BorshDeserialize)]
        struct Friends {
            names: Vec<String>,
        }
        let serialized = Account {
            name: "Tom".to_string(),
            balance: 100,
            friends: Friends {
                names: vec!["Jason".to_string(), "Kay".to_string()],
            },
        }
        .try_to_vec()
        .unwrap();

        let result = parse_call_result_from_schema(
            &serialized,
            &serde_json::json!(
            {
                "argument_name": "Person",
                "argument_type": [
                    {"argument_name": "name", "argument_type":"String"},
                    {"argument_name": "balance", "argument_type":"u64"},
                    {"argument_name": "friends", "argument_type": [
                        {"argument_type":"Vec<String>"}
                    ]}
                ]
            }
            ),
        )
        .unwrap();

        assert_eq!(
            result
                .iter()
                .map(|(n, v)| (n.as_str(), v.as_str()))
                .collect::<Vec<(&str, &str)>>(),
            vec![
                ("Person.name", "\"Tom\""),
                ("Person.balance", "100"),
                ("Person.friends[0]", "[\"Jason\", \"Kay\"]"),
            ]
        );

        #[derive(BorshSerialize, BorshDeserialize)]
        struct Token {
            name: String,
            symbol: String,
            decimals: u8,
            total_supply: u64,
        }
        let serialized = Token {
            name: "my token".to_string(),
            symbol: "MTK".to_string(),
            decimals: 10,
            total_supply: 100000000000,
        }
        .try_to_vec()
        .unwrap();

        let result = parse_call_result_from_schema(
            &serialized,
            &serde_json::json!(
            [
                {"argument_name": "name", "argument_type":"String"},
                {"argument_name": "symbol", "argument_type":"String"},
                {"argument_name": "decimals", "argument_type":"u8"},
                {"argument_name": "total_supply", "argument_type":"u64"},
            ]
            ),
        )
        .unwrap();

        assert_eq!(
            result
                .iter()
                .map(|(n, v)| (n.as_str(), v.as_str()))
                .collect::<Vec<(&str, &str)>>(),
            vec![
                ("name", "\"my token\""),
                ("symbol", "\"MTK\""),
                ("decimals", "10"),
                ("total_supply", "100000000000"),
            ]
        );
    }

    #[test]
    fn test_callresult() {
        macro_rules! assert_data_types {
            ($($t:expr, $v:expr, $e:expr,)*) => {
                $(
                    let value = $v.try_to_vec().unwrap();

                    assert_eq!(
                        super::parse_call_result_from_data_type(&value, $t.to_string()).unwrap(),
                        $e
                    );
                )*
            }
        }

        let test_data_32: [u8; 32] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
            0, 1, 2,
        ];
        let test_data_64: [u8; 64] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
            0, 1, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6,
            7, 8, 9, 0, 1, 2,
        ];

        assert_data_types!(
            "u8",
            1u8,
            "1".to_string(),
            "u16",
            123u16,
            "123".to_string(),
            "u32",
            9898u32,
            "9898".to_string(),
            "u64",
            9999999999999u64,
            "9999999999999".to_string(),
            "u128",
            111199999999999u128,
            "111199999999999".to_string(),
            "i8",
            -1i8,
            "-1".to_string(),
            "i16",
            -123i16,
            "-123".to_string(),
            "i32",
            -9898i32,
            "-9898".to_string(),
            "i64",
            -9999999999999i64,
            "-9999999999999".to_string(),
            "i128",
            -111199999999999i128,
            "-111199999999999".to_string(),
            "bool",
            false,
            "false".to_string(),
            "String",
            "asdas".to_string(),
            "\"asdas\"".to_string(),
            "Vec<u8>",
            [0u8, 1u8, 2u8].to_vec(),
            "[0, 1, 2]".to_string(),
            "Vec<u16>",
            [99u16].to_vec(),
            "[99]".to_string(),
            "Vec<u32>",
            [0u32, 6u32].to_vec(),
            "[0, 6]".to_string(),
            "Vec<u64>",
            [0u64, 6123123123u64].to_vec(),
            "[0, 6123123123]".to_string(),
            "Vec<i8>",
            [0i8, 1i8, -2i8].to_vec(),
            "[0, 1, -2]".to_string(),
            "Vec<i16>",
            [-99i16].to_vec(),
            "[-99]".to_string(),
            "Vec<i32>",
            [0i32, 6i32].to_vec(),
            "[0, 6]".to_string(),
            "Vec<i64>",
            [-1i64, -6123123123i64].to_vec(),
            "[-1, -6123123123]".to_string(),
            "Vec<i128>",
            [-1i128, -1i128, -1i128, -1i128, -6123123123i128].to_vec(),
            "[-1, -1, -1, -1, -6123123123]".to_string(),
            "Vec<bool>",
            [true, false, false].to_vec(),
            "[true, false, false]".to_string(),
            "Vec<String>",
            ["true", "false", "false"].to_vec(),
            "[\"true\", \"false\", \"false\"]".to_string(),
            "[ u 8; 3 2]",
            test_data_32,
            format!("{:?}", test_data_32),
            "[ u 8; 6 4]",
            test_data_64,
            format!("{:?}", test_data_64),
        );
    }
}
