#[macro_use]
extern crate log;

mod schema_definition;
mod data_location;

#[cfg(test)]
mod tests {
    use crate::schema_definition::IntWidth;
    use crate::schema_definition::PainlessValue;
    use crate::schema_definition::PainlessVariable;
    use crate::schema_definition::PainlessSchema;
    use crate::schema_definition::PainlessStruct;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_schema_serialization() {
        let version = semver::Version::parse("0.0.1").unwrap();
        let sixty_four_bit = IntWidth::SixtyFour;
        let uint64 = PainlessValue::PainlessInteger { sign : false, width : sixty_four_bit, int_value: 64 };
        let painless_var = PainlessVariable { name : "test_int".to_string(), painless: uint64 };

        let painless_struct = PainlessStruct { variables : Box::new([painless_var]) };

        let painless_schema =
            PainlessSchema 
            { 
                painless_schema_version : version,
                painless_schema_name : "test name".to_string(),
                painless_structs : Box::new([painless_struct]) 
            };

        println!("{}", serde_yaml::to_string(&painless_schema).unwrap());
    }
}
