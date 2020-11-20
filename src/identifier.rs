/// Trait for id structures.
///
/// This trait is not intented to be implemented manually you should use derive.
/// Refers to the module documentation to provide a module with functions to:
/// - generate inner id / u128 value
/// - validate
///
pub trait Identifier: std::cmp::Eq {
    type Id;
    type ParseError;

    /// Generate a new id
    fn generate() -> Self::Id;

    /// format the id to a string
    fn format(&self) -> String;

    /// Parse a string to an id
    fn parse_str(s: &str) -> Result<Self::Id, Self::ParseError>;
}
