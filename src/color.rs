#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct Palette {
    pub(crate) source: styled::Style,
    pub(crate) error: styled::Style,
    pub(crate) content: styled::Style,
}

impl Palette {
    pub(crate) fn current() -> Self {
        if concolor::get(concolor::Stream::Either).ansi_color() {
            Self {
                source: styled::Style(yansi::Style::new(yansi::Color::Blue).bold()),
                error: styled::Style(yansi::Style::new(yansi::Color::Red).bold()),
                content: styled::Style::default(),
            }
        } else {
            Self::default()
        }
    }
}

mod styled {
    #[derive(Copy, Clone, Debug, Default)]
    pub(crate) struct Style(pub(crate) yansi::Style);

    impl Style {
        pub(crate) fn paint<T: std::fmt::Display>(self, item: T) -> impl std::fmt::Display {
            self.0.paint(item)
        }
    }
}
