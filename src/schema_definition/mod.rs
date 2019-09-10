use semver::Version;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PainlessSchema {
    painless_schema_version: Version,
    painless_structs: Box<[PainlessStruct]>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PainlessStruct {
    variables: Box<[PainlessVariable]>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PainlessVariable {
    name: String,
    painless_type: PainlessType
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PainlessType {
    PainlessString,
    PainlessInteger { sign: bool, width: IntWidth },
    PainlessBoolean,
    PainlessList { elem_type: Box<PainlessType>, size: u32 },
    PainlessMap { key_type: Box<PainlessType>, value_type: Box<PainlessType> },
    PainlessObject // a PainlessObject is just like a PainlessMap, but without the type constraints
}

#[derive(Serialize, Deserialize, Debug)]
pub enum IntWidth {
    Eight,
    Sixteen,
    ThirtyTwo,
    SixtyFour
}

