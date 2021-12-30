use std::rc::Rc;

pub struct Slice {
    data: Rc<[u8]>,
    start: usize,
    length: usize
}

impl Slice {
    pub fn new(data: &Rc<[u8]>, start: u32, length: u32) -> Slice {
        Slice {
            data: Rc::clone(data),
            start: start as usize,
            length: length as usize
        }
    }

    pub fn length(&self) -> u32 {
        self.length as u32
    }

    pub fn at(&self, index: u32) -> u8 {
        let i = self.start + index as usize;
        self.data[i]
    }
}