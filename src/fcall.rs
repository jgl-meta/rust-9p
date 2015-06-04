
//! Define data types and constants used in 9P protocol

extern crate num;

/// The type of I/O
///
/// Open mode to be checked against the permissions for the file.
pub mod om {
    /// Open for read
    pub const READ: u8      = 0;
    /// Write
    pub const WRITE: u8     = 1;
    /// Read and write
    pub const RDWR: u8      = 2;
    /// Execute, == read but check execute permission
    pub const EXEC: u8      = 3;
    /// Or'ed in (except for exec), truncate file first
    pub const TRUNC: u8     = 16;
    /// Or'ed in, close on exec
    pub const CEXEC: u8     = 32;
    /// Or'ed in, remove on close
    pub const RCLOSE: u8    = 64;
}

/// Bits in Qid.typ
pub mod qt {
    /// Type bit for directories
    pub const DIR: u8       = 0x80;
    /// Type bit for append only files
    pub const APPEND: u8    = 0x40;
    /// Type bit for exclusive use files
    pub const EXCL: u8      = 0x20;
    /// Type bit for mounted channel
    pub const MOUNT: u8     = 0x10;
    /// Type bit for authentication file
    pub const AUTH: u8      = 0x08;
    /// Type bit for not-backed-up file
    pub const TMP: u8       = 0x04;
    /// Plain file
    pub const FILE: u8      = 0x00;
}

/// Bits in Stat.mode
pub mod dm {
    /// Mode bit for directories
    pub const DIR: u32      = 0x80000000;
    /// Mode bit for append only files
    pub const APPEND: u32   = 0x40000000;
    /// Mode bit for exclusive use files
    pub const EXCL: u32     = 0x20000000;
    /// Mode bit for mounted channel
    pub const MOUNT: u32    = 0x10000000;
    /// Mode bit for authentication file
    pub const AUTH: u32     = 0x08000000;
    /// Mode bit for non-backed-up files
    pub const TMP: u32      = 0x04000000;
    /// Mode bit for read permission
    pub const READ: u32     = 0x4;
    /// Mode bit for write permission
    pub const WRITE: u32    = 0x2;
    /// Mode bit for execute permission
    pub const EXEC: u32     = 0x1;
}

/// Server side data type for path tracking
///
/// The server's unique identification for the file being accessed
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Qid {
    /// Specify whether the file is a directory, append-only file, etc.
    pub typ: u8,
    /// Version number for a file; typically, it is incremented every time the file is modified
    pub version: u32,
    /// An integer which is unique among all files in the hierarchy
    pub path: u64
}

/// Namespace metadata (somewhat like a unix fstat)
///
/// NOTE: Defined as `Dir` in libc.h of Plan 9
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Stat {
    /// Server type
    pub typ: u16,
    /// Server subtype
    pub dev: u32,
    /// Unique id from server
    pub qid: Qid,
    /// Permissions
    pub mode: u32,
    /// Last read time
    pub atime: u32,
    /// Last write time
    pub mtime: u32,
    /// File length
    pub length: u64,
    /// Last element of path
    pub name: String,
    /// Owner name
    pub uid: String,
    /// Group name
    pub gid: String,
    /// Last modifier name
    pub muid: String
}

impl Stat {
    /// Get the current size of the stat
    pub fn size(&self) -> u16 {
        use std::mem::{size_of, size_of_val};
        (size_of_val(&self.typ) +
        size_of_val(&self.dev) +
        size_of_val(&self.qid) +
        size_of_val(&self.mode) +
        size_of_val(&self.atime) +
        size_of_val(&self.mtime) +
        size_of_val(&self.length) +
        (size_of::<u16>() * 4) +
        self.name.len() + self.uid.len() +
        self.gid.len() + self.muid.len()) as u16
    }
}

/// Data type used in Rread and Twrite
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Data(Vec<u8>);

impl Data {
    pub fn new(v: Vec<u8>) -> Data { Data(v) }
    pub fn data(&self) -> &[u8] { &self.0 }
}

enum_from_primitive! {
    /// Message type, 9P operations
    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum MsgType {
        Tversion =  100,
        Rversion,
        Tauth =     102,
        Rauth,
        Tattach =   104,
        Rattach,
        // Illegal, never used
        Terror =    106,
        Rerror,
        Tflush =    108,
        Rflush,
        Twalk =     110,
        Rwalk,
        Topen =     112,
        Ropen,
        Tcreate =   114,
        Rcreate,
        Tread =     116,
        Rread,
        Twrite =    118,
        Rwrite,
        Tclunk =    120,
        Rclunk,
        Tremove =   122,
        Rremove,
        Tstat =     124,
        Rstat,
        Twstat =    126,
        Rwstat,
    }
}

/// Envelope for 9P2000 messages
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Msg {
    /// Message type, one of the constants in MsgType
    pub typ: MsgType,
    /// Chosen and used by the client to identify the message.
    /// The reply to the message will have the same tag
    pub tag: u16,
    /// Message body encapsulating the various 9P messages
    pub body: Fcall
}

/// A data type encapsulating the various 9P messages
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Fcall {
    Tversion { msize: u32, version: String },
    Rversion { msize: u32, version: String },
    Tauth { afid: u32, uname: String, aname: String },
    Rauth { aqid: Qid },
    Rerror { ename: String },
    Tflush { oldtag: u16 },
    Rflush,
    Tattach { fid: u32, afid: u32, uname: String, aname: String },
    Rattach { qid: Qid },
    Twalk { fid: u32, newfid: u32, wnames: Vec<String> },
    Rwalk { wqids: Vec<Qid> },
    Topen { fid: u32, mode: u8 },
    Ropen { qid: Qid, iounit: u32 },
    Tcreate { fid: u32, name: String, perm: u32, mode: u8 },
    Rcreate { qid: Qid, iounit: u32 },
    Tread { fid: u32, offset: u64, count: u32 },
    Rread { data: Data },
    Twrite { fid: u32, offset: u64, data: Data },
    Rwrite { count: u32 },
    Tclunk { fid: u32 },
    Rclunk,
    Tremove { fid: u32 },
    Rremove,
    Tstat { fid: u32 },
    Rstat { stat: Stat },
    Twstat { fid: u32, stat: Stat },
    Rwstat,
}

impl Fcall {
    /// Get request's fid if available
    pub fn fid(&self) -> Option<u32> {
        match self {
            &Fcall::Tattach { fid, .. }     => Some(fid),
            &Fcall::Twalk { fid, .. }       => Some(fid),
            &Fcall::Topen { fid, .. }       => Some(fid),
            &Fcall::Tcreate { fid, .. }     => Some(fid),
            &Fcall::Tread { fid, .. }       => Some(fid),
            &Fcall::Twrite { fid, .. }      => Some(fid),
            &Fcall::Tclunk { fid }          => Some(fid),
            &Fcall::Tremove { fid }         => Some(fid),
            &Fcall::Tstat { fid }           => Some(fid),
            &Fcall::Twstat { fid, .. }      => Some(fid),
            _ => None
        }
    }

    /// Get request's newfid if available
    pub fn newfid(&self) -> Option<u32> {
        match self {
            &Fcall::Twalk { newfid, .. }    => Some(newfid),
            _ => None
        }
    }

    pub fn qid(&self) -> Option<Qid> {
        match self {
            &Fcall::Rauth { aqid }          => Some(aqid),
            &Fcall::Rattach { qid }         => Some(qid),
            &Fcall::Rwalk { ref wqids }     => wqids.last().map(|q| *q),
            &Fcall::Ropen { qid, .. }       => Some(qid),
            &Fcall::Rcreate { qid, .. }     => Some(qid),
            _ => None
        }
    }
}
