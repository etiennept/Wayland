pub mod client;



use std::ffi::{c_void, CStr, CString};
use std::fmt::Display;
use std::ptr::{null, null_mut};


 macro_rules! include_file {
    ( $path:expr ) => {
        include!(concat!(env!("OUT_DIR"), $path));
    };
}

pub mod util{
    include_file!("/wayland_util.rs");
}











#[cfg(test)]
mod tests {


    use std::ffi::CStr;

    use std::ptr::null;



    use super::util::*;




    #[test]
    fn it_works() {
        println!("eeee")
    }

    #[test]
    fn test_1  (){



    }

}
