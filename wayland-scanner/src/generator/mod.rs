mod client;

use std::ops::Add;
use proc_macro2::{TokenStream, TokenTree};
use quote::quote;



use crate::parser::{Arg, Enum, Interface, Protocol};
use crate::parser::Type;


pub(crate) fn generate(protocol: Protocol)  -> TokenStream  {
    let token  = protocol.interfaces.into_iter().map(| interface| {
        generate_from_interface(interface )
    }).collect::<Vec< TokenStream>>() ;
    quote!{
        #(#token)*
    }
}

fn generate_from_interface(interface: Interface) -> TokenStream {
    let name = interface.name ;
    let rust_name =  to_rust_name(name.clone() ) ;
    let enums : Vec<TokenStream>  = interface.enums.map(|enum_|{
        generate_from_enum(   rust_name ,   enum_  )
    } ).collect() ;
    let static_interface =  name+"_interface" ;
    //let name_interface = n   ame.add("_interface" )  ;
    quote! {
        #(#enums)*
        pub struct #rust_name {
            ptr : * mut c_void
            
        }
        impl CWlOject for #rust_name {
            fn get_interface(){
                unsafe{
                    #static_interface
                }
            }
        }
    }
}
fn generate_from_enum( interface_name : String  ,   enum_: Enum) -> TokenStream {
    let mut enum_stream = Vec::<TokenStream>::new();
    let mut from_stream = Vec::<TokenStream>::new();
    let enum_name =  interface_name.add(to_rust_name(enum_.name).as_str());
    for entry in enum_.entries{
        let name = to_rust_name(entry.name);
        let value = entry.value;
        enum_stream.push( quote!{
            #name = #value ,
        }) ;
        from_stream.push( quote!{
            #value => #enum_name::#name ,
        })
    }
    quote! {
        #[repr(u32)]
        enum #enum_name{
            #enum_stream
        }
        impl WlEnum{
            fn from_u32 (value : u32 )->Self {
                match value {
                    #from_stream
                }
            }
        }
    }
}






fn arg_interface(interface: Option<String>) -> TokenStream {
    match interface {
        None => { quote! { T   } }
        Some(string) => { quote! { #string   } }
    }
}

enum Interface_call {
    Rust,
    C,
}

/*
fn interface_const_generator(interface: Interface, interface_call: Interface_call) -> TokenStream {
    let const_name = interface_to_const(interface.clone());
    match interface_call {
        Interface_call::Rust => {
            let name = interface.name;
            let version = interface.version;
            let method_count = interface.requests.len();
            let event_count = interface.events.len();
            let methods = message_to_struct_const(&interface.requests);
            let events = message_to_struct_const(&interface.events);
            quote! {
                pub const #const_name:wl_interface = wl_interface{
                    name : #name ,
                    version :#name ,
                    method_count : #method_count ,
                    methods : #methods  ,
                    event_count : #event_count ,
                    events : #events
                } ,
            }
        }
        Interface_call::C => {
            quote! {
                extern "C"{
                    pub static #const_name :  wl_interface;
                }
            }
        }
    }
}*/

/*
fn c_interface_to_const(interface: Interface) -> String { format!("{}_interface", interface.name) }

fn message_to_struct_const(messages: &Vec<Message>) -> TokenStream {
    if messages.is_empty() {
        quote! {null()}
    } else {
        let mut token = TokenStream::new();
        messages.iter().for_each(|message|{
            let name = message.name;
            let signature = to_signate(message);
            let type_token = arg_to_type(&message.clone().args) ;
            token = quote! {
                #token
                wl_message{
                    name : #name
                    signature : #signature
                    types : #type_token
                }.ptr()   ,
            }
        } )  ;
        quote! {
            [#token]
        }
    }
}




fn arg_to_type   (message_args  :&Vec<Arg>    ) -> TokenStream {
    let mut args = Vec::<Arg>::new();
    message_args.iter().for_each(|it| {
        match it.type_ {
            Type::Object | Type::NewId => {
                args.push(it.clone());
            }
            _ => {}
        };
    });
    if args.is_empty() {
        quote! { null_mut() }
    } else {
        let mut arg_token = TokenStream::new();
        args.iter().for_each(|it| {
            let mut a = TokenStream::new();
            if let Some(y) = it.clone().interface {
                a = format!("{}_interface", y) .parse::<TokenStream>().unwrap();
            } else {
                a = quote! {null::<wl_interface>() }
            };
            arg_token = quote! {
                        arg_token
                        #a,
                    }
        });
        quote! {
                    &mut [#arg_token].ptr()
                }
    }
}*/


/*

fn to_signate(message: Message) -> String {
    let mut string = String::new();
    if let Some(y) = message.since {
        string += y.as_str()
    }
    for arg in message.args {
        if arg.allow_null {
            string += "?"
        }
        string += match arg.type_ {
            Type::Int => { "i" }
            Type::Uint => { "u" }
            Type::Fixed => { "f" }
            Type::String => { "s" }
            Type::Object => { "o" }
            Type::NewId => { if arg.interface.is_none() { "sun" } else { "n" } }
            Type::Array => { "a" }
            Type::Fd => { "h" }
        };
    }
    string
}


fn to_type(arg: Arg) -> String {
    match arg.type_ {
        Type::Int => { "i32".to_string() }
        Type::Uint => { if let Some(y) = arg.enum_ { y } else { "u32".to_string() } }
        Type::Fixed => { "wl_fixed_t".to_string() }
        Type::String => { "*const ::std::os::raw::c_char".to_string() }
        Type::Object => { format!("*mut  {}", if let Some(y) = arg.interface { y } else { "::std::os::raw::c_void".to_string() }) }
        Type::NewId => { format!(" ") }
        Type::Array => { "*mut wl_array".to_string() }
        Type::Fd => { "i32".to_string() }
    }
}
 */
fn to_rust_name(name: String) -> String {
    let a = name.split("_");
    let mut value = "".to_string();
    for x in a {
        let a = x[0..1].to_uppercase() + (&x[1..]);
        value += a.as_str();
    }
    value
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::quote;
    use crate::generator::to_rust_name;

    #[test]
    fn name_test() {
        let a = to_rust_name("wl_display".to_string());

        println!("{}", a)
    }

    #[test]
    fn test() {
        let mut a = TokenStream::new();
        for i in 0..2 {
            let x = i.to_string();
            a = quote! {
                #a ee #x
             };
        }
        println!("{}", a)
    }
}