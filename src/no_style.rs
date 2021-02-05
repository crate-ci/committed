#[derive(Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Commit<'c> {
    pub raw_subject: &'c str,
    pub body: Option<&'c str>,
}

impl<'c> Commit<'c> {
    pub fn parse(commit: &'c str) -> Result<Self, anyhow::Error> {
        let (raw_subject, body) = split_parts(commit);
        let c = Commit { raw_subject, body };

        Ok(c)
    }
}

impl<'c> crate::style::Style for Commit<'c> {
    fn subject(&self) -> &str {
        self.raw_subject
    }

    fn body(&self) -> Option<&str> {
        self.body
    }

    fn type_(&self) -> Option<unicase::UniCase<&str>> {
        None
    }

    fn scope(&self) -> Option<unicase::UniCase<&str>> {
        None
    }
}

static SECTION_RE: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new("\r?\n").unwrap());

fn split_parts(commit: &str) -> (&str, Option<&str>) {
    let mut sections = SECTION_RE.splitn(commit, 2);
    let raw_subject = sections.next().expect("Regex should always match");
    let body = sections.next().map(|s| s.trim()).unwrap_or("");
    if body.is_empty() {
        (raw_subject, None)
    } else {
        (raw_subject, Some(body))
    }
}

#[cfg(test)]
mod test_split_parts {
    use super::*;

    #[test]
    fn subject() {
        let actual = split_parts("feat(parser): Parse bad greetings");
        let expected = ("feat(parser): Parse bad greetings", None);
        assert_eq!(actual, expected);
    }

    #[test]
    fn body() {
        let actual = split_parts(
            r#"feat(parser): Parse bad greetings

Hello
World

Foo
Bar"#,
        );
        let expected = (
            "feat(parser): Parse bad greetings",
            Some("Hello\nWorld\n\nFoo\nBar"),
        );
        assert_eq!(actual, expected);
    }
}
