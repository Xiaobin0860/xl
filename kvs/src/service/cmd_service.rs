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
        for pair in self.pairs {
            let _ = store.set(&self.table, pair.key, pair.value.unwrap_or_default());
        }
        CmdRes::ok()
    }
}

impl CmdService for Hmget {
    fn execute(self, store: &impl Storage) -> CmdRes {
        let values: Vec<_> = self
            .keys
            .iter()
            .map(|k| match store.get(&self.table, k) {
                Ok(Some(v)) => v,
                _ => Value::none(),
            })
            .collect();
        values.into()
    }
}

impl CmdService for Hdel {
    fn execute(self, store: &impl Storage) -> CmdRes {
        match store.del(&self.table, &self.key) {
            Ok(Some(v)) => v.into(),
            _ => Value::none().into(),
        }
    }
}

impl CmdService for Hmdel {
    fn execute(self, store: &impl Storage) -> CmdRes {
        let values: Vec<_> = self
            .keys
            .iter()
            .map(|k| match store.del(&self.table, k) {
                Ok(Some(v)) => v,
                _ => Value::none(),
            })
            .collect();
        values.into()
    }
}

impl CmdService for Hexist {
    fn execute(self, store: &impl Storage) -> CmdRes {
        match store.contains(&self.table, &self.key) {
            Ok(b) => CmdRes::bool(b),
            _ => CmdRes::bool(false),
        }
    }
}

impl CmdService for Hmexist {
    fn execute(self, store: &impl Storage) -> CmdRes {
        CmdRes::bool(
            self.keys
                .iter()
                .all(|k| match store.contains(&self.table, k) {
                    Ok(true) => true,
                    _ => false,
                }),
        )
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
    fn hmset_should_work() {
        let store = MemTable::new();
        let pairs = vec![
            Kvpair::new("k1", "v1"),
            Kvpair::new("k2", 2),
            Kvpair::new("k3", 3),
        ];
        let cmd = CmdReq::new_hmset("t1", pairs);
        let res = dispatch(cmd.clone(), &store);
        assert_ok(res);

        let res = dispatch(cmd, &store);
        assert_ok(res);
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
    fn hexist_should_work() {
        let store = MemTable::new();
        dispatch(CmdReq::new_hset("t", "k", 1), &store);
        let res = dispatch(CmdReq::new_hexist("t", "k"), &store);
        assert_res_ok(res, &[true.into()], &[]);

        let res = dispatch(CmdReq::new_hexist("t", "n"), &store);
        assert_res_ok(res, &[false.into()], &[]);
        let res = dispatch(CmdReq::new_hexist("t1", "k"), &store);
        assert_res_ok(res, &[false.into()], &[]);
    }

    #[test]
    fn hdel_should_work() {
        let store = MemTable::new();
        let cmd = CmdReq::new_hset("t", "k", 1);
        dispatch(cmd, &store);
        let cmd = CmdReq::new_hdel("t", "k");
        let res = dispatch(cmd.clone(), &store);
        assert_res_ok(res, &[1.into()], &[]);

        let res = dispatch(cmd, &store);
        assert_res_ok(res, &[Value::default()], &[]);
    }

    #[test]
    fn hmget_should_work() {
        let store = MemTable::new();
        let pairs = vec![
            Kvpair::new("k1", "v1"),
            Kvpair::new("k2", 2),
            Kvpair::new("k3", 3),
            Kvpair::new("k1", 1),
        ];
        let cmd = CmdReq::new_hmset("t1", pairs);
        dispatch(cmd, &store);

        let keys = vec![
            "k1".to_string(),
            "k2".to_string(),
            "k3".to_string(),
            "k4".to_string(),
        ];
        let cmd = CmdReq::new_hmget("t1", keys);
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &[1.into(), 2.into(), 3.into(), Value::none()], &[]);
    }

    #[test]
    fn hmexist_should_work() {
        let store = MemTable::new();
        let pairs = vec![
            Kvpair::new("k1", "v1"),
            Kvpair::new("k2", 2),
            Kvpair::new("k3", 3),
        ];
        let cmd = CmdReq::new_hmset("t1", pairs);
        dispatch(cmd, &store);

        let keys = vec!["k1".to_string(), "k2".to_string()];
        let cmd = CmdReq::new_hmexist("t1", keys.clone());
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &[true.into()], &[]);

        let cmd = CmdReq::new_hmexist("t", keys);
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &[false.into()], &[]);

        let cmd = CmdReq::new_hmexist(
            "t1",
            vec!["k1".to_string(), "k2".to_string(), "n".to_string()],
        );
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &[false.into()], &[]);
    }

    #[test]
    fn hmdel_should_work() {
        let store = MemTable::new();
        let pairs = vec![
            Kvpair::new("k1", "v1"),
            Kvpair::new("k2", 2),
            Kvpair::new("k3", 3),
            Kvpair::new("k1", 1),
        ];
        let cmd = CmdReq::new_hmset("t1", pairs);
        dispatch(cmd, &store);

        let keys = vec![
            "k1".to_string(),
            "k2".to_string(),
            "k3".to_string(),
            "k4".to_string(),
        ];
        let cmd = CmdReq::new_hmdel("t1", keys);
        let res = dispatch(cmd.clone(), &store);
        assert_res_ok(res, &[1.into(), 2.into(), 3.into(), Value::none()], &[]);

        let res = dispatch(cmd, &store);
        assert_res_ok(
            res,
            &[Value::none(), Value::none(), Value::none(), Value::none()],
            &[],
        );
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
