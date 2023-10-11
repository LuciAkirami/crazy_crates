
pub fn prettified (sentence: &str, style: Graphics) -> String {
    return  style.to_string() + sentence + &Graphics::Reset.to_string();
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Graphics{
    Reset,
    Bold,
    Italic,
    Underline,
    Strikethrough
}

pub mod graphics_impl {
    use std::fmt::{Display, Formatter, Result};

    use super::*;

    impl Display for Graphics {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{}", make_graphics(*self))
        }
    }

    pub const CSI: &str = "\x1b[";
    pub const SGR: &str = "m";

    //#[rustfmt::skip]
    fn make_graphics(graphic: Graphics) -> String {
        match graphic {
            Graphics::Reset              => format!("{CSI}0{SGR}"),
            Graphics::Bold              => format!("{CSI}1{SGR}"),
            Graphics::Italic            => format!("{CSI}3{SGR}"),
            Graphics::Underline         => format!("{CSI}4{SGR}"),
            Graphics::Strikethrough     => format!("{CSI}9{SGR}"),
        }
    }
}

#[cfg(test)]
mod test{
    use super::Graphics;
    
    #[test]
    fn bold() {
        let graphics = Graphics::Bold;
        assert_eq!(graphics.to_string(), "\x1b[1m");
    }

    #[test]
    fn reset() {
        let graphics = Graphics::Reset;
        assert_eq!(graphics.to_string(), "\x1b[0m");
    }

    #[test]
    fn italic() {
        let graphics = Graphics::Italic;
        assert_eq!(graphics.to_string(), "\x1b[3m");
    }

    #[test]
    fn underline() {
        let graphics = Graphics::Underline;
        assert_eq!(graphics.to_string(), "\x1b[4m");
    }

    #[test]
    fn strikethrough() {
        let graphics = Graphics::Strikethrough;
        assert_eq!(graphics.to_string(), "\x1b[9m");
    }
}