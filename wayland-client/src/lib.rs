use std::ffi::{c_void, CStr, CString};
use std::ptr::null;
use c_wayland_client::{wl_display, wl_display_connect, wl_display_disconnect,
                       wl_display_dispatch, wl_display_roundtrip, wl_registry_listener,
                       wl_registry, wl_registry_bind, wl_compositor, wl_interface, wl_compositor_interface, wl_shell, wl_shell_interface};


pub(crate) struct Display {
    wl_display: *mut wl_display,
}

impl Display {
    pub fn connect(value: Option<String>) -> Result<Display, String> {
        let b = match value {
            None => { null() }
            Some(y) => unsafe { CString::new(y).unwrap().as_ptr() }
        };
        let a = unsafe { wl_display_connect(b) };
        if a.is_null() {
            Err("Connection established".to_string())
        } else {
            Ok(Display { wl_display: a })
        }
    }
    pub fn dispatch(&self) -> bool {
        unsafe { wl_display_dispatch(self.wl_display) == 1 }
    }
    pub fn round_trip(&self) {
        unsafe { wl_display_roundtrip(self.wl_display) };
    }
    pub fn get_registry(&self) -> *mut wl_registry {
        unsafe { wayland_client::wl_display_get_registry(self.wl_display) }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe { wl_display_disconnect(self.wl_display) };
    }
}

//#[repr(C)]
struct wl {
    compositor: *mut wl_compositor,
    shell: *mut wl_shell,

}

unsafe extern "C" fn global_fn(data: *mut c_void, wl_registry: *mut wayland_client::wl_registry, name: u32, interface: *const i8, version: u32) {
    let a = CStr::from_ptr(interface).to_str().unwrap();
    println!("{}", a);
    match a {
        "wl_compositor" => {
            (*(data as *mut wl)).compositor = wl_registry_bind(wl_registry, name, &wl_compositor_interface as *const wl_interface, 0) as *mut wl_compositor;
        }
        "wl_display" => {
            (*(data as *mut wl)).shell = wl_registry_bind(wl_registry, name, &wl_shell_interface as *const wl_interface, 0) as *mut wl_shell
        }
        _ => {}
    };
}

extern "C" fn global_remove_fn(data: *mut c_void, wl_registory: *mut wayland_client::wl_registry, name: u32) {}

#[cfg(test)]
mod tests {
    use std::ptr::null_mut;
    use c_wayland_client::wl_registry_add_listener;
    use wayland_client::wl_registry_add_listener;
    use super::*;

    #[test]
    fn it() {
        let a = Display::connect(None).unwrap();

        let x = &mut wl_registry_listener {
            global: Some(global_fn),
            global_remove: Some(global_remove_fn),
        } as *mut wl_registry_listener;
        let r = &mut wl {
            compositor: null_mut(),
            shell: null_mut(),
        };

        let t = a.get_registry();
        unsafe {
            wl_registry_add_listener(t, x, (r as *mut wl) as *mut c_void);
        }
        a.dispatch();
        a.round_trip();
        println!("{}", r.shell.is_null());
    }
}
