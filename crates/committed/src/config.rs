static DEFAULT_TYPES: &[&str] = &[
    "fix", "feat", "chore", "docs", "style", "refactor", "perf", "test",
];

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, derive_more::Display,
)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Style {
    #[serde(alias = "Conventional")]
    Conventional,
    #[serde(alias = "None")]
    None,
}

#[derive(Clone, Default, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub(crate) struct Config {
    pub(crate) ignore_author_re: Option<String>,
    pub(crate) subject_length: Option<usize>,
    pub(crate) subject_capitalized: Option<bool>,
    pub(crate) subject_not_punctuated: Option<bool>,
    pub(crate) imperative_subject: Option<bool>,
    pub(crate) no_fixup: Option<bool>,
    pub(crate) no_wip: Option<bool>,
    pub(crate) hard_line_length: Option<usize>,
    pub(crate) line_length: Option<usize>,
    pub(crate) style: Option<Style>,
    pub(crate) allowed_types: Option<Vec<String>>,
    pub(crate) merge_commit: Option<bool>,
}

impl Config {
    pub(crate) fn from_defaults() -> Self {
        let empty = Self::default();
        Self {
            ignore_author_re: empty.ignore_author_re().map(|s| s.to_owned()),
            subject_length: Some(empty.subject_length()),
            subject_capitalized: Some(empty.subject_capitalized()),
            subject_not_punctuated: Some(empty.subject_not_punctuated()),
            imperative_subject: Some(empty.imperative_subject()),
            no_fixup: Some(empty.no_fixup()),
            no_wip: Some(empty.no_wip()),
            hard_line_length: Some(empty.hard_line_length()),
            line_length: Some(empty.line_length()),
            style: Some(empty.style()),
            allowed_types: Some(empty.allowed_types().map(|s| s.to_owned()).collect()),
            merge_commit: Some(empty.merge_commit()),
        }
    }

    pub(crate) fn update(&mut self, source: Self) {
        if let Some(source) = source.ignore_author_re {
            self.ignore_author_re = Some(source);
        }
        if let Some(source) = source.subject_length {
            self.subject_length = Some(source);
        }
        if let Some(source) = source.subject_capitalized {
            self.subject_capitalized = Some(source);
        }
        if let Some(source) = source.subject_not_punctuated {
            self.subject_not_punctuated = Some(source);
        }
        if let Some(source) = source.imperative_subject {
            self.imperative_subject = Some(source);
        }
        if let Some(source) = source.no_fixup {
            self.no_fixup = Some(source);
        }
        if let Some(source) = source.no_wip {
            self.no_wip = Some(source);
        }
        if let Some(source) = source.hard_line_length {
            self.hard_line_length = Some(source);
        }
        if let Some(source) = source.line_length {
            self.line_length = Some(source);
        }
        if let Some(source) = source.style {
            self.style = Some(source);
        }
        if let Some(source) = source.allowed_types {
            self.allowed_types = Some(source);
        }
        if let Some(source) = source.merge_commit {
            self.merge_commit = Some(source);
        }
    }

    pub(crate) fn ignore_author_re(&self) -> Option<&str> {
        self.ignore_author_re.as_deref()
    }

    pub(crate) fn subject_length(&self) -> usize {
        self.subject_length.unwrap_or(50)
    }

    pub(crate) fn subject_capitalized(&self) -> bool {
        self.subject_capitalized.unwrap_or(true)
    }

    pub(crate) fn subject_not_punctuated(&self) -> bool {
        self.subject_not_punctuated.unwrap_or(true)
    }

    pub(crate) fn imperative_subject(&self) -> bool {
        self.imperative_subject.unwrap_or(true)
    }

    pub(crate) fn no_fixup(&self) -> bool {
        self.no_fixup.unwrap_or(true)
    }

    pub(crate) fn no_wip(&self) -> bool {
        self.no_wip.unwrap_or(true)
    }

    pub(crate) fn line_length(&self) -> usize {
        self.line_length.unwrap_or(72)
    }

    pub(crate) fn hard_line_length(&self) -> usize {
        self.hard_line_length.unwrap_or(0)
    }

    pub(crate) fn style(&self) -> Style {
        self.style.unwrap_or(Style::None)
    }

    pub(crate) fn allowed_types<'s>(&'s self) -> Box<dyn Iterator<Item = &str> + 's> {
        self.allowed_types
            .as_ref()
            .map(|v| {
                let b: Box<dyn Iterator<Item = &str>> = Box::new(v.iter().map(|s| s.as_str()));
                b
            })
            .unwrap_or_else(|| Box::new(DEFAULT_TYPES.iter().copied()))
    }

    pub(crate) fn merge_commit(&self) -> bool {
        self.merge_commit.unwrap_or(true)
    }
}
