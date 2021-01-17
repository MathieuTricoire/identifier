use identifier::{FromStr, Identifier};

mod mod_id {
    pub fn generate() -> u32 {
        0x12345678
    }

    pub fn validate(_: u32) -> bool {
        true
    }
}

#[derive(Identifier, Eq, PartialEq, FromStr, Debug)]
#[identifier(with = "mod_id")]
struct Id(u32);

fn main() {
    let id = Id::generate();
    let expected_id: Id = "12345678".parse().unwrap();
    assert_eq!(id, expected_id);
}
