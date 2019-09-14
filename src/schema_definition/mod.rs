use semver::Version;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PainlessSchema {
    painless_schema_version: Version,
    painless_structs: Box<[PainlessStruct]>
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
    PainlessMap { keyValueList: Box<[(Box<str>, PainlessValue)]> },
    PainlessObject {fields: Box<[PainlessVariable]>} // a PainlessObject is just like a PainlessMap, but without the type constraints
}

#[derive(Serialize, Deserialize, Debug)]
pub enum IntWidth {
    Eight,
    Sixteen,
    ThirtyTwo,
    SixtyFour
}

