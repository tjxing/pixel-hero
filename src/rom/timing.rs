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