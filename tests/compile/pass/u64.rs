use identifier::{FromStr, Identifier};

mod mod_id {
    pub fn generate() -> u64 {
        0x1234567890_abcdef
    }

    pub fn validate(_: u64) -> bool {
        true
    }
}

#[derive(Identifier, Eq, PartialEq, FromStr, Debug)]
#[identifier(with = "mod_id")]
struct Id(u64);

fn main() {
    let id = Id::generate();
    let expected_id: Id = "1234567890-abcdef".parse().unwrap();
    assert_eq!(id, expected_id);
}
