#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
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
pub enum Source<'s> {
    #[serde(serialize_with = "serialize_oid")]
    Oid(git2::Oid),
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
pub enum Severity {
    #[display(fmt = "error")]
    Error,
}

#[derive(Debug, serde::Serialize, derive_more::From, derive_more::Display)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Content<'s> {
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
#[display(fmt = "Invalid commit format {}", error)]
pub struct InvalidCommitFormat {
    #[serde(serialize_with = "serialize_error")]
    pub error: anyhow::Error,
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

pub type Report = fn(msg: Message);

pub fn print_silent(_: Message) {}

pub fn print_brief(msg: Message) {
    println!("{}: {} {}", msg.source, msg.severity, msg.content)
}

pub fn print_json(msg: Message) {
    println!("{}", serde_json::to_string(&msg).unwrap());
}
