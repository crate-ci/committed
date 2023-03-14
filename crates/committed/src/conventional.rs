pub use git_conventional::Commit;

impl<'c> crate::style::Style for Commit<'c> {
    fn subject(&self) -> &str {
        self.description()
    }

    fn body(&self) -> Option<&str> {
        self.body()
    }

    fn type_(&self) -> Option<unicase::UniCase<&str>> {
        Some(unicase::UniCase::new(self.type_().as_str()))
    }

    fn scope(&self) -> Option<unicase::UniCase<&str>> {
        self.scope().map(|s| unicase::UniCase::new(s.as_str()))
    }
}
