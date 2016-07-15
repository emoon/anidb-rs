extern crate crypto;

mod errors;
mod cutil;
pub mod ed2k;
pub mod md4;

use std::net::{SocketAddr, ToSocketAddrs};
pub use errors::{AnidbError, Result};
//use std::io;
use std::str;
use std::thread;
use std::time::Duration;

use std::net::UdpSocket;

pub struct Anidb {
    pub socket: UdpSocket,
    pub address: SocketAddr,
    pub session: String,
}

pub struct ServerReply {
    pub code: usize,
    pub data: String,
}

impl Anidb {
    ///
    /// Creates a new instance of Anidb and makes a connection to the AniDB API server
    /// ```ignore
    /// // code unwraps for simplicy but the error codes should be handled by the errors
    /// let mut db = anidb::Anidb::new(("anidb_server.net", 6666)).unwrap();
    /// ```
    ///
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<Anidb> {
        let socket = try!(UdpSocket::bind(("0.0.0.0", 9000)));
        try!(socket.connect(&addr));

        Ok(Anidb {
            socket: socket,
            address: addr.to_socket_addrs().unwrap().next().unwrap(),
            session: "".to_owned(),
        })
    }

    ///
    /// Login the user to AniDB. You need to supply a user/pass that you have regisered at https://anidb.net/
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // code unwraps for simplicy but the error codes should be handled by the errors
    /// let mut db = anidb::Anidb::new(("anidb_server.net", 6666)).unwrap();
    /// db.login("leeloo_dallas", "multipass").unwrap();
    /// ```
    ///
    pub fn login(&mut self, username: &str, password: &str) -> Result<()> {
        let login_str = Self::format_login_string(username, password);

        let reply = try!(self.send_wait_reply(&login_str));

        println!("Reply from server {}", reply.data);

        self.session = try!(Self::validate_auth_command(&reply));

        Ok(())
    }

    ///
    /// Login the user to AniDB. You need to supply a user/pass that you have regisered at https://anidb.net/
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // code unwraps for simplicy but the error codes should be handled by the errors
    /// let mut db = anidb::Anidb::new(("anidb_server.net", 6666)).unwrap();
    /// db.login("leeloo_dallas", "multipass").unwrap();
    /// db.logout()unwrap();
    /// ```
    ///
    pub fn logout(&mut self) -> Result<()> {

        if self.session == "" {
            return Err(AnidbError::StaticError("Not logged in"));
        }

        let logout_str = Self::format_logout_string(&self.session);

        let reply = try!(self.send_wait_reply(&logout_str));

        println!("Reply from server {}", reply.data);

        Ok(())
    }

    /// Validates that the auth command has a correct reply from the server
    fn validate_auth_command(reply: &ServerReply) -> Result<String> {
        if reply.code != 200 {
            return Err(AnidbError::ErrorCode(reply.code, reply.data.to_owned()));
        }

        let v: Vec<&str> = reply.data.split(' ').collect();

        if v.len() != 3 {
            return Err(AnidbError::Error(format!("Invalid AUTH reply: {} expceted 3 args", reply.data)));
        }

        if v[1] != "LOGIN" || v[2] != "ACCEPTED\n" {
            return Err(AnidbError::Error(format!("Invalid AUTH reply: {} LOGIN ACCEPTED\\n expected", reply.data)));
        }

        Ok(v[0].to_owned())
    }

    pub fn wait_exec_command(&self, time: u64) {
        thread::sleep(Duration::from_millis(time))
    }

    /// Parse the reply from the server which is expected to be in xxx - format. If that is not the
    /// case this function will return an error that the reply couldn't be parsed.
    fn parse_reply(reply: &[u8], len: usize) -> Result<ServerReply> {
        if len < 5 {
            return Err(AnidbError::StaticError("Reply less than 5 chars"));
        }
        let code_str = try!(str::from_utf8(&reply[0..3]));
        let code = try!(code_str.parse::<usize>());
        Ok(ServerReply {
            code: code,
            data: String::from_utf8_lossy(&reply[4..len]).into_owned(),
        })
    }

    fn send_wait_reply(&self, message: &str) -> Result<ServerReply> {
        let mut result = [0; 2048];
        try!(self.socket.send(message.as_bytes()));
        let len = try!(self.socket.recv(&mut result));
        Self::parse_reply(&result, len)
    }

    fn format_logout_string(session_id: &str) -> String {
        format!("LOGOUT s={}", session_id)
    }

    fn format_login_string(username: &str, password: &str) -> String {
        format!("AUTH user={}&pass={}&protover=3&client=anidbrs&clientver=1", username, password)
    }
}


#[cfg(test)]
mod test_parse {
    use super::*;

    #[test]
    fn test_parse_reply_ok() {
        let reply = b"500 LOGIN FAILED";
        let ret = Anidb::parse_reply(reply, reply.len()).unwrap();
        assert_eq!(ret.code, 500);
        assert_eq!(ret.data, "LOGIN FAILED");
    }

    #[test]
    fn test_parse_reply_fail_1() {
        let reply = b"a3i5LOGIN FAILED";
        assert_eq!(true, Anidb::parse_reply(reply, reply.len()).is_err());
    }

    #[test]
    fn test_parse_reply_fail_2() {
        let reply = b"34i5LOGIN FAILED";
        assert_eq!(true, Anidb::parse_reply(reply, reply.len()).is_err());
    }

    #[test]
    fn test_parse_reply_too_short() {
        let reply = b"3D";
        assert_eq!(true, Anidb::parse_reply(reply, reply.len()).is_err());
    }

    #[test]
    fn test_parse_reply_exact_length() {
        let reply = b"777 O";
        let ret = Anidb::parse_reply(reply, reply.len()).unwrap();
        assert_eq!(ret.code, 777);
        assert_eq!(ret.data, "O");
    }
}

#[cfg(test)]
mod test_format {
    use super::*;

    #[test]
    fn test_format_login_string() {
        let login_string = Anidb::format_login_string("leeloo_dallas", "multipass");
        assert_eq!(login_string, "AUTH user=leeloo_dallas&pass=multipass&protover=3&client=anidbrs&clientver=1");
    }

    #[test]
    fn test_format_logout_string() {
        let logout_str = Anidb::format_logout_string("abcd1234");
        assert_eq!(logout_str, "LOGOUT s=abcd1234");
    }
}
