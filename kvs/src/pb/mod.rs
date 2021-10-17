pub mod abi;
pub use abi::{cmd_req::ReqData, *};

use crate::KvError;
use http::StatusCode;

impl CmdReq {
    /// 创建 HSET 命令
    pub fn new_hset(
        table: impl Into<String>,
        key: impl Into<String>,
        value: impl Into<Value>,
    ) -> Self {
        Self {
            req_data: Some(ReqData::Hset(Hset {
                table: table.into(),
                pair: Some(Kvpair::new(key, value.into())),
            })),
        }
    }

    pub fn new_hget(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            req_data: Some(ReqData::Hget(Hget {
                table: table.into(),
                key: key.into(),
            })),
        }
    }

    pub fn new_hgetall(table: impl Into<String>) -> Self {
        Self {
            req_data: Some(ReqData::Hgetall(Hgetall {
                table: table.into(),
            })),
        }
    }
}

impl Kvpair {
    /// 创建一个新的 kv pair
    pub fn new(key: impl Into<String>, value: impl Into<Value>) -> Self {
        Self {
            key: key.into(),
            value: Some(value.into()),
        }
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self {
            value: Some(value::Value::String(s.to_string())),
        }
    }
}

impl From<i64> for Value {
    fn from(n: i64) -> Self {
        Self {
            value: Some(value::Value::Integer(n)),
        }
    }
}

impl From<Value> for CmdRes {
    fn from(v: Value) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: vec![v],
            ..Default::default()
        }
    }
}

impl From<KvError> for CmdRes {
    fn from(e: KvError) -> Self {
        let mut result = Self {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as _,
            message: e.to_string(),
            values: vec![],
            pairs: vec![],
        };
        match e {
            KvError::NotFound(_, _) => result.status = StatusCode::NOT_FOUND.as_u16() as _,
            KvError::InvalidCmd(_) => result.status = StatusCode::BAD_REQUEST.as_u16() as _,
            _ => {}
        }
        result
    }
}

impl From<Vec<Kvpair>> for CmdRes {
    fn from(pairs: Vec<Kvpair>) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            pairs,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = Kvpair::new("hello", "world");
        assert_eq!(
            v,
            Kvpair {
                key: "hello".to_string(),
                value: Some(Value {
                    value: Some(value::Value::String("world".to_string()))
                })
            }
        );

        assert_eq!(
            CmdReq::new_hset("table", "hello", 1),
            CmdReq {
                req_data: Some(ReqData::Hset(Hset {
                    table: "table".to_string(),
                    pair: Some(Kvpair::new("hello", 1))
                }))
            }
        );
    }
}
