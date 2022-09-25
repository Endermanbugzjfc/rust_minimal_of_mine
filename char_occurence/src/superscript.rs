pub trait Superscript {
    fn superscript(&self) -> String;
}

static NUMBERS: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];

impl Superscript for usize {
    fn superscript(&self) -> String {
        let n = *self;
        if n >= NUMBERS.len() {
            (n / 10).superscript() + &(n % 10).superscript()
        } else {
            NUMBERS[n].to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u32_max() {
        let n = u32::MAX as usize;
        assert_eq!(n.superscript(), "⁴²⁹⁴⁹⁶⁷²⁹⁵");
    }
}
