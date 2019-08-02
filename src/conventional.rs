#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Message<'c> {
    pub raw_subject: &'c str,
    pub commit_type: unicase::UniCase<&'c str>,
    pub scope: Option<unicase::UniCase<&'c str>>,
    pub important: bool,
    pub subject: &'c str,

    pub body: Option<&'c str>,
    pub trailer: Option<Vec<&'c str>>,
    #[doc(hidden)]
    __do_not_match_exhaustively: (),
}

impl<'c> Message<'c> {
    pub fn parse(commit: &'c str) -> Result<Self, failure::Error> {
        let commit = commit.trim();

        let mut sections = split_sections(commit);

        let raw_subject = sections
            .next()
            .ok_or_else(|| failure::Context::new("Commit is empty"))?;
        let (commit_type, scope, important, subject) = parse_subject(raw_subject)?;
        let commit_type = unicase::UniCase::new(commit_type);
        let scope = scope.map(|s| unicase::UniCase::new(s));

        let body = sections.next();
        let trailer = sections.next().map(|s| s.lines().collect());
        if let Some(section) = sections.next() {
            failure::bail!("Cannot have sections past body+trailer: ```{}```", section);
        }

        let c = Message {
            raw_subject,
            commit_type,
            scope,
            important,
            subject,
            body,
            trailer,
            __do_not_match_exhaustively: (),
        };
        Ok(c)
    }
}

impl<'c> crate::style::Style for Message<'c> {
    fn subject(&self) -> &str {
        self.subject
    }
}

static SECTION_RE: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new("\r?\n\r?\n").unwrap());

fn split_sections(commit: &str) -> impl Iterator<Item = &str> {
    SECTION_RE.split(commit).map(|s| s.trim())
}

#[cfg(test)]
mod test_split_sections {
    use super::*;

    #[test]
    fn subject() {
        let actual: Vec<_> = split_sections("feat(parser): Parse bad greetings").collect();
        let expected = ["feat(parser): Parse bad greetings"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn body() {
        let actual: Vec<_> = split_sections(
            r#"feat(parser): Parse bad greetings

Hello
World

Foo
Bar"#,
        )
        .collect();
        let expected = [
            "feat(parser): Parse bad greetings",
            "Hello\nWorld",
            "Foo\nBar",
        ];
        assert_eq!(actual, expected);
    }
}

static META_RE: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new(r#"^(.*?)(\(.*?\))?(!)?$"#).unwrap());

fn parse_subject(raw_subject: &str) -> Result<(&str, Option<&str>, bool, &str), failure::Error> {
    if raw_subject.contains("\n") {
        failure::bail!("Subject must be a single line");
    }

    let mut parts = raw_subject.splitn(2, ":");
    let meta = parts.next().unwrap();
    let subject = parts
        .next()
        .ok_or_else(|| failure::Context::new("No commit metadata provided"))?
        .trim();

    let captures = META_RE
        .captures(meta)
        .expect("Regex should match against everything");
    let commit_type = captures
        .get(1)
        .expect("commit_type should match against everything")
        .as_str();
    let scope = captures
        .get(2)
        .map(|m| m.as_str().trim_start_matches('(').trim_end_matches(')'));
    let important = captures.get(3).is_some();

    if scope.is_none() {
        if commit_type.contains('(') {
            failure::bail!("Scope has unclosed '('");
        } else if commit_type.contains(')') {
            failure::bail!("Scope is closed but never opened");
        }
    }

    Ok((commit_type, scope, important, subject))
}

#[cfg(test)]
mod test_parse_subject {
    use super::*;

    #[test]
    fn basic() {
        let actual = parse_subject("feat: Parse bad greetings").unwrap();
        let expected = ("feat", None, false, "Parse bad greetings");
        assert_eq!(actual, expected);
    }

    #[test]
    fn with_scope() {
        let actual = parse_subject("feat(parser): Parse bad greetings").unwrap();
        let expected = ("feat", Some("parser"), false, "Parse bad greetings");
        assert_eq!(actual, expected);
    }

    #[test]
    fn with_important() {
        let actual = parse_subject("feat!: Parse bad greetings").unwrap();
        let expected = ("feat", None, true, "Parse bad greetings");
        assert_eq!(actual, expected);
    }

    #[test]
    fn with_scope_and_important() {
        let actual = parse_subject("feat(parser)!: Parse bad greetings").unwrap();
        let expected = ("feat", Some("parser"), true, "Parse bad greetings");
        assert_eq!(actual, expected);
    }

    #[test]
    fn error_without_metadata() {
        parse_subject("Parse bad greetings").unwrap_err();
    }

    #[test]
    fn error_on_unclosed() {
        parse_subject("feat(parser: Parse bad greetings").unwrap_err();
    }

    #[test]
    fn error_on_unopened() {
        parse_subject("featparser): Parse bad greetings").unwrap_err();
    }
}
