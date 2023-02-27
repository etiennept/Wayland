use std::io::{Bytes, IoSlice, IoSliceMut};
use std::os::unix::io::{AsFd, AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::os::unix::net::UnixStream;
use nix::cmsg_space;

use nix::{ sys::socket , Result as NixResult };
use nix::sys::socket::{ControlMessage, ControlMessageOwned};
use nix::unistd::{close, pipe};



/*
fn read() -> Vec<u8> {
    let mut a = cmsg_space!([ RawFd ; 4096] );
    let bytes = &mut vec![u8::default(); 1];

    let bytes = unsafe {
        ::std::slice::from_raw_parts_mut(bytes.as_ptr() as *mut u8, 1024)
    };
    let message = socket::recvmsg::<()>(self.fd,
                                        &mut [IoSliceMut::new(bytes), ],
                                        Some(&mut a),
                                        socket::MsgFlags::MSG_DONTWAIT | socket::MsgFlags::MSG_CMSG_CLOEXEC | socket::MsgFlags::MSG_NOSIGNAL,
    ).unwrap();
    let mut byteMessage = Vec::<u8>::new();
    for i in 0..message.bytes {
        byteMessage.push(bytes[i])
    }

    /*let t = message.cmsgs().flat_map(|it| {
         match it {
             ControlMessageOwned::ScmRights(y) => { y }
             _ => Vec::new()
         }
     });
     for (a  ,i) in   t.zip( fds.iter_mut()  ){
         *i =a
     } ; */
    byteMessage
} */


fn  receive<T : AsRawFd > (socket :T, buffer: &mut [u8], fds: &mut [RawFd]) -> NixResult<(usize, i32)> {
    let mut cmsg = cmsg_space!([RawFd; 4096]) ;

    let msg = socket::recvmsg::<()>(
        socket.as_raw_fd(),
        &mut [IoSliceMut::new(buffer)],
        Some(&mut cmsg),
        socket::MsgFlags::MSG_DONTWAIT
            | socket::MsgFlags::MSG_CMSG_CLOEXEC
            | socket::MsgFlags::MSG_NOSIGNAL,
    )?;

    let received_fds = msg.cmsgs().flat_map(|cmsg| match cmsg {
        ControlMessageOwned::ScmRights(s) => s,
        _ => Vec::new(),
    });

    let mut fd_count = 0;

    for (fd, place) in received_fds.zip(fds.iter_mut()) {
        fd_count += 1 ;
        *place = fd;
    }

    Ok((msg.bytes, fd_count))


}


fn send <T :AsRawFd> (socket : T , bytes: &[u8], fds  : &[RawFd]) -> usize {
    let cmsgs = &[ControlMessage::ScmRights(fds)]  ;
    socket::sendmsg::<()>(  socket.as_raw_fd()  , &[IoSlice::new(bytes)],
                           if !fds.is_empty() { cmsgs } else {&[]},
                          socket::MsgFlags::MSG_DONTWAIT | socket::MsgFlags::MSG_NOSIGNAL,
                          None).unwrap()
}




macro_rules! Buffer_impl{
    (  ) => {};
}

pub struct BufferedSocket <  T :AsRawFd> {
    socket: T  ,
    input_buffer: Buffer ,
    output_buffer: Buffer ,
}


pub(crate) struct Buffer {
   pub(crate) data: Vec<u32 >,
   pub(crate) offset: Vec<RawFd> ,
}



/*
impl Buffer<'_ >{
    fn new(data_size: usize, offset_size: usize) -> Buffer {
        Buffer {
            data:  [ ] ,
            offset:   VecBuffer::new(offset_size) ,
        }
    }
} */


/*impl From<UnixStream> for Socket {
    fn from(stream: UnixStream) -> Self {
        Socket {

        }
    }
} */






#[cfg(test)]
mod test {
    use std::ffi::CStr;
    use std::os::unix::net::UnixStream;
    use nix::libc::read;
    use nix::unistd::pipe;
    use std::string::String;
    use std::sync::{mpsc, Mutex};
    use std::sync::mpsc::Sender;
    use std::thread;
    use std::time::Duration;
    use nix::sys::socket::socketpair;



    #[test]
    fn test() {
        let mut  a = vec![3 , 3 , 5] ;



        /*
                let (client, server) = UnixStream::pair().unwrap();
               // pipe().unwrap();
                let (tx, rx): (Sender<String>, std::sync::mpsc::Receiver<String>) = mpsc::channel();
                // let z = Mutex::new("") ;
                let a = Socket;
                for i in  0..10  {
                    a.write("hellotou r ttt ret".as_bytes());
                    //thread::sleep(Duration::from_secs(1));
                }

               /* thread::spawn(|| {

                }); */
                thread::spawn(move || {
                    let mut server = &mut Socket::from(server);
                    for i in 0..10 {
                        tx.send(String::from_utf8(server.read()).unwrap()).unwrap()
                    }
                });
                for i in rx {
                    println!("{}", i)
                };*/
    }
}
