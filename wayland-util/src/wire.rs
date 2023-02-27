use std::borrow::Borrow;
use std::os::unix::io::RawFd;
use nix::fcntl;
use crate::socket::{Buffer, BufferedSocket};
use nix::{Error as NixError, Result as NixResult};
fn dup_fd_cloexec(fd: RawFd) -> NixResult<RawFd> {
    use nix::fcntl;
    match fcntl::fcntl(fd, fcntl::FcntlArg::F_DUPFD_CLOEXEC(0)) {
        Ok(fd) => Ok(fd),
        Err(NixError::EINVAL) => {
            // F_DUPFD_CLOEXEC is not recognized, kernel too old, fallback
            // to setting CLOEXEC manually
            let newfd = fcntl::fcntl(fd, fcntl::FcntlArg::F_DUPFD(0))?;

            let flags = fcntl::fcntl(newfd, fcntl::FcntlArg::F_GETFD);
            let result = flags
                .map(|f| fcntl::FdFlag::from_bits(f).unwrap() | fcntl::FdFlag::FD_CLOEXEC)
                .and_then(|f| fcntl::fcntl(newfd, fcntl::FcntlArg::F_SETFD(f)));
            match result {
                Ok(_) => {
                    // setting the O_CLOEXEC worked
                    Ok(newfd)
                }
                Err(e) => {
                    // something went wrong in F_GETFD or F_SETFD
                    let _ = ::nix::unistd::close(newfd);
                    Err(e)
                }
            }
        }
        Err(e) => Err(e),
    }
}


struct Message {
    id: u32,
    opcode: u32,
    argument: Vec<Argument>,
}

enum Argument {
    Int(i32),
    Uint(u32),
    String(Option<String>),
    Fixed(i32),
    Object(Option<u32>),
    NewId(u32),
    Array(Vec<u8>),
    Fd(RawFd),
}



impl Message {
    fn toBuffer (&self, mut buffer  :  Buffer) {
        let mut data = buffer.data ;
        let mut fds = buffer.offset ;
        for a in &self.argument {
            match a {
                Argument::Int(i) => { data.push(*i as u32 )  }
                Argument::Uint(i) => { data.push(*i )  }
                Argument::String(string) => {
                    match string {
                        None => {}
                        Some(i) => {}
                    }
                }
                Argument::Fixed(i) => { data.push(*i as u32 ) }
                Argument::Object(object) => {
                    match object {
                        None => {}
                        Some(i) => {}
                    }
                }
                Argument::NewId(i) => { }
                Argument::Array(i) => {  }
                Argument::Fd(i) => {
                   // let a = dup_fd_cloexec( i ).map_err()?;

                }
            }
        };
        let argument_len = data.len()*4 ;
        data.insert(0, self.id ) ;
        data.insert(1, ((argument_len as u32) << 16) | self.opcode    ) ;
        //let a = *data ;

    }
}

