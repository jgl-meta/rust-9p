use crate::error;

pub type Result<T> = ::std::result::Result<T, error::Error>;

macro_rules! io_err {
    ($kind:ident, $msg:expr) => {
        ::std::io::Error::new(::std::io::ErrorKind::$kind, $msg)
    };
}

macro_rules! res {
    ($err:expr) => {
        Err(From::from($err))
    };
}

pub fn parse_proto(arg: &str) -> Option<(&str, String)> {
    match arg == "io" {
        true => Some((arg, "".to_string())),
        _ => {
            let mut split = arg.split('!');
            let (proto, addr, port) = (split.next()?, split.next()?, split.next()?);
            let mut delim = "_";

            match proto == "tcp" {
                true => delim = ":",
                _ => (),
            }

            Some((proto, addr.to_owned() + delim + port))
        }
    }
}
