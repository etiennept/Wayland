extern crate core;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;
use xml::EventReader;
use xml::reader::XmlEvent;
use crate::parser::{Protocol, Stack};


mod parser;
mod generator;



fn parser(filename: &str) -> Rc<Protocol>{
    let file = File::open(filename).unwrap();
    let file = BufReader::new(file);
    let mut parser = EventReader::new(file);
    let mut a = Stack::new();
    for e in parser {
        match e.unwrap() {
            XmlEvent::StartElement { name, attributes, namespace } => {
                let attributes = attributes.iter().map(|x| { (x.clone().name.to_string(), x.clone().value) }).collect::<BTreeMap<String, String>>();
                a.push(name.local_name, attributes)
            }
            XmlEvent::EndElement { name } => {
                a.pop(name.local_name)
            }
            XmlEvent::Characters(name) => {
                a.text(name)
            }
            _ => {}
        }
    }
    Rc::new(a.give() )
}

fn generator (filename: &str ){
    let x = parser(filename );
    for interface in x.interfaces.iter() {
        println!("  {} " ,  interface.name) ;

        for  enum_ in interface.enums  {
            println!("{}" ,  enum_.name)
        }
        for event  in  interface.events{
            println!( "{}" , event.name)
        }

        for request  in interface.requests {
            println!("{}", request.name )
        }


    }



}



#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::collections::HashSet;
    use std::fs::read;
    use std::os::raw::c_char;
    use super::*;



    #[test]
    fn it_works() {

    }

    #[test]
    fn eee() {}
}
