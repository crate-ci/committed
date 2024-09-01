use anstream::println;

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub(crate) struct Message<'s> {
    pub(crate) source: Source<'s>,
    pub(crate) severity: Severity,
    pub(crate) content: Content<'s>,
}

impl<'s> Message<'s> {
    pub(crate) fn error<S, C>(source: S, content: C) -> Self
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
pub(crate) enum Source<'s> {
    #[serde(serialize_with = "serialize_oid")]
    Oid(git2::Oid),
    ShortId(&'s str),
    #[display("{}", "_0.display()")]
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
pub(crate) enum Severity {
    #[display("error")]
    Error,
}

#[derive(Debug, serde::Serialize, derive_more::From, derive_more::Display)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
#[non_exhaustive]
pub(crate) enum Content<'s> {
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
    "Commit subject is too long, {} exceeds the max length of {}",
    actual_length,
    max_length
)]
pub(crate) struct SubjectTooLong {
    pub(crate) max_length: usize,
    pub(crate) actual_length: usize,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[derive(derive_more::Display)]
#[display(
    "Line is too long, {} exceeds the max length of {}",
    actual_length,
    max_length
)]
pub(crate) struct LineTooLong {
    pub(crate) max_length: usize,
    pub(crate) actual_length: usize,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[derive(derive_more::Display)]
#[display("Subject should be capitalized but found `{}`", first_word)]
pub(crate) struct CapitalizeSubject<'s> {
    pub(crate) first_word: &'s str,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[derive(derive_more::Display)]
#[display("Subject should not be punctuated but found `{}`", punctuation)]
pub(crate) struct NoPunctuation {
    pub(crate) punctuation: char,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[derive(derive_more::Display)]
#[display("Subject should be in the imperative mood but found `{}`", first_word)]
pub(crate) struct Imperative<'s> {
    pub(crate) first_word: &'s str,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[derive(derive_more::Display)]
#[display("Work-in-progress commits must be cleaned up")]
pub(crate) struct Wip {}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[derive(derive_more::Display)]
#[display("Fixup commits must be squashed")]
pub(crate) struct Fixup {}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[derive(derive_more::Display)]
#[display("Commit is not in {} format: {}", style, error)]
pub(crate) struct InvalidCommitFormat {
    #[serde(serialize_with = "serialize_error")]
    pub(crate) error: anyhow::Error,
    pub(crate) style: crate::config::Style,
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
#[display("Disallowed type `{}` used, please use one of {:?}", used, allowed)]
pub(crate) struct DisallowedCommitType {
    pub(crate) used: String,
    pub(crate) allowed: Vec<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[derive(derive_more::Display)]
#[display("Merge commits are disallowed")]
pub(crate) struct MergeCommitDisallowed {}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[derive(derive_more::Display)]
#[display("Empty commits are disallowed")]
pub(crate) struct EmptyCommit {}

pub(crate) type Report = fn(msg: Message<'_>);

pub(crate) fn print_silent(_: Message<'_>) {}

pub(crate) fn print_brief(msg: Message<'_>) {
    let palette = crate::color::Palette::new();
    let severity_style = match msg.severity {
        Severity::Error => palette.error,
    };
    println!(
        "{:#}: {:#} {:#}",
        palette.source(msg.source),
        crate::color::Styled::new(msg.severity, severity_style),
        palette.content(msg.content)
    );
}

pub(crate) fn print_json(msg: Message<'_>) {
    println!("{}", serde_json::to_string(&msg).unwrap());
}
