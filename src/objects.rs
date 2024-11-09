
pub type Visual = Vec<u8>;

pub trait Rend {
    fn rend(&self) -> Visual;
}


#[derive(Clone)]
pub struct Pixel {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
    pub(crate) a: u8,
}
impl Rend for Pixel {
    fn rend(&self) -> Visual {
        vec![self.r, self.g, self.b, self.a]
    }
}

