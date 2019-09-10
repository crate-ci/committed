static DEFAULT_TYPES: &'static [&'static str] = &[
    "fix", "feat", "chore", "docs", "style", "refactor", "perf", "test",
];

#[derive(Copy, Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Style {
    Conventional,
    None,
}

#[derive(Clone, Default, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Config {
    subject_length: Option<usize>,
    subject_capitalized: Option<bool>,
    subject_not_punctuated: Option<bool>,
    imperative_subject: Option<bool>,
    no_fixup: Option<bool>,
    no_wip: Option<bool>,
    hard_line_length: Option<usize>,
    line_length: Option<usize>,
    style: Option<Style>,
    allowed_types: Option<Vec<String>>,
    pub merge_commit: Option<bool>,
}

impl Config {
    pub fn subject_length(&self) -> usize {
        self.subject_length.unwrap_or(50)
    }

    pub fn subject_capitalized(&self) -> bool {
        self.subject_capitalized.unwrap_or(true)
    }

    pub fn subject_not_punctuated(&self) -> bool {
        self.subject_not_punctuated.unwrap_or(true)
    }

    pub fn imperative_subject(&self) -> bool {
        self.imperative_subject.unwrap_or(true)
    }

    pub fn no_fixup(&self) -> bool {
        self.no_fixup.unwrap_or(true)
    }

    pub fn no_wip(&self) -> bool {
        self.no_wip.unwrap_or(true)
    }

    pub fn line_length(&self) -> usize {
        self.line_length.unwrap_or(72)
    }

    pub fn hard_line_length(&self) -> usize {
        self.hard_line_length.unwrap_or(0)
    }

    pub fn style(&self) -> Style {
        self.style.unwrap_or(Style::None)
    }

    pub fn allowed_types<'s>(&'s self) -> Box<dyn Iterator<Item = &str> + 's> {
        self.allowed_types
            .as_ref()
            .map(|v| {
                let b: Box<dyn Iterator<Item = &str>> = Box::new(v.iter().map(|s| s.as_str()));
                b
            })
            .unwrap_or_else(|| Box::new(DEFAULT_TYPES.iter().map(|s| *s)))
    }

    pub fn merge_commit(&self) -> bool {
        self.merge_commit.unwrap_or(true)
    }
}
