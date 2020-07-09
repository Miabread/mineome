pub trait Validate {
    type Error;

    fn validate(&self) -> Vec<Self::Error>;
}
