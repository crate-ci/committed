#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

mod style;

pub mod conventional;
pub mod no_style;

pub use style::*;
