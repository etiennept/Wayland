use crate::util::* ;

macro_rules! include_file {
    ( $path:expr ) => {
        include!(concat!(env!("OUT_DIR"), $path));
    };
}





#[cfg(test) ]
mod test {
    use std::ffi::CStr;
    use std::ptr::null;


    enum WlDisplay{ }
    #[test]
    fn ee (){




    }
}