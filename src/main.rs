
extern crate time;
// extern crate libsqlite3_sys as ffi;

#[macro_use]
extern crate bitflags;

use time::Timespec;
use std::mem;
use std::ptr;
use std::path::{Path, PathBuf};
use std::cell::RefCell;
use std::os::raw::c_int;

// use std::ffi::CString;
// pub use ffi::ErrorCode;

// Number of cached prepared statements we'll hold on to.
const STATEMENT_CACHE_DEFAULT_CAPACITY: usize = 16;
const SQLITE_DATETIME_FMT: &str = "%Y-%m-%d %H:%M:%S:%f %Z";

#[derive(Debug)]
struct DBL(f64);
#[derive(Debug)]
struct INT(i32);

#[allow(dead_code)]
impl From<INT> for DBL {
    fn from(x: INT) -> Self {
        DBL (x.0 as f64)
    }
}

#[derive(Debug)]
enum Coord {
    D2(i32, i32),
    D3(i32, i32, i32),
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Value {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}

impl From<i32> for Value {
    fn from(i: i32) -> Value {
        Value::Integer(i64::from(i))
    }
}

impl From<String> for Value {
    fn from(s: String) -> Value {
        Value::Text(s)
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum ValueRef<'a> {
    Null,
    Integer(i64),
    Real(f64),
    Text(&'a str),
    Blob(&'a [u8]),
}

impl<'a> From<&'a str> for ValueRef<'a> {
    fn from(s: &str) -> ValueRef {
        ValueRef::Text(s)
    }
}

impl<'a> From<&'a Value> for ValueRef<'a> {
    fn from(value: &'a Value) -> ValueRef<'a> {
        match *value {
            Value::Null => ValueRef::Null,
            Value::Integer(i) => ValueRef::Integer(i),
            Value::Real(r) => ValueRef::Real(r),
            Value::Text(ref s) => ValueRef::Text(s),
            Value::Blob(ref b) => ValueRef::Blob(b),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ToSqlOutput<'a> {
    Borrowed(ValueRef<'a>),
    Owned(Value),
}

impl<'a, T> From<T> for ToSqlOutput<'a>
    where T: Into<Value>
{
    fn from(t: T) -> Self {
        ToSqlOutput::Owned(t.into())
    }
}

impl<'a, T: ?Sized> From<&'a T> for ToSqlOutput<'a>
    where &'a T: Into<ValueRef<'a>>
{
    fn from(t: &'a T) -> Self {
        ToSqlOutput::Borrowed(t.into())
    }
}

use std::result;
pub enum Error {
    InvalidFileName(String),
    NoPermision(String),
}
/// A typedef of the result returned by many methods.
pub type Result<T> = result::Result<T, Error>;
pub trait ToSql {
    fn to_sql(&self) -> Result<ToSqlOutput>;
}

impl<'a> ToSql for ToSqlOutput<'a>
{
    fn to_sql(&self) -> Result<ToSqlOutput> {
        Ok(match *self {
            ToSqlOutput::Borrowed(v) => ToSqlOutput::Borrowed(v),
	    ToSqlOutput::Owned(ref v) => ToSqlOutput::Borrowed(ValueRef::from(v)),
        })
    }
}

impl ToSql for time::Timespec {
    fn to_sql(&self) -> Result<ToSqlOutput> {
        let time_string = time::at_utc(*self)
            .strftime(SQLITE_DATETIME_FMT)
            .unwrap()
            .to_string();
        Ok(ToSqlOutput::from(time_string))
    }
}

impl ToSql for i32 {
    fn to_sql(&self) -> Result<ToSqlOutput> {
        Ok(ToSqlOutput::from(*self))
    }
}

impl ToSql for String {
    fn to_sql(&self) -> Result<ToSqlOutput> {
        Ok(ToSqlOutput::from(self.as_str()))
    }
}

/*
struct InnerConnection {
    db: *mut ffi::sqlite3,
}

bitflags! {
    #[doc = "Flags for opening SQLite database connections."]
    #[doc = "See [sqlite3_open_v2](http://www.sqlite.org/c3ref/open.html) for details."]
    #[repr(C)]
    pub struct OpenFlags: ::std::os::raw::c_int {
        const SQLITE_OPEN_READ_ONLY     = ffi::SQLITE_OPEN_READONLY;
        const SQLITE_OPEN_READ_WRITE    = ffi::SQLITE_OPEN_READWRITE;
        const SQLITE_OPEN_CREATE        = ffi::SQLITE_OPEN_CREATE;
        const SQLITE_OPEN_URI           = 0x0000_0040;
        const SQLITE_OPEN_MEMORY        = 0x0000_0080;
        const SQLITE_OPEN_NO_MUTEX      = ffi::SQLITE_OPEN_NOMUTEX;
        const SQLITE_OPEN_FULL_MUTEX    = ffi::SQLITE_OPEN_FULLMUTEX;
        const SQLITE_OPEN_SHARED_CACHE  = 0x0002_0000;
        const SQLITE_OPEN_PRIVATE_CACHE = 0x0004_0000;
    }
}

impl Default for OpenFlags {
    fn default() -> OpenFlags {
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE |
        OpenFlags::SQLITE_OPEN_NO_MUTEX | OpenFlags::SQLITE_OPEN_URI
    }
}

/// A connection to a SQLite database.
pub struct Connection {
    db: RefCell<InnerConnection>,
    path: Option<PathBuf>,
}

unsafe impl Send for Connection {}

impl Drop for Connection {
    fn drop(&mut self) {}
}

impl InnerConnection {
    fn open_with_flags(c_path: &CString, flags: OpenFlags) -> Result<InnerConnection> {
        unsafe {
            let mut db: *mut ffi::sqlite3 = mem::uninitialized();
            let r = ffi::sqlite3_open_v2(c_path.as_ptr(), &mut db, flags.bits(), ptr::null());
            let r = ffi::sqlite3_busy_timeout(db, 5000);
            ffi::sqlite3_extended_result_codes(db, 1);
            Ok(InnerConnection { db: db })
        }
    }
    fn db(&self) -> *mut ffi::sqlite3 {
        self.db
    }
}

*/

struct Connection;

impl Connection {
    /*
    pub fn open_in_memory() -> Result<Connection> {
        let flags = Default::default();
        Connection::open_in_memory_with_flags(flags)
    }
    pub fn open_in_memory_with_flags(flags: OpenFlags) -> Result<Connection> {

        let c_memory = CString::new(":memory:").unwrap();
        Connection::open_with_flags(&c_memory, flags).map(|db| {
            Connection {
                db: RefCell::new(db),
                path: None,
            }
        })
    }
    pub fn open_with_flags<P: AsRef<Path>>(path: P, flags: OpenFlags) -> Result<Connection> {
        let c_path = CString::new("path.as_ref()").unwrap();
        InnerConnection::open_with_flags(&c_path, flags).map(|db| {
            Connection {
                db: RefCell::new(db),
                path: Some(path.as_ref().to_path_buf()),
            }
        })
    }
    pub fn execute(&self, sql: &str, params: &[&ToSql]) -> Result<c_int> {
        self.prepare(sql)
            .and_then(|mut stmt| stmt.execute(params))
    }
    */
     pub fn execute(&self, sql: &str, params: &[&ToSql]) {
        for p in params {
            let output  = p.to_sql();
            match output {
                Err(_) => println!("Error"),
                Ok(val) => println!("{:?}", val),
            }
            // println!("{:?}", output.unwrap());
            // println!("{:?}", ToSqlOutput::from(Value::from(3 as i32)));
        }
    }
}


#[derive(Debug)]
struct Person {
    id: i32,
    age: i32,
    name: String,
    time_created: Timespec,
    data: Option<Vec<u8>>
}

fn main() {
    /*
    let ti = INT(99);
    let tf = DBL::from(ti);
    println!("tf = {:?}", tf);

    let origin2d = Coord::D2(3, 4);
    let origin3d = Coord::D3(3, 4, 5);
    let pt_ref = &origin2d;
    match *pt_ref {
        Coord::D2(x, y) => println!("2 dim origin {}, {}", x, y),
        Coord::D3(x, y, z) => println!("3 dim origin {}, {} {}", x, y, z),
    }

    println!("origin2d = {:?}", origin2d);
    println!("origin2d = {:?}", origin3d);

    let conn = Connection::open_in_memory().unwrap();

    conn.execute("CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  time_created    TEXT NOT NULL,
                  data            BLOB
    )", &[]).unwrap();
    */
    let conn = Connection{};
    let me = Person {
        id: 1,
        age: 32,
        name: "Steven".to_string(),
        time_created: time::get_time(),
        data: None
    };
    conn.execute("INSERT INTO person (name, time_created, data) VALUES (?1, ?2, ?3)",
                 &[&me.id, &me.age, &me.name, &me.time_created]);
//                 &[&me.name, &me.time_created, &me.data]
//    ).unwrap();

}
