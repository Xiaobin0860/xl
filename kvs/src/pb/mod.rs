pub(crate) mod abi;

pub(crate) use abi::{cmd_req::ReqData, *};

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
}

impl Kvpair {
    /// 创建一个新的 kv pair
    fn new(key: impl Into<String>, value: impl Into<Value>) -> Self {
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
