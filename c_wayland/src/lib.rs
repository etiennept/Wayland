extern crate core;

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

#[cfg(feature = "client")]
pub mod client {
    use std::ffi::CStr;
    use std::os::raw::{c_char, c_int};
    use crate::util::*;
    include_file!("/wayland_client.rs");

    trait Ptr  {
        fn get_class  (& mut self) -> &str {
            unsafe { CStr::from_ptr( wl_proxy_get_class(self )  ).to_str().unwrap()  }
        }
        fn get_version (& mut self ) -> u32 {
            unsafe { wl_proxy_get_version( self )

            }
        }
        fn get_id( ) -> i32 ;


        /*fn add_listener<T >  (&mut self) -> T {
            unsafe { wl_proxy_add_listener(self ) }

        } */



    }


    impl wl_display{
        fn flush(& mut self) -> i32 {
           unsafe {wl_display_flush(self)}
        }
        fn connect_to_fd <'a> (fd : i32 ) -> Option<&'a mut wl_display> {
            unsafe { wl_display_connect_to_fd(fd  ).as_mut()}
        }
        fn  ee(){
            unsafe {

            }
        }
    }




}








fn eee() {

}



#[cfg(test)]
mod tests {
    use std::ffi::CStr;

    use std::ptr::null;

    use super::util::*;

    #[test]
    fn it_works() {

    }

    #[test]
    fn test_1  (){



    }

}
