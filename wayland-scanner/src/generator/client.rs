use proc_macro2::TokenStream;
use quote::quote;

use crate::generator::to_rust_name;
use crate::parser::{Interface, Protocol, Stack};

fn generator (protocol  : Protocol ) -> TokenStream{
    let token  = protocol.interfaces.into_iter().map(  |it| {
         generate_interface(it)
    } ).collect::< Vec<TokenStream>>()   ;
    quote!{
        #(#token)*
    }
}
fn generate_interface( interface : Interface )-> TokenStream{


    let a = to_rust_name( interface.name );
    quote!{




    }
}