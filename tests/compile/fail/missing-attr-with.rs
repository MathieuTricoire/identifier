use identifier::Identifier;

#[derive(Identifier, Eq, PartialEq)]
#[identifier(params = "1")]
struct Id(u128);

fn main() {}
