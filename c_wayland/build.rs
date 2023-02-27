use std::borrow::Cow;
use std::env;
use std::path::{Path, PathBuf};
use bindgen::Builder;
use bindgen::builder;
use pkg_config::{Library, probe_library};

macro_rules! eee {
($path:expr ,  $header:expr,   $writer:expr ) => {
    let path_header  =  $path.clone().join($header) ;
    let  header  = path_header.to_string_lossy();
    Builder::default().header(header.clone())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_file( header)
        .generate().unwrap()
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join( $writer)).unwrap();
};
($path:expr ,  $header:expr,  $header_block:expr  ,  $writer:expr )   =>{
    let path_header = $path.clone().join($header) ;
    let header = path_header.to_string_lossy() ;
    Builder::default().header(header.clone())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_file( header)
        .blocklist_file($path.clone().join("wayland-version.h").to_string_lossy( ) )
        .blocklist_file($path.clone().join($header_block).to_string_lossy() )
        .generate().unwrap()
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join(format!($writer))).unwrap();
}
}


fn main() {


    //.allowlist_file(library.include_paths.first().unwrap().join(  "wayland-client-core.h").to_string_lossy())
    //.allowlist_file(library.include_paths.first().unwrap().join(  "wayland-client-protocol.h").to_string_lossy())



    if std::env::var_os("CARGO_FEATURE_CLIENT").is_some() {
        let library = probe_library("wayland-client").unwrap();
        let path = library.include_paths.first().unwrap();
        eee!( path , "wayland-util.h" , "wayland_util.rs"  ) ;
        eee!( path , "wayland-client-core.h" ,"wayland-util.h", "wayland_client.rs"  ) ;
    }


   // eee!( path , "wayland-client-protocol.h" ,   " wayland_client_protocol.rs") ;

    //eee!( path , "wayland_ ")





    /*let a = Builder::default().generate_inline_functions(true)
        .header(path.clone().join("wayland-client.h").to_string_lossy())
        //.header("/home/etienne/wayland-1.21.0/protocol/wayland.c")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_file(library.include_paths.first().unwrap().join("wayland-client-core.h").to_string_lossy())
        //.allowlist_file(library.include_paths.first().unwrap().join(  "wayland-client-protocol.h").to_string_lossy())
        .allowlist_file(path.clone().join("wayland-util.h").to_string_lossy())
        .blocklist_file(path.clone().join("wayland-util.h").to_string_lossy())

        //.allowlist_file("/home/etienne/wayland-1.21.0/protocol/wayland.c")
        .generate().unwrap()
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join(format!("wayland_client.rs"))).unwrap(); */
}
