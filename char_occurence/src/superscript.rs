pub trait Superscript {
    fn superscript(&self) -> String;
}

pub static NUMBERS: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];

impl Superscript for u8 {
    fn superscript(&self) -> String {
        self.to_string()
            .chars()
            .map(|c| superscript_digit(c))
            .collect()
    }
}
