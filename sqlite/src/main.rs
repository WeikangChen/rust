
// time crate and its constants
extern crate time;
use time::Timespec;
const SQLITE_DATETIME_FMT: &str = "%Y-%m-%d %H:%M:%S:%f %Z";

#[derive(Clone, Debug)]
pub enum Value {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}

macro_rules! from_i64(
    ($t:ty) => (
        impl From<$t> for Value {
            fn from(i: $t) -> Value {
                Value::Integer(i64::from(i))
            }
        }
    )
);

from_i64!(i8);
from_i64!(i16);
from_i64!(i32);
from_i64!(u8);
from_i64!(u16);
from_i64!(u32);

impl From<String> for Value {
    fn from(s: String) -> Value {
        Value::Text(s)
    }
}

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

macro_rules! to_sql_self(
    ($t:ty) => (
        impl ToSql for $t {
            fn to_sql(&self) -> Result<ToSqlOutput> {
                Ok(ToSqlOutput::from(*self))
            }
        }
    )
);

to_sql_self!(i8);
to_sql_self!(i16);
to_sql_self!(i32);
to_sql_self!(u8);
to_sql_self!(u16);
to_sql_self!(u32);


impl ToSql for String {
    fn to_sql(&self) -> Result<ToSqlOutput> {
        Ok(ToSqlOutput::from(self.as_str()))
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

impl<'a> ToSql for ToSqlOutput<'a>
{
    fn to_sql(&self) -> Result<ToSqlOutput> {
        Ok(match *self {
            ToSqlOutput::Borrowed(v) => ToSqlOutput::Borrowed(v),
	    ToSqlOutput::Owned(ref v) => ToSqlOutput::Borrowed(ValueRef::from(v)),
        })
    }
}



struct Connection;

impl Connection {
    pub fn execute(&self, sql: &str, params: &[&ToSql]) {
        println!("Execute cmd {}", sql);
        for p in params {
            let output  = p.to_sql();
            match output {
                Err(_) => println!("[Error]"),
                Ok(val) => println!("[Param] {:?}", val),
            }
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
    let me = Person {
        id: 1,
        age: 32,
        name: "Steven".to_string(),
        time_created: time::get_time(),
        data: None
    };

    let conn = Connection{};
    conn.execute("INSERT INTO person (name, time_created, data) VALUES (?1, ?2, ?3)",
                 &[&me.id, &me.age, &me.name, &me.time_created]);
}
