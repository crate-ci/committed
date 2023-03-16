use anstream::println;

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct Message<'s> {
    pub source: Source<'s>,
    pub severity: Severity,
    pub content: Content<'s>,
}

impl<'s> Message<'s> {
    pub fn error<S, C>(source: S, content: C) -> Self
    where
        S: Into<Source<'s>>,
        C: Into<Content<'s>>,
    {
        Message {
            source: source.into(),
            severity: Severity::Error,
            content: content.into(),
        }
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize, derive_more::From, derive_more::Display)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
#[non_exhaustive]
pub enum Source<'s> {
    #[serde(serialize_with = "serialize_oid")]
    Oid(git2::Oid),
    ShortId(&'s str),
    #[display(fmt = "{}", "_0.display()")]
    Path(&'s std::path::Path),
}

fn serialize_oid<S>(oid: &git2::Oid, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let oid = oid.to_string();
    s.serialize_str(&oid)
}

#[derive(Copy, Clone, Debug, serde::Serialize, derive_more::Display)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Severity {
    #[display(fmt = "error")]
    Error,
}

#[derive(Debug, serde::Serialize, derive_more::From, derive_more::Display)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum Content<'s> {
    EmptyCommit(EmptyCommit),
    SubjectTooLong(SubjectTooLong),
    LineTooLong(LineTooLong),
    CapitalizeSubject(CapitalizeSubject<'s>),
    NoPunctuation(NoPunctuation),
    Imperative(Imperative<'s>),
    Wip(Wip),
    Fixup(Fixup),
    InvalidCommitFormat(InvalidCommitFormat),
    DisallowedCommitType(DisallowedCommitType),
    MergeCommitDisallowed(MergeCommitDisallowed),
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[derive(derive_more::Display)]
#[display(
    fmt = "Commit subject is too long, {} exceeds the max length of {}",
    actual_length,
    max_length
)]
pub struct SubjectTooLong {
    pub max_length: usize,
    pub actual_length: usize,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[derive(derive_more::Display)]
#[display(
    fmt = "Line is too long, {} exceeds the max length of {}",
    actual_length,
    max_length
)]
pub struct LineTooLong {
    pub max_length: usize,
    pub actual_length: usize,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[derive(derive_more::Display)]
#[display(fmt = "Subject should be capitalized but found `{}`", first_word)]
pub struct CapitalizeSubject<'s> {
    pub first_word: &'s str,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[derive(derive_more::Display)]
#[display(fmt = "Subject should not be punctuated but found `{}`", punctuation)]
pub struct NoPunctuation {
    pub punctuation: char,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[derive(derive_more::Display)]
#[display(
    fmt = "Subject should be in the imperative mood but found `{}`",
    first_word
)]
pub struct Imperative<'s> {
    pub first_word: &'s str,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[derive(derive_more::Display)]
#[display(fmt = "Work-in-progress commits must be cleaned up")]
pub struct Wip {}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[derive(derive_more::Display)]
#[display(fmt = "Fixup commits must be squashed")]
pub struct Fixup {}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[derive(derive_more::Display)]
#[display(fmt = "Commit is not in {} format: {}", style, error)]
pub struct InvalidCommitFormat {
    #[serde(serialize_with = "serialize_error")]
    pub error: anyhow::Error,
    pub style: crate::config::Style,
}

fn serialize_error<S>(error: &anyhow::Error, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let error = error.to_string();
    s.serialize_str(&error)
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[derive(derive_more::Display)]
#[display(
    fmt = "Disallowed type `{}` used, please use one of {:?}",
    used,
    allowed
)]
pub struct DisallowedCommitType {
    pub used: String,
    pub allowed: Vec<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[derive(derive_more::Display)]
#[display(fmt = "Merge commits are disallowed")]
pub struct MergeCommitDisallowed {}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[derive(derive_more::Display)]
#[display(fmt = "Empty commits are disallowed")]
pub struct EmptyCommit {}

pub type Report = fn(msg: Message);

pub fn print_silent(_: Message) {}

pub fn print_brief(msg: Message) {
    let palette = crate::color::Palette::new();
    let severity_style = match msg.severity {
        Severity::Error => palette.error,
    };
    println!(
        "{:#}: {:#} {:#}",
        palette.source(msg.source),
        crate::color::Styled::new(msg.severity, severity_style),
        palette.content(msg.content)
    )
}

pub fn print_json(msg: Message) {
    println!("{}", serde_json::to_string(&msg).unwrap());
}
