use semver::Version;

struct PainlessSchema {
    painless_schema_version: Version,
    painless_structs: Box<[PainlessStruct]>
}

struct PainlessStruct {
    variables: Box<[PainlessVariable]>
}

struct PainlessVariable {
    name: String,
    painless_type: PainlessType
}

enum PainlessType {
    PainlessString,
    PainlessInteger { sign: bool, width: IntWidth },
    PainlessBoolean,
    PainlessList { elem_type: Box<PainlessType>, size: u32 },
    PainlessMap { key_type: Box<PainlessType>, value_type: Box<PainlessType> },
    PainlessObject // a PainlessObject is just like a PainlessMap, but without the type constraints
}

enum IntWidth {
    Eight,
    Sixteen,
    ThirtyTwo,
    SixtyFour
}

