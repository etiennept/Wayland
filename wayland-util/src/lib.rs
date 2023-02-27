pub mod socket;
pub mod wire;
pub mod protocol;
mod map;


#[repr(u32)]
pub enum A {
    A = 1,
    B = 2,

}




#[cfg(test)]
mod test {
    use crate::A;

    #[test]
    fn a() {

    }
}







