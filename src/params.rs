use std::{fmt::Display, num::NonZeroU32};

pub enum Window {
    ClassRegex(String),
    TitleRegex(String),
    Pid(NonZeroU32),
    Address(String),
    Floating,
    Tiled,
}

impl Display for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Window::ClassRegex(class_regex) => write!(f, "{class_regex}"),
            Window::TitleRegex(title_regex) => write!(f, "title:{title_regex}"),
            Window::Pid(pid) => write!(f, "pid:{pid}"),
            Window::Address(address) => write!(f, "address:{address}"),
            Window::Floating => write!(f, "floating"),
            Window::Tiled => write!(f, "tiled"),
        }
    }
}
