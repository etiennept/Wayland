extern crate alloc;
extern crate core;


use alloc::rc::Rc;
use core::fmt::Arguments;

use std::convert::AsMut;
use std::ffi::{c_void, CStr, CString};
use std::io::Bytes;
use std::os::raw::c_char;
use std::os::unix::io::{AsFd, AsRawFd, FromRawFd, RawFd};


mod client;

mod protocol;


//const a: A<i32> = A{ to_argument:  &| i | {WlArgument{i } }, from_argument:  &| i |{ unsafe { i.i }} } ;

pub trait WlEnum where Self: Sized {
    fn from_u32(uint: u32) -> Self;
    fn to_u32(self) -> u32;
}


pub enum WlArgument {
    Int(i32),
    UInt(u32),
    Fixed(i32),
    String(*const c_char),
    Object(*mut c_void),
    NewId(*mut c_void),
    Array(*mut c_void),
    FD(i32),
}

impl WlArgument {
    fn to_CWlArgument(self) -> CWlArgument {
        match self {
            WlArgument::Int(i) => { CWlArgument { i } }
            WlArgument::UInt(u) => { CWlArgument { u } }
            WlArgument::Fixed(f) => { CWlArgument { f } }
            WlArgument::String(s) => { CWlArgument { s } }
            WlArgument::Object(o) => { CWlArgument { o } }
            WlArgument::NewId(n) => { CWlArgument { n } }
            WlArgument::Array(a) => { CWlArgument { a } }
            WlArgument::FD(f) => { CWlArgument { f } }
        }
    }
    unsafe fn from_CWlArgument(i: u8, argument: CWlArgument) -> Self {
        match i {
            b'i' => { WlArgument::Int(argument.i) }
            b'u' => { WlArgument::UInt(argument.u) }
            b'f' => { WlArgument::Fixed(argument.f) }
            b's' => { WlArgument::String(argument.s) }
            b'o' => { WlArgument::Object(argument.o) }
            b'n' => { WlArgument::NewId(argument.n) }
            b'a' => { WlArgument::NewId(argument.a) }
            b'h' => { WlArgument::FD(argument.h) }
            _ => { panic!("") }
        }
    }
}

macro_rules! argument {
    ( $name:ident , $enum_:path ,  $type_:ty ) => {
        fn $name (self) -> $type_   {
            if let $enum_(value) = self   {
                   value
            }  else { panic!("error type ")   }
        }
    };
}

impl WlArgument {
    argument!(get_int , WlArgument::Int, i32);
    argument!(get_uint, WlArgument::UInt, u32);
    argument!(get_fixed, WlArgument::Fixed, i32);
    argument!(get_string, WlArgument::String, * const c_char );
    argument!(get_object, WlArgument::Object, * mut c_void);
    argument!(get_newId, WlArgument::NewId, * mut c_void );
    argument!(get_array, WlArgument::Array, * mut c_void  );
    argument!(get_fd, WlArgument::FD, i32);
}
pub trait WlRequests {
    fn get_opcode (self) -> u32  ;
    fn from_opcode( opcode : u32 ) -> Self ;
}

pub trait WlEvents{
    fn get_opcode ( self  ) -> u32 ;
    fn from_opcode( opcode :  u32) -> Self ;
}



pub trait WlObject {
    fn get_interface()  -> WlInterface where Self: Sized;
  //  type Requests ;
    //type Events ;
}


/*
impl WlType for RawFd {
    unsafe fn to_argument(self) -> WlArgument {
        WlArgument{h : self }
    }
    unsafe fn from_argument(argument: WlArgument)  -> RawFd{
         argument.h
    }
} */


extern "C" {
    static wl_display_interface: WlInterface;
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WlMessage {
    #[doc = " Message name"]
    pub name: *const ::std::os::raw::c_char,
    #[doc = " Message signature"]
    pub signature: *const ::std::os::raw::c_char,
    #[doc = " Object argument interfaces"]
    pub types: *mut *const WlInterface,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WlInterface {
    #[doc = " Interface name"]
    pub name: *const ::std::os::raw::c_char,
    #[doc = " Interface version"]
    pub version: ::std::os::raw::c_int,
    #[doc = " Number of methods (requests)"]
    pub request_count: ::std::os::raw::c_int,
    #[doc = " Method (request) signatures"]
    pub requests: *const WlMessage,
    #[doc = " Number of events"]
    pub event_count: ::std::os::raw::c_int,
    #[doc = " Event signatures"]
    pub events: *const WlMessage,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WlArray {
    #[doc = " Array size"]
    pub size: ::std::os::raw::c_ulong,
    #[doc = " Allocated space"]
    pub alloc: ::std::os::raw::c_ulong,
    #[doc = " Array data"]
    pub data: *mut ::std::os::raw::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
union CWlArgument {
    #[doc = "< `int`"]
    i: i32,
    #[doc = "< `uint`"]
    u: u32,
    #[doc = "< `fixed`"]
    f: i32,
    #[doc = "< `string`"]
    s: *const ::std::os::raw::c_char,
    #[doc = "< `object`"]
    o: *mut std::os::raw::c_void,
    #[doc = "< `new_id`"]
    n: *mut std::os::raw::c_void,
    #[doc = "< `array`"]
    a: *mut std::os::raw::c_void,
    #[doc = "< `fd`"]
    h: i32,
}


#[cfg(test)]
mod tests {
    use core::ffi;
    use core::str::{from_utf8, from_utf8_unchecked};
    use std::ffi::c_void;
    use super::*;

    struct A {
        a: *const c_char,
    }


    #[test]
    fn it_works() {
        /*

            let a = "eeee".bytes().map( | it |{
                it as c_char  }).collect::<Vec<c_char>>().as_ptr() as * const c_char  ;
            let b = a.cast::<& [i8]>().to_vec()     ;
            println!( "{}" ,  b ) */
    }
}
