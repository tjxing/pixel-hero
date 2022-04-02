#[derive(Copy, Clone)]
pub enum Timing {
    NTSC,
    PAL,
    MultipleRegion,
    Dendy
}

impl PartialEq<Timing> for Timing {

    fn eq(&self, other: &Timing) -> bool {
        let x = self.clone() as u8;
        let y = other.clone() as u8;
        x == y
    }

}

impl PartialEq<Timing> for &Timing {

    fn eq(&self, other: &Timing) -> bool {
        (*self).eq(other)
    }

}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        let x = Timing::NTSC;
        let y = Timing:: PAL;
        assert_eq!(false, x == y);
        assert_eq!(false, &x == &y);

        let y = Timing::NTSC;
        assert_eq!(true, x == y);
        assert_eq!(true, &x == &y);
    }
}