use identifier::{FromStr, Identifier};

mod mod_id {
    pub fn generate(prefix: u32, suffix: u32) -> u128 {
        let rand: u128 = 0x01010101_10101010;
        ((prefix as u128) << 96) + (rand << 32) + suffix as u128
    }

    pub fn validate(value: u128, prefix: u32, suffix: u32) -> bool {
        (value >> 96) as u32 == prefix && value as u32 == suffix
    }
}

const PREFIX: u32 = 0x12345678;

#[derive(Identifier, Eq, PartialEq, FromStr, Debug)]
#[identifier(with = "mod_id", params = "PREFIX, 0x90abcdef")]
struct Id(u128);

fn main() {
    let id = Id::generate();
    let expected_id: Id = "12345678-01010101-10101010-90abcdef".parse().unwrap();
    assert_eq!(id, expected_id);

    let err_prefix = "00000000-01010101-10101010-12345678".parse::<Id>();
    assert!(if let Err(identifier::ParseError::Invalid) = err_prefix {
        true
    } else {
        false
    });

    let err_suffix = "12345678-01010101-10101010-00000000".parse::<Id>();
    assert!(if let Err(identifier::ParseError::Invalid) = err_suffix {
        true
    } else {
        false
    });
}
