pub trait Style {
    fn subject(&self) -> &str;
    fn body(&self) -> Option<&str>;

    fn type_(&self) -> Option<unicase::UniCase<&str>>;
    fn scope(&self) -> Option<unicase::UniCase<&str>>;
}
