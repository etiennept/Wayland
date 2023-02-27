use c_wayland::client::{wl_proxy, wl_proxy_create, wl_proxy_destroy};
use c_wayland::util::wl_interface;
mod c ;


struct Proxy {

}










impl  Drop for  Proxy  {
    fn drop(&mut self) {
        // unsafe {wl_proxy_destroy( self.ptr )}
    }
}


impl  Proxy  {
    /*fn create<T : Interface> ( wl_proxy :*mut wl_proxy ) ->  Self{
        let a =  unsafe {
           // (wl_proxy_create (wl_proxy  , null()) as * mut wl_proxy  ).as_mut()
        }.unwrap() ;
        Proxy{
            ptr : a
        }
    } */


}
trait Interface {
    const INTERFACE: & 'static wl_interface ;

}