use std::env::args;
use std::fmt::format;

use std::string::String;
use proc_macro2::{LexError, TokenStream};
use quote::quote;


use crate::*;
// use crate::parser::parse;

/*
fn generator(file_name: &str) {
    //let data = Vec::new();

    //a.interfaces
}
fn enum_builder(  enum_ : Enum ) -> TokenStream{
    let  mut tokenEnum = TokenStream::new() ;
    let mut from_u32Enum = TokenStream::new() ;
    let name  =  enum_.name ;

    for entry in enum_.entries.iter() {
        let arg_name = entry.name.to_uppercase() ;
        /*let value = entry.value ;
        tokenEnum = quote!{
            #tokenEnum
            #arg_name = #value ,
        } ;
        from_u32Enum  = quote!{
            #from_u32Enum
            #value => Ok(#name::#arg_name),
        }; */


    }


    quote!{
        #[repr(u32)]
        pub enum #name {
            #tokenEnum
        }
        impl Wl_Enum for #name {
            fn from_u32(int : u32 ) -> Resulf<#name,()>{
                match  int {
                    #from_u32Enum
                    _=> { Err(())}
                }
            }
            fn as_32 (&self)-> u32 { self as u32 }
        }
    }
}

fn arg_null  (allow_null : bool , token  :TokenStream  ) -> TokenStream {
    match allow_null {
        true => {quote!{ Option <#token > }   }
        false => {  token  }
    }
}
fn arg_interface(interface : Option<String> ) -> TokenStream {
    match interface {
        None => { quote!{ T   }   }
        Some( string  ) => { quote!{ #string   }}
    }
}




fn client_interface_generator(interface : Interface ) -> TokenStream{

    let name = interface.name.split("_") .last().unwrap() ;
    let version = interface.version ;


    let token_request = TokenStream::new() ;
    for request  in interface.requests {
        let token_arg = TokenStream::new() ;
        for arg in  request.args  {
            match arg.type_{
                Type::Int => {
                    quote!{i32}
                }
                Type::Uint => {
                    match arg.enum_ {
                        None => {quote!{ u32  } }
                        Some(name) => {quote!{ #name }}
                        }
                    }
                Type::Fixed => {quote!{}}
                Type::String  => {
                    arg_null(arg.allow_null , quote!{&str } )
                }
                Type::Object => {
                    match  arg.interface {
                        None => {

                        }
                        Some(b ) => {}
                    }
                    quote!{

                    }


                }
                Type::NewId => {

                    match  arg.interface  {
                        None => { }
                        Some( b  ) => {}
                    }
                    TokenStream::new()
                }
                Type::Array => {quote!{&[u8]}   } ,
                Type::Fd => {quote!{


                } }
            };
        }
        quote!{
            #token_request

            fn (){ }
        }  ;

    }


    quote! {
        struct #name {

        }
         impl #name  {

        }
    }


}






pub(crate) struct Build {
    //protocol: Protocol,
}

impl Build {
    /*fn parser(file_name: &str) -> Build {
        Build {
            protocol:  //parse(&mut Reader::from_file(file_name).unwrap())
        }
    } */

    fn generate_client(self) {
       // self.protocol.interfaces;
    }
    fn generate_server(self) {

    }
}

fn interface_struct_generator(interface: Interface) -> TokenStream {
    let name = interface.name;
    quote! {
        #[repr(C)]
        pub struct #name{
             _unused: [u8; 0],
        }
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


fn enums_to_generator(data: &mut Vec<String>, interface: Interface) -> TokenStream {
    let mut token = TokenStream::new();

    for it in interface.enums {
        let name = format!("{}_{}", interface.name, it.name);
        let mut token_entries = TokenStream::new();
        for entry in it.entries {
            let entry_name = format!("{}_{}", name, entry.name);
            let value = entry.value;
            token_entries = quote! {
                #token_entries
                #entry_name : #name = #value;
            }
        }
        token = quote! {
            #token
            pub type #name = u32 ;
            #token_entries
        }
    };
    token
}


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

fn eee (){

}

*/


#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::quote;


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