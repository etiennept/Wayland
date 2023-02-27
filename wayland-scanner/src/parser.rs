use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Debug;

use std::fs::{File, read};
use std::hash::Hash;
use std::io::{BufRead, BufReader, Read};
use std::rc::Rc;

use std::vec;

use syn::__private::bool;
use syn::Attribute;
use syn::ReturnType::Default;
use syn::token::In;
use xml::attribute::OwnedAttribute;
use xml::EventReader;
use xml::name::OwnedName;
use xml::reader::XmlEvent;
use crate::*;



/*
enum Message {
    Request ,
    Event
} */





#[derive(Debug)]
pub struct Protocol {
    pub name: Rc<str>,
    pub interfaces: Vec<Interface>,
}

#[derive(Debug)]
pub struct Interface {
    pub name: Rc<str>,
    pub version: Rc<str>,
    pub requests : Vec<Message> ,
    pub events : Vec<Message> ,
    pub enums  : Vec<Enum> ,
}

#[derive(Debug)]
pub struct Enum {
   pub name: Rc<str>,
   pub since: Option<Rc<str>>,
   pub bitfield: Option<Rc<str>>,
   pub entries: Vec<Entry>,
}

#[derive(Debug)]
pub struct Message {
    pub name: Rc<str>,
    pub since: Option<Rc<str>>,
    pub is_destructor: bool,
    pub args: Vec<Arg>,
}


#[derive(Debug)]
pub struct Arg {
    pub name: Rc<str>,
    pub interface: Option<Rc<str>>,
    pub type_: Type,
    pub allow_null: bool,
    pub enum_: Option<Rc<str>>,
}

#[derive(Debug)]
pub struct Entry {
    pub name: Rc<str>,
    pub value: Rc<str>,
    pub summary: Option<Rc<str>>,
    pub since: Option<Rc<str>>,
}





impl<R: AsRef<str>> From<R> for Type {
    fn from(string: R) -> Self {
        match string.as_ref() {
            "int" => Type::Int,
            "uint" => Type::Uint,
            "fixed" => Type::Fixed,
            "string" => Type::String,
            "object" => Type::Object,
            "new_id" => Type::NewId,
            "array" => Type::Array,
            "fd" => Type::Fd,
            _ => panic!("type not recognized")
        }
    }
}


#[derive(Debug, Eq, PartialEq , Clone)]
pub enum Type {
    Int,
    Uint,
    Fixed,
    String,
    Object,
    NewId,
    Array,
    Fd,
}




impl Protocol {
    fn new(attributes: BTreeMap<String, String>) -> Self {
        Protocol {
            name: Rc::from(attributes.get("name").unwrap().to_string()),
            interfaces: vec![],
        }
    }
}


impl Interface {
    fn new(attributes: BTreeMap<String, String>) -> Self {
        Interface {
            name: Rc::from(attributes.get("name").unwrap().to_string()),
            version: Rc::from(attributes.get("version").unwrap().to_string()),
            requests: vec![],
            events: vec![],
            enums: vec![],
        }
    }
}

impl Enum {
    fn new(attributes: BTreeMap<String, String>) -> Self {
        Enum {
            name: Rc::from(attributes.get("name").unwrap().to_string()),
            since: None,
            bitfield: None,
            entries: vec![],
        }
    }
}

impl Message {
    fn new(attributes: BTreeMap<String, String>) -> Self {
        Message {
            name: Rc::from(attributes.get("name").unwrap().to_string()),
            since: None,
            is_destructor: attributes.get("type") == Some(&"destructor".to_string()),
            args: vec![],
        }
    }

}


impl Arg {
    fn new(attributes: BTreeMap<String, String>) -> Self {
        Arg {
            name: Rc::from(attributes.get("name").unwrap().to_string()),
            interface: attributes.get("").map(|x| { Rc::from(x.to_string()) }),
            type_: Type::from(attributes.get("type").unwrap()),
            allow_null: attributes.get("allow-null") == Some(&"true".to_string()),
            enum_: attributes.get("").map(|x| { Rc::from(x.to_string()) }),
        }
    }
}

impl Entry {
    fn new(attributes: BTreeMap<String, String>) -> Self {
        Entry {
            name: Rc::from(attributes.get("name").unwrap().to_string()),
            value: Rc::from(attributes.get("value").unwrap().to_string()),
            summary: attributes.get("").map(|x| { Rc::from(x.to_string()) }),
            since: attributes.get("").map(|x| { Rc::from(x.to_string()) }),
        }
    }
}



macro_rules! rule {
    (  $type_:expr , $text:expr ,  $($typeName:ident ),+ ) => {
        match $type_ {
           $(  NodeType::$typeName { texts  , ..  }=> {  texts.push( $text )   } )+
            _=>{  panic!("eee")}
        }

    };
}

enum NodeType {
    Protocol { protocol: Protocol },
    CopyRight { texts: Vec<String> },
    Description { summery: Option<String>, texts: Vec<String> },
    Interface { interface: Interface },
    Enum { enum_: Enum },
    Request { message: Message },
    Event { message: Message },
    Entry { entry: Entry },
    Arg { arg: Arg },
}
impl NodeType {
    fn new(name: String, attributes: BTreeMap<String, String>) -> Self {
        match name.as_str() {
            "protocol" => {
                NodeType::Protocol {
                    protocol: Protocol::new(attributes)
                }
            }
            "copyright" => {
                NodeType::CopyRight { texts: vec![] }
            }
            "description" => {
                NodeType::Description { summery: None, texts: vec![] }
            }
            "interface" => {
                NodeType::Interface { interface: Interface::new(attributes) }
            }
            "enum" => {
                NodeType::Enum {
                    enum_: Enum::new(attributes)
                }
            }
            "request" => {
                NodeType::Request {
                    message: Message::new(attributes),
                }
            }
            "event" => {
                NodeType::Event {
                    message: Message::new(attributes)
                }
            }
            "entry" => {
                NodeType::Entry {
                    entry: Entry::new(attributes)
                }
            }
            "arg" => {
                NodeType::Arg {
                    arg: Arg::new(attributes)
                }
            }
            _ => { panic!("") }
        }
    }
}

impl ToString for NodeType {
    fn to_string(&self) -> String {
        match self {
            NodeType::Protocol { .. } => { "protocol" }
            NodeType::CopyRight { .. } => { "copyright" }
            NodeType::Description { .. } => { "description" }
            NodeType::Interface { .. } => { "interface" }
            NodeType::Enum { .. } => { "enum" }
            NodeType::Request { .. } => { "request" }
            NodeType::Event { .. } => { "event" }
            NodeType::Entry { .. } => { "entry" }
            NodeType::Arg { .. } => { "arg" }
        }.to_string()
    }
}


use std::ops::Deref;

impl Message {
    fn add_arg  (& mut self , value  :NodeType ) {
        match value {
            NodeType::Description { .. } => {}
            NodeType::Arg { arg } => {
                self.args.push(arg)
            }
            _=>{
                panic!("eeee")
            }
        }
    }
}


pub(crate) struct Stack {
    vec: Vec<NodeType>,
    value : Option<Rc<Protocol>>
}

impl Stack {
    pub(crate) fn new() -> Self {
        Stack {
            vec: vec![],
            value : None
        }
    }
    pub(crate) fn push(&mut self, name: String, attributes: BTreeMap<String, String>) {
        self.vec.push( NodeType::new(name , attributes) ) ;
    }
    pub(crate) fn text(&mut self, text: String) {
        rule!(self.vec.last_mut().unwrap() , text  , CopyRight , Description )
    }
    pub(crate) fn pop(&mut self, name: String) {
        let value = self.vec.pop().unwrap();
        if value.to_string() != name {
            panic!("")
        }
        if let Some(last) = self.vec.last_mut() {
            match last {
                NodeType::Protocol { protocol } => {
                    match  value {
                        NodeType::CopyRight { .. } => {}
                        NodeType::Interface { interface } => {
                            protocol.interfaces.push( interface)
                        }
                        _=> {
                            panic!("")
                        }
                    }
                }
                NodeType::Interface { interface } => {
                    match value {
                        NodeType::Description { .. } => {}
                        NodeType::Enum { enum_ } => {
                            interface.enums.push(enum_)
                        }
                        NodeType::Request { message } => {
                            interface.requests.push(message)
                        }
                        NodeType::Event { message } => {
                            interface.events.push(  message )
                        }
                        _=> { panic!("")}
                    }

                }
                NodeType::Enum { enum_ } => {
                    match value {
                        NodeType::Description { ..}=> {

                        }
                        NodeType::Entry { entry} =>{
                            enum_.entries.push(entry)
                        }
                        _=>{ panic!("")}
                    }

                }
                NodeType::Request {  message } => {
                    message.add_arg(value)
                }
                NodeType::Event { message } => {
                    message.add_arg(value)
                }
                _=>{
                    panic!("")
                }
            }

        } else {
            match value {
                NodeType::Protocol { protocol } => {
                    self.value  = Some( Rc::new(protocol))
                }
                _=> {
                    panic!("")
                }

            }

        }
    }
    pub(crate) fn give (& self) -> Rc<Protocol> {
        self.value.as_ref().unwrap().clone()
    }
}


#[cfg(test)]
mod tests {
    use std::borrow::{Borrow, BorrowMut};
    use std::cell::{Ref, RefCell};
    use std::fs::File;
    use std::io::BufReader;
    use std::rc::Rc;
    use xml::EventReader;
    use xml::reader::XmlEvent;
    use crate::parser::parser;


    #[test]
    fn test_parser() {
        /* let mut a = E {
             vec: vec![],
         };
         a.vec.push("eeee".to_string());
  */



        // let a= Reader::from_file("/home/etienne/wayland-1.21.0/protocol/wayland.xml"  ).unwrap() ;
    }


    #[test]
    fn eee() {

    }
}
