use std::sync::Arc;
use tracing::debug;

use crate::*;
mod cmd_service;

/// 对 Command 的处理的抽象
pub trait CmdService {
    /// 处理 Command，返回 Response
    fn execute(self, store: &impl Storage) -> CmdRes;
}

pub struct Service<Store = MemTable> {
    inner: Arc<ServiceInner<Store>>,
}

impl<Store: Storage> Service<Store> {
    pub fn new(store: Store) -> Self {
        Self {
            inner: Arc::new(ServiceInner { store }),
        }
    }

    pub fn execute(&self, cmd: CmdReq) -> CmdRes {
        debug!("Got req: {:?}", cmd);
        let res = dispatch(cmd, &self.inner.store);
        debug!("Executed res: {:?}", res);
        res
    }
}

pub struct ServiceInner<Store> {
    store: Store,
}

impl<Store> Clone for Service<Store> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

fn dispatch(req: CmdReq, store: &impl Storage) -> CmdRes {
    match req.req_data.unwrap() {
        ReqData::Hget(v) => v.execute(store),
        ReqData::Hgetall(v) => v.execute(store),
        ReqData::Hmget(v) => v.execute(store),
        ReqData::Hset(v) => v.execute(store),
        ReqData::Hmset(v) => v.execute(store),
        ReqData::Hdel(v) => v.execute(store),
        ReqData::Hmdel(v) => v.execute(store),
        ReqData::Hexist(v) => v.execute(store),
        ReqData::Hmexist(v) => v.execute(store),
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn service_should_works() {
        let service = Service::new(MemTable::new());

        let cloned = service.clone();

        let handle = thread::spawn(move || {
            let res = cloned.execute(CmdReq::new_hset("t1", "k1", "v1"));
            assert_res_ok(res, &[Value::default()], &[]);
        });
        handle.join().unwrap();

        let res = service.execute(CmdReq::new_hget("t1", "k1"));
        assert_res_ok(res, &["v1".into()], &[]);
    }
}

#[cfg(test)]
use crate::{Kvpair, Value};

#[cfg(test)]
pub fn assert_res_ok(mut res: CmdRes, values: &[Value], pairs: &[Kvpair]) {
    res.pairs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(res.status, 200);
    assert_eq!(res.message, "");
    assert_eq!(res.values, values);
    assert_eq!(res.pairs, pairs);
}

#[cfg(test)]
pub fn assert_res_err(res: CmdRes, code: u32, msg: &str) {
    assert_eq!(res.status, code);
    assert!(res.message.contains(msg));
    assert_eq!(res.values, &[]);
    assert_eq!(res.pairs, &[]);
}
