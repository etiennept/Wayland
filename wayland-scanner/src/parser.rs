use std::collections::HashSet;
use std::fs::{File, read};

use std::io::{BufRead, BufReader, Read};
use std::vec;
use quick_xml::events::{BytesStart, Event};
use quick_xml::events::Event::{CData, Empty, End, Eof, Start, Text};
use quick_xml::name::QName;
use quick_xml::Reader;
use crate::data::*;


fn parse_string(utf8: &[u8]) -> String {
    std::str::from_utf8(utf8).unwrap().to_string()
}

fn parse_type(utf8: &[u8]) -> Type {
    match utf8 {
        b"int" => Type::Int,
        b"uint" => Type::Uint,
        b"fixed" => Type::Fixed,
        b"string" => Type::String,
        b"object" => Type::Object,
        b"new_id" => Type::NewId,
        b"array" => Type::Array,
        b"fd" => Type::Fd,
        b"destructor" => Type::Destructor,
        e => panic!(""),
    }
}

fn get_event<'b, R: BufRead>(reader: &mut Reader<R>, buf: &'b mut Vec<u8>) -> quick_xml::events::Event<'b> {
    reader.read_event_into(buf).unwrap()
}

fn parse<B : BufRead>(reader : &mut Reader<B> ) -> Protocol {

    reader.trim_text(true);

    let mut protocol = Protocol {
        name: "".to_string(),
        interfaces: Vec::<Interface>::new(),
    };
    let mut a = true;
    loop {
        match get_event( reader, &mut vec!()) {
            Start(bytesStar) => {
                if (bytesStar.name().as_ref() == b"protocol") && a {
                    println!("eeeee");
                    protocol_parser(reader, &bytesStar, &mut protocol)
                } else {
                    panic!("")
                }
            }
            Text(e) => {
                panic!("")
            }
            CData(e) => {
                panic!("")
            }
            Empty(e) => {
                panic!("")
            }
            Eof => { break; }
            _ => {}
        }
    }
    protocol
}

fn protocol_parser<R: BufRead>(reader: &mut Reader<R>, bytesStart: &BytesStart, protocol: &mut Protocol) {
    bytesStart.attributes().for_each(|attr| {
        let attr = attr.unwrap();
        if attr.key.as_ref() == b"name" {
            protocol.name = parse_string(attr.value.as_ref())
        }
    });
    let mut buf = Vec::<u8>::new();
    loop {
        match get_event(reader, &mut buf) {
            Start(bytesStart) => {
                match bytesStart.name().as_ref() {
                    b"copyright" => {}
                    b"description" => {}
                    b"interface" => {
                        protocol.interfaces.push(interface_parser(reader, &bytesStart))
                    }
                    _ => {
                        panic!("")
                    }
                }
            }
            End(bytesEnd) => {
                if bytesEnd.name() == bytesStart.name() {
                    break;
                }
            }
            _ => {}
        }
    }
}

fn interface_parser<R: BufRead>(reader: &mut Reader<R>, bytesStart: &BytesStart) -> Interface {
    let mut interface = Interface {
        name: "".to_string(),
        version: "".to_string(),
        enums: vec!(),
        requests: vec!(),
        event: vec!(),
    };
    bytesStart.attributes().for_each(|it| {
        let it = it.unwrap();
        let a = parse_string(it.value.as_ref());
        match it.key.as_ref() {
            b"name" => {
                interface.name = a
            }
            b"version" => {
                interface.version = a
            }
            _ => { panic!("attribute not valid") }
        }
    });
    let buf = &mut vec!();
    loop {
        match get_event(reader, buf) {
            Start(bytesStart) => {
                match bytesStart.name().as_ref() {
                    b"description" => {}
                    b"enum" => {
                        interface.enums.push(enum_parser(reader, &bytesStart));
                    }
                    b"event" => {
                        interface.event.push(callable_parser(reader, &bytesStart));
                    }
                    b"request" => {
                        interface.requests.push(callable_parser(reader, &bytesStart))
                    }
                    _ => {
                        panic!("")
                    }
                }
            }
            End(bytesEnd) => {
                if bytesEnd.name() == bytesStart.name() {
                    break;
                }
            }
            _ => {}
        }
    }
    interface
}


fn callable_parser<R: BufRead>(reader: &mut Reader<R>, bytesStart: &BytesStart) -> Callable {
    let mut callable = Callable {
        name: "".to_string(),
        since: None,
        types: None,
        args: vec![],
    };
    bytesStart.attributes().for_each(|it| {
        let it = it.unwrap();
        let value = it.value.as_ref();
        match it.key.as_ref() {
            b"name" => {
                callable.name = parse_string(value);
            }
            b"since" => {
                callable.since = Some(parse_string(value));
            }
            b"type" => {
                callable.types = Some(parse_type(value))
            }
            _ => { panic!("attribute not valid") }
        }
    });
    let buf = &mut vec!();
    loop {
        match get_event(reader, buf) {
            Empty(bytesStart) => {
                match bytesStart.name().as_ref() {
                    b"arg" => {
                        callable.args.push(arg_parser(&bytesStart))
                    }
                    _ => {}
                }
            }
            Start(bytesStart) => {
                match bytesStart.as_ref() {
                    b"description" => {}
                    _ => {}
                };
            }
            End(bytesEnd) => {
                if bytesEnd.name() == bytesStart.name() {
                    break;
                }
            }
            _ => {  }
        }
    }
    callable
}

fn arg_parser(bytesStart: &BytesStart) -> Arg {
    let mut arg = Arg {
        name: "".to_string(),
        interface: None,
        type_: Type::Object,
        enum_: None,
        allow_null: false,
    };

    bytesStart.attributes().for_each(|it| {
        let it = it.unwrap();
        let a = "eee";
        let value = it.value.as_ref();
        match it.key.as_ref() {
            b"interface" => { arg.interface = Some(parse_string(value)) }
            b"name" => { arg.name = parse_string(value) }
            b"summary" => {}

            b"allow-null" => {
                if { value == b"true" } {
                    arg.allow_null = true
                }
            }
            b"enum" => { arg.enum_ = Some(parse_string(value)) }
            b"type" => { arg.type_ = parse_type(value) }
            _ => {}
        };
    }
    );
    arg
}


fn enum_parser<R: BufRead>(reader: &mut Reader<R>, bytesStart: &BytesStart) -> Enum {
    let mut enum_ = Enum {
        name: "".to_string(),
        since : None,
        bitfield : None,
        entries: vec![],
    };

    bytesStart.attributes().for_each(|it| {
        let it = it.unwrap();
        let value = parse_string(it.value.as_ref());
        match it.key.as_ref() {
            b"since" => {
                enum_.since = Some(value)
            }
            b"name" => {
                enum_.name = value
            }
            b"bitfield" => {
                enum_.bitfield = Some(value)
            }
            _ => panic!("")
        }
    });


    let b = &mut Vec::new();
    loop {
        match get_event(reader, b) {
            Empty(bytesStart) => {
                match bytesStart.name().as_ref() {
                    b"entry" => {
                        enum_.entries.push(entry_parser(&bytesStart))
                    }
                    _ => {
                        panic!("")
                    }
                }
            }
            Start(bytesStart) => {
                match bytesStart.as_ref() {
                    b"description" => {}
                    _ => {}
                };
            }
            End(bytesEnd) => {
                if bytesEnd.name() == bytesStart.name() {
                    break;
                }
            }

            _ => {}
        }
    }
    enum_
}

fn entry_parser(bytesStart: &BytesStart) -> Entry {
    let mut name = None as Option<String>;
    let mut value = None as Option<String>;
    bytesStart.attributes().for_each(|it| {
        let it = it.unwrap();

        let a = parse_string(it.value.as_ref());
        match it.key.as_ref() {
            b"summary" => {}
            b"since" => {}
            b"name" => {
                name = Some(a)
            }
            b"value" => {
                value = Some(a)
            }
            _ => {}
        }
    });
    Entry {
        name: name.unwrap(),
        value: value.unwrap(),
        summary: None,
        since: None,
    }
    /*
      */
}


fn get_file(file_name: &str) -> Reader<BufReader<File>> {
    Reader::from_file(file_name).unwrap()
}

fn fff(name: QName) {
    match name.as_ref() {
        b"entry" => {}
        b"description" => {}
        b"arg" => {}
        _ => {}
    };
}

#[cfg(test)]
mod tests {
    use quick_xml::name::QName;
    use quick_xml::Reader;
    use super::*;

    #[test]
    fn rrr() {
        let mut reader = Reader::from_file("/home/etienne/wayland-1.21.0/protocol/wayland.xml").unwrap();
        let mut a = HashSet::<String>::new();

        loop {
            match get_event(&mut reader, &mut vec!()) {
                Start(bytesStart) => {}
                Empty(bytesStart) => {
                    match bytesStart.name().as_ref() {
                        b"arg" => {
                            bytesStart.attributes().for_each(|it| {
                                a.insert(parse_string(it.unwrap().key.as_ref()).to_string());
                            })
                        }
                        _ => {}
                    }
                }
                Eof => {
                    break;
                }
                _ => {}
            }
        }
        a.iter().for_each(
            |it| {
                println!("{}", it)
            }
        );
    }

    #[test]
    fn eee() {
        let a = &mut Reader::from_file("/home/etienne/wayland-1.21.0/protocol/wayland.xml"  ).unwrap();
        parse(a );


        /*let mut  a = Protocol

             interfaces : Vec::new()
         }; */
    }
}
