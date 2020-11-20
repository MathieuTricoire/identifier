use identifier::Identifier;

mod mod_id {
    pub fn generate() -> u128 {
        1
    }

    pub fn validate(_value: u128) -> bool {
        true
    }
}

#[derive(Identifier, Eq, PartialEq)]
#[identifier(with = "mod_id")]
struct Id(u64);

fn main() {}
