#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Message<'c> {
    pub subject: &'c str,
    pub body: Option<&'c str>,
    #[doc(hidden)]
    __do_not_match_exhaustively: (),
}

impl<'c> Message<'c> {
    pub fn parse(commit: &'c str) -> Result<Self, failure::Error> {
        let (subject, body) = split_parts(commit);
        let c = Message {
            subject,
            body,
            __do_not_match_exhaustively: (),
        };

        Ok(c)
    }
}

static SECTION_RE: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new("\r?\n\r?\n").unwrap());

fn split_parts(commit: &str) -> (&str, Option<&str>) {
    let mut sections = SECTION_RE.splitn(commit, 2);
    let subject = sections.next().expect("Regex should always match");
    let body = sections.next().map(|s| s.trim_end()).unwrap_or("");
    if body.is_empty() {
        (subject, None)
    } else {
        (subject, Some(body))
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
