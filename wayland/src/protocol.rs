use std::ffi::CStr;
use std::fmt::Error;
use std::os::unix::io::{AsFd, AsRawFd, FromRawFd, RawFd};
use std::rc::Rc;
use c_wayland::util::wl_argument;


/*
pub struct Fixed {}

pub trait Interface  {
    const NAME  : String ;
    const VERSION : String ;
    type Requests : Messages;
    type Events : Messages;
}



#[derive(Debug , PartialEq , Eq ,   )]
pub  enum AllowNull {
    Allow,
    NotAllow,
}
#[derive(Debug , PartialEq , Eq   )]
pub enum ArgumentType <   > {
    Int,
    Uint,
    Enumeration  ,
    String(AllowNull),
    Object(AllowNull),
    Fixed,
    Array,
    Fd,
}
 */










/*
impl MessageDesc for Test{
    const NAME: &'static str = "";
    const VERSION: &'static str = "";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const IS_DESTRUCTOR: bool = false;
}*/






/*
#[derive(Debug, Clone)]
pub struct MESSAGE {
    name: String,
    version : String,
    types: Vec<Box<Interface>>,
}



#[derive(Debug, Clone, )]
pub struct Interface {
    pub name: String,
    pub version: String,
    pub methods: Vec<MESSAGE>,
    pub events: Vec<MESSAGE>,
}*/