use core::borrow::{Borrow, BorrowMut};
use core::fmt::{Arguments, Display};
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::panic::catch_unwind;
use std::ptr::null;



use crate::{CWlArgument, WlArgument, WlInterface, WlMessage, WlObject, WlRequests};


//#[link("wayland-client")]
extern "C" {
    fn wl_display_connect(name :* const c_char  ) ->  * mut c_void ;
    fn wl_display_disconnect( display  :  * mut c_void ) ;
    fn wl_proxy_get_class (proxy :  & mut dyn  WlProxy) -> * const c_char ;
    fn wl_proxy_get_version  (proxy :  & mut  dyn WlProxy) -> u32 ;
    fn wl_proxy_destroy( proxy : & mut dyn WlProxy) ;
    //fn wl_proxy_create(  )
    fn wl_proxy_set_user_data (proxy : & mut dyn WlProxy, data : * mut  c_void ) ;
    fn wl_proxy_get_user_data(proxy : & mut dyn WlProxy ) ->  * mut c_void  ;
    fn wl_proxy_marshal_flags( proxy : & mut dyn WlProxy, opcode :  u32 ,   interface  : *const WlInterface ,  flags: u32, args : ... );
    fn wl_proxy_marshal_array_flags(proxy: & mut dyn WlProxy ,
        opcode: u32,
        interface: *const WlInterface,
        version: u32,
        flags: u32,
        args: *mut CWlArgument,
    ) -> *mut c_void;
    fn wl_proxy_add_dispatcher(proxy : &mut dyn WlProxy, dispatcher_func  : Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_void,
                arg2: *mut ::std::os::raw::c_void,
                arg3: u32,
                arg4: *const WlMessage,
                arg5: *mut CWlArgument,
            ) -> ::std::os::raw::c_int>  , dispatcher_data  : *const ::std::os::raw::c_void   ,  data : * mut c_void ) ;
    fn wl_proxy_get_listener( proxy :   & mut dyn WlProxy  ) -> * mut c_void ;
    fn wl_proxy_get_id ( proxy :   & mut dyn WlProxy )   -> i32 ;
}
unsafe extern "C" fn dispatcher_data( proxy : * mut dyn WlProxy , data: *mut ::std::os::raw::c_void, arg3: u32,
                                     wl_message: *const WlInterface,
                                     wl_argument: *mut CWlArgument, ) -> std::os::raw::c_int {
    catch_unwind(
        ||{

        }
    ).is_err() as i32
 }

/*
impl WlDisplay{
    fn connect_to_env<'a> () -> &'a mut WlDisplay {
        unsafe {
            wl_display_connect( null()).as_mut()
        }.unwrap()
    }
    fn connect <'a>(name : &str ) -> &'a mut WlDisplay {
        unsafe {
            wl_display_connect( CString::new(name ).unwrap().as_ptr() ).as_mut()
        }.unwrap()
    }

    fn disconnect (&self){
        unsafe {
            wl_display_disconnect( self) ;
        }
    }
} */

trait WlProxy : WlObject {
    fn get_class (& mut   self) -> &str where Self: Sized {
        unsafe {CStr::from_ptr(wl_proxy_get_class( self )).to_str() }.unwrap()
    }
    fn get_version (&mut self) -> u32 where Self: Sized {
        unsafe {wl_proxy_get_version(self)}
    }
    fn get_user_data <T>  (& mut self  ,   ) -> * mut  T  where Self: Sized{
        unsafe { wl_proxy_get_user_data(self) as * mut T }
    }
    fn get_user_data_mut <T> (& mut self) -> * mut T    where Self: Sized{
        unsafe { wl_proxy_get_user_data(self  ) as * mut T }
    }
    fn set_user_data <T> (& mut self  , data  : *const T    ) where Self: Sized{
        unsafe { wl_proxy_set_user_data(self ,  data   as * mut  c_void  ) }
    }
    fn destroy (&mut self ) where Self: Sized {
        unsafe { wl_proxy_destroy(self) }
    }
    fn marshal_flag <   T :   WlRequests > (&mut self, opcode : T ,  is_destructor : bool  , mut  arguments : Vec<WlArgument> ) where Self : Sized{
        unsafe {
            wl_proxy_marshal_array_flags(self, opcode.get_opcode(), null(), wl_proxy_get_version(self ), is_destructor as u32, to_c_arguments(arguments   )   )
        };
    }
    fn marshal_constructed <  R : WlObject ,   T :   WlRequests > (&mut self, opcode : T ,  is_destructor : bool  , mut  arguments : Vec<WlArgument>  ) -> &mut  T   where Self : Sized   {
        unsafe {
            wl_proxy_marshal_array_flags(self, opcode.get_opcode(), &R::get_interface() as  * const WlInterface, wl_proxy_get_version(self ), is_destructor as u32, to_c_arguments(arguments   )   ).as_mut().unwrap()
        }
    }


    fn get_listener<T>(&mut self) -> *mut T  where Self : Sized{
        unsafe { wl_proxy_get_listener(   self ) as  * mut  T  }
    }

}



fn to_c_arguments(arguments  :  Vec<WlArgument>  ) -> *mut CWlArgument {
    arguments.iter().map(|it |{
        it.to_CWlArgument()
    }).collect::<Vec<CWlArgument>>().as_mut_ptr()
}


#[cfg(test)]
mod test {
    use alloc::rc::Rc;
    use std::alloc::{alloc, Layout};
    use std::cell::RefCell;
    use std::ffi::{CStr, CString};
    use std::os::raw::{c_char, c_void};
    use std::panic::catch_unwind;
    use std::ptr::{drop_in_place, null};



    #[test]
    fn test1 (){
        let a = env!("WAYLAND_DISPLAY");
        let b = env!("XDG_RUNTIME_DIR");
     }
    #[test]
    fn test (){


        println!("{}"  , true as i32)



        // println!("{}" , a.get_class()    );
        //println!("{}"   , a .get_version() ) ;

        //println!("{}" , x )
    }
}





