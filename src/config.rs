#[derive(Copy, Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Style {
    Conventional,
    None,
}

#[derive(Clone, Default, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Config {
    subject_length: Option<usize>,
    style: Option<Style>,
}

impl Config {
    pub fn subject_length(&self) -> usize {
        self.subject_length.unwrap_or(50)
    }

    pub fn style(&self) -> Style {
        self.style.unwrap_or(Style::Conventional)
    }
}
