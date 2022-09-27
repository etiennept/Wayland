use std::ffi::c_void;
use std::ptr::{null, null_mut};
include!(concat!(env!("OUT_DIR"), "/wayland_client.rs"));

/*
pub unsafe fn wl_registry_add_listener(wl_registry : *mut wl_registry , listener : * mut wl_registry_listener , data :  *mut c_void) -> i32 {
    wl_proxy_add_listener(  wl_registry as * mut wl_proxy , listener as  *mut Option<unsafe extern "C" fn()>, data )
}

pub unsafe fn wl_display_get_registry(wl_display : *mut wl_display) -> *mut wl_registry {
    wl_proxy_marshal_flags(wl_display as * mut wl_proxy , WL_DISPLAY_GET_REGISTRY, &wl_registry_interface as * const wl_interface ,  wl_proxy_get_version(wl_display as * mut wl_proxy)  , 0   ) as *mut wl_registry
}


pub unsafe fn wl_registry_bind( wl_registry : * mut wl_registry  , name : u32 , interface : * const wl_interface ,   version : u32   ) -> *mut  wl_proxy{
    wl_proxy_marshal_flags(wl_registry  as * mut wl_proxy , WL_REGISTRY_BIND , interface  , version , 0 , (*interface).name ,version )  //as *mut c_void
}
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {


    }
}
