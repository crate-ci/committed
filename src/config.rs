#[derive(Copy, Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Style {
    Conventional,
    None,
}

#[derive(Clone, Default, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Config {
    style: Option<Style>,
}

impl Config {
    pub fn style(&self) -> Style {
        self.style.unwrap_or(Style::Conventional)
    }
}
