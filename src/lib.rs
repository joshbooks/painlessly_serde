mod schema_definition;

#[cfg(test)]
mod tests {

    use crate::schema_definition::IntWidth;
    use crate::schema_definition::PainlessType;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_schema_serialization() {
        let version = semver::Version::parse("0.0.1").unwrap();
        let sixtyFourBit = IntWidth::SixtyFour;
        let uint64Type = PainlessType::PainlessInteger { sign : false, width : sixtyFourBit };
        println!("{}", serde_yaml::to_string(&uint64Type).unwrap());
    }
}
