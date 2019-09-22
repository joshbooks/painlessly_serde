use semver::Version;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PainlessSchema {
    pub painless_schema_version: Version,
    pub painless_schema_name: String,
    pub painless_structs: Box<[PainlessStruct]>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PainlessStruct {
    pub variables: Box<[PainlessVariable]>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PainlessVariable {
    pub name: String,
    pub painless: PainlessValue
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PainlessValue {
    PainlessString { string_value: Box<str> },
    PainlessInteger { sign: bool, width: IntWidth, int_value: u64 },
    PainlessBoolean { bool_value: bool },
    PainlessList { elem_type: Box<PainlessValue>, size: u32, list_value: Box<[PainlessValue]> },
    PainlessMap { key_value_list: Box<[(Box<str>, PainlessValue)]> },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum IntWidth {
    Eight,
    Sixteen,
    ThirtyTwo,
    SixtyFour
}

