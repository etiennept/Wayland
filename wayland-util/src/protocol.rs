
use std::os::unix::prelude::RawFd;



#[derive(Debug , PartialEq , Eq , Clone ,  )]
pub  enum AllowNull {
    Allow,
    NotAllow,
}
#[derive(Debug , PartialEq , Eq , Clone ,  )]
pub enum ArgumentType {
    Int,
    Uint,
    String(AllowNull),
    Object(AllowNull),
    NewId,
    Fixed,
    Array,
    Fd,
}

pub trait WL_Enum  {
    fn from_u32  (  int : u32  ) -> Self   ;
    fn as_u32  ( &self ) -> u32 ;
}





pub trait InterfaceDesc{
    const NAME  : &'static str;
    const VERSION : &'static str;
     type METHODS  : MessagesGroup +'static;
    type EVENTS  : MessagesGroup +'static ;
}


pub  trait  MessagesGroup {
    //const MESSAGE:  [dyn MessageDesc] ;
    //fn get_opcode  () -> 1
}


pub trait MessageDesc {
    const NAME  : &'static str;
    const VERSION : &'static str;
    const ARGUMENTS  : &'static [ArgumentType ] ;
    const IS_DESTRUCTOR : bool ;
}
enum Test {
    A ,
    B

}

impl MessageDesc for Test{
    const NAME: &'static str = "";
    const VERSION: &'static str = "";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const IS_DESTRUCTOR: bool = false;
}






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