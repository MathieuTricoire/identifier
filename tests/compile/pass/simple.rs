use identifier::{FromStr, Identifier};

mod mod_id {
    pub fn generate() -> u128 {
        0x1234567890_abcdef_1234567890_abcdef
    }

    pub fn validate(_: u128) -> bool {
        true
    }
}

#[derive(Identifier, Eq, PartialEq, FromStr, Debug)]
#[identifier(with = "mod_id")]
struct Id(u128);

fn main() {
    let id = Id::generate();
    let expected_id: Id = "1234567890-abcdef-1234567890-abcdef".parse().unwrap();
    assert_eq!(id, expected_id);
}
