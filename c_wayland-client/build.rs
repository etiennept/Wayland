use std::borrow::Cow;
use std::env;
use std::path::PathBuf;
use bindgen::Builder;
use bindgen::builder;
use pkg_config::{Library, probe_library};

fn main(){
    let library = probe_library("wayland-client").unwrap();
    Builder::default().generate_inline_functions(true)
        .header(library.include_paths.first().unwrap().join("wayland-client.h").to_string_lossy() )
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_file(library.include_paths.first().unwrap().join(  "wayland-client-core.h").to_string_lossy())
       // .allowlist_file(library.include_paths.first().unwrap().join(  "wayland-client-protocol.h").to_string_lossy())
        .generate().unwrap()
        .write_to_file( PathBuf::from(env::var("OUT_DIR").unwrap()).join(format!("wayland_client.rs" )) ).unwrap() ;
}
