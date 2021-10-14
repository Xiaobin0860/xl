pub mod abi;

use abi::{cmd_req::ReqData, *};
use http::StatusCode;

use crate::KvError;

impl CmdReq {
    /// 创建 HGET 命令
    pub fn new_hget(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            req_data: Some(ReqData::Hget(Hget {
                table: table.into(),
                key: key.into(),
            })),
        }
    }

    /// 创建 HGETALL 命令
    pub fn new_hgetall(table: impl Into<String>) -> Self {
        Self {
            req_data: Some(ReqData::Hgetall(Hgetall {
                table: table.into(),
            })),
        }
    }

    /// 创建 HSET 命令
    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        Self {
            req_data: Some(ReqData::Hset(Hset {
                table: table.into(),
                pair: Some(Kvpair::new(key, value)),
            })),
        }
    }
}

impl Kvpair {
    /// 创建一个新的 kv pair
    pub fn new(key: impl Into<String>, value: Value) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}

/// 从 String 转换成 Value
impl From<String> for Value {
    fn from(s: String) -> Self {
        Self {
            value: Some(value::Value::String(s)),
        }
    }
}

/// 从 &str 转换成 Value
impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self {
            value: Some(value::Value::String(s.into())),
        }
    }
}

/// 从 i64转换成 Value
impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Self {
            value: Some(value::Value::Integer(i)),
        }
    }
}

/// 从 Value 转换成 CmdRes
impl From<Value> for CmdRes {
    fn from(v: Value) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: vec![v],
            ..Default::default()
        }
    }
}

/// 从 Vec<Kvpair> 转换成 CmdRes
impl From<Vec<Kvpair>> for CmdRes {
    fn from(v: Vec<Kvpair>) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            pairs: v,
            ..Default::default()
        }
    }
}

/// 从 KvError 转换成 CmdRes
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
            KvError::InvalidCommand(_) => result.status = StatusCode::BAD_REQUEST.as_u16() as _,
            _ => {}
        }

        result
    }
}
