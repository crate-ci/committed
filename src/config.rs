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
    line_length: Option<usize>,
    style: Option<Style>,
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

    pub fn style(&self) -> Style {
        self.style.unwrap_or(Style::None)
    }

    pub fn merge_commit(&self) -> bool {
        self.merge_commit.unwrap_or(true)
    }
}
