use crate::*;

impl CmdService for Hget {
    fn execute(self, store: &impl Storage) -> CmdRes {
        match store.get(&self.table, &self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => KvError::NotFound(self.table, self.key).into(),
            Err(e) => e.into(),
        }
    }
}

impl CmdService for Hset {
    fn execute(self, store: &impl Storage) -> CmdRes {
        match self.pair {
            Some(v) => match store.set(&self.table, v.key, v.value.unwrap_or_default()) {
                Ok(Some(v)) => v.into(),
                Ok(None) => Value::default().into(),
                Err(e) => e.into(),
            },
            None => Value::default().into(),
        }
    }
}

impl CmdService for Hgetall {
    fn execute(self, store: &impl Storage) -> CmdRes {
        match store.get_all(&self.table) {
            Ok(v) => v.into(),
            Err(e) => e.into(),
        }
    }
}

impl CmdService for Hmset {
    fn execute(self, store: &impl Storage) -> CmdRes {
        todo!()
    }
}

impl CmdService for Hmget {
    fn execute(self, store: &impl Storage) -> CmdRes {
        todo!()
    }
}

impl CmdService for Hdel {
    fn execute(self, store: &impl Storage) -> CmdRes {
        todo!()
    }
}

impl CmdService for Hmdel {
    fn execute(self, store: &impl Storage) -> CmdRes {
        todo!()
    }
}

impl CmdService for Hexist {
    fn execute(self, store: &impl Storage) -> CmdRes {
        todo!()
    }
}

impl CmdService for Hmexist {
    fn execute(self, store: &impl Storage) -> CmdRes {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hset_should_work() {
        let store = MemTable::new();
        let cmd = CmdReq::new_hset("t", "k", 1);
        let res = dispatch(cmd.clone(), &store);
        assert_res_ok(res, &[Value::default()], &[]);

        let res = dispatch(cmd, &store);
        assert_res_ok(res, &[1.into()], &[]);
    }

    #[test]
    fn hget_should_work() {
        let store = MemTable::new();
        let cmd = CmdReq::new_hset("t", "k", 1);
        dispatch(cmd, &store);
        let cmd = CmdReq::new_hget("t", "k");
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &[1.into()], &[]);
    }

    #[test]
    fn hget_with_non_exist_key_should_return_404() {
        let store = MemTable::new();
        let cmd = CmdReq::new_hget("t", "u");
        let res = dispatch(cmd, &store);
        assert_res_err(res, 404, "Not found");
    }

    #[test]
    fn hget_all_should_work() {
        let store = MemTable::new();
        let cmds = vec![
            CmdReq::new_hset("t", "u1", 1),
            CmdReq::new_hset("t", "u2", 2),
            CmdReq::new_hset("t", "u3", 3),
            CmdReq::new_hset("t", "u1", 4),
        ];
        for cmd in cmds {
            dispatch(cmd, &store);
        }
        let cmd = CmdReq::new_hgetall("t");
        let res = dispatch(cmd, &store);
        let pairs = &[
            Kvpair::new("u1", 4),
            Kvpair::new("u2", 2),
            Kvpair::new("u3", 3),
        ];
        assert_res_ok(res, &[], pairs);
    }
}
