#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct Palette {
    pub(crate) source: anstyle::Style,
    pub(crate) error: anstyle::Style,
    pub(crate) content: anstyle::Style,
}

impl Palette {
    pub(crate) fn new() -> Self {
        Self {
            source: anstyle::AnsiColor::Blue.on_default() | anstyle::Effects::BOLD,
            error: anstyle::AnsiColor::Red.on_default() | anstyle::Effects::BOLD,
            content: anstyle::Style::default(),
        }
    }

    pub(crate) fn source<D: std::fmt::Display>(self, display: D) -> Styled<D> {
        Styled::new(display, self.source)
    }

    pub(crate) fn content<D: std::fmt::Display>(self, display: D) -> Styled<D> {
        Styled::new(display, self.content)
    }
}

#[derive(Debug)]
pub(crate) struct Styled<D> {
    display: D,
    style: anstyle::Style,
}

impl<D: std::fmt::Display> Styled<D> {
    pub(crate) fn new(display: D, style: anstyle::Style) -> Self {
        Self { display, style }
    }
}

impl<D: std::fmt::Display> std::fmt::Display for Styled<D> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(
                f,
                "{}{}{}",
                self.style.render(),
                self.display,
                self.style.render_reset()
            )
        } else {
            self.display.fmt(f)
        }
    }
}
