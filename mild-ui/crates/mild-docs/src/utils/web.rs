use std::fmt::{Display, Formatter, Result};

pub enum KeyBoard {
    Enter,
    Space,
}

impl Display for KeyBoard {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let s = match self {
            KeyBoard::Enter => "Enter",
            KeyBoard::Space => " ",
        };

        write!(f, "{s}")
    }
}
