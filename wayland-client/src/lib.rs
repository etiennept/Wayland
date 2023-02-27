mod connection;
mod c;

use std::env;
use std::ffi::{ CStr, CString};
use std::ops::Deref;
use std::os::raw::{c_void , c_int };
use std::os::unix::io::AsFd;
use std::os::unix::net::UnixStream;
use std::os::unix::prelude::{AsRawFd, FromRawFd};
use std::path::PathBuf;
use std::ptr::null;
use std::rc::Rc;
use std::sync::Arc;
//use nix::fcntl;
//use nix::fcntl::F_GETFD;
//use wayland_util::Interface;
use c_wayland::client::* ;
use c_wayland::util::*;


// wl_registry, wl_registry_bind, wl_compositor, wl_interface, wl_compositor_interface, wl_shell, wl_shell_interface;



struct Connection {

}

impl Connection  {
    fn connect (){
       /* if let Ok(a) = env::var("WAYLAND_SOCKET")  {
           let fd = a.parse::<i32>().unwrap();
            env::remove_var("WAYLAND_SOCKET");
            let flag = fcntl::fcntl(fd, F_GETFD);
            let result = flag.map(|f| fcntl::FdFlag::from_bits(f).unwrap() | fcntl::FdFlag::FD_CLOEXEC)
                .and_then(|f| fcntl::fcntl(fd, fcntl::FcntlArg::F_SETFD(f)));

            match result {
                Ok(_) => unsafe {
                    //FromRawFd::from_raw_fd(fd  )
                }
                Err( T ) => {

                }
            }
        }else {
            let b = env::var("WAYLAND_DISPLAY").map(Into::<PathBuf>::into) ;
                //.ok_or()?; ;
        } ; */
    }
}



#[repr(C)]

struct Display{
    ptr  :  * mut   wl_display
}

impl Display {
   //pub fn connect( )  -> Result<Display<> ,String> { }

    fn connect_from_unix_stream(unix_stream:UnixStream) -> Result<Display<>, &'static str> {
        if let Some(ptr) = unsafe{(wl_display_connect_to_fd( unix_stream.as_raw_fd()  ) as * mut wl_display).as_mut()}     {
            Ok(Display{ptr})
        }else {
            Err("")
        }
    }

}


impl<'a > Drop for Display< > {
    fn drop(&mut self) {
        unsafe { wl_display_disconnect( self.ptr  ) };
    }
}







/*
unsafe extern "C" fn global_fn(data: *mut c_void, wl_registry: *mut wayland_client::wl_registry, name: u32, interface: *const i8, version: u32) {
    let a = CStr::from_ptr(interface).to_str().unwrap();
    println!("{}", a);
    match a {
        "wl_compositor" => {
            (*(data as *mut wl)).compositor = wl_registry_bind(wl_registry, name, &wl_compositor_interface as *const wl_interface, 0) as *mut wl_compositor;
        }
        "wl_display" => {
            (*(data as *mut wl)).shell = wl_registry_bind(wl_registry, name, &wl_shell_interface as *const wl_interface, 0) as *mut wl_shell
        }
        _ => {}
    };
}

extern "C" fn global_remove_fn(data: *mut c_void, wl_registory: *mut wayland_client::wl_registry, name: u32) {}
 */




extern "C" fn dispatcher_c_fun(  data : *const ::std::os::raw::c_void,
                     proxy: *mut ::std::os::raw::c_void,
                     arg3: u32,
                     message : *const wl_message,
                     argument: *mut wl_argument  )  ->  c_int {
    0
}


trait T   {
    fn ee () ;
}

#[repr(C)]
struct A {
    b  : i32 ,
}
fn eee (){


}





#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use std::env;
    use std::io::Read;
    use std::path::Path;
    use std::ptr::null_mut;
    use wayland_util::A;
    // use nix::libc::{pthread_rwlockattr_setkind_np, timer_create};
   // use nix::sys::socket::SockaddrLike;



    use super::*;


    #[test]
    fn it() {
        let name= PathBuf::from (env!("WAYLAND_DISPLAY" ) );
        let name = if name.is_absolute() { name
        }else {
            PathBuf::from(env!("XDG_RUNTIME_DIR")).join(name)};
        let x = UnixStream::connect( name ).unwrap();


        let a = &Display::connect_from_unix_stream(x).unwrap();

        let x  =  unsafe { wl_proxy_get_listener(a.ptr as * mut wl_proxy ) }  ;
        x ;
           //*a ;

    }

    #[test]
    fn test_it(){






      /*  env::vars().for_each(|it | {
            println!("{} : {}" ,  it.0 , it.1 )
        } ) */

          // let a = Display::connect().unwrap();
      // println!("{}" ,  env!("WAYLAND_DEBUG") ) ;

      //  println!("{}" , env::var("WAYLAND_SOCKET").unwrap())




       // let x  = a.as_mut( ) as w l_display ;
    }
}
