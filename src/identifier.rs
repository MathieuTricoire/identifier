/// Trait for id structures.
///
/// This trait is not intented to be implemented manually you should use derive.
/// Refers to the module documentation to provide a module with functions to:
/// - generate inner id / u128 value
/// - validate
///
pub trait Identifier<T>: std::cmp::Eq {
    type Id;
    type ParseError;

    /// Generate a new id
    fn generate() -> Self::Id;

    // Create an id from the underlying value
    fn new(v: T) -> Self::Id;

    /// Parse a string to an id
    fn parse_str(s: &str) -> Result<Self::Id, Self::ParseError>;

    /// format the id to a string
    fn format(&self) -> String;

    /// Return inner value
    fn inner_value(&self) -> T;
}
