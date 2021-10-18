use crate::{KvError, Kvpair, Value};

mod memory;
pub use memory::MemTable;

/// 对存储的抽象，我们不关心数据存在哪儿，但需要定义外界如何和存储打交道
pub trait Storage {
    /// 从一个 HashTable 里获取一个 key 的 value
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// 从一个 HashTable 里设置一个 key 的 value，返回旧的 value
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;
    /// 查看 HashTable 中是否有 key
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
    /// 从 HashTable 中删除一个 key
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// 遍历 HashTable，返回所有 kv pair（这个接口不好）
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;
    /// 遍历 HashTable，返回 kv pair 的 Iterator
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memtable_basic_interface_should_work() {
        let store = MemTable::new();
        test_basi_interface(store);
    }

    #[test]
    fn memtable_get_all_should_work() {
        let store = MemTable::new();
        test_get_all(store);
    }

    #[test]
    fn memtable_get_iter_should_work() {
        let store = MemTable::new();
        test_get_iter(store);
    }

    fn test_basi_interface(store: impl Storage) {
        // 第一次 set 会创建 table，插入 key 并返回 None（之前没值）
        let v = store.set("t1", "k".to_string(), 1.into());
        assert!(v.unwrap().is_none());
        // 再次 set 同样的 key 会更新，并返回之前的值
        let v = store.set("t1", "k".to_string(), 2.into());
        assert_eq!(v.unwrap(), Some(1.into()));
        // get 存在的 key 会得到最新的值
        let v = store.get("t1", "k");
        assert_eq!(v.unwrap(), Some(2.into()));
        // get 不存在的 key 或者 table 会得到 None
        assert!(store.get("t1", "k1").unwrap().is_none());
        assert!(store.get("t", "k").unwrap().is_none());
        // contains 存在的 key 返回 true，否则 false
        assert_eq!(store.contains("t1", "k").unwrap(), true);
        assert_eq!(store.contains("t1", "k1").unwrap(), false);
        assert_eq!(store.contains("t", "k").unwrap(), false);
        // del 存在的 key 返回之前的值
        let v = store.del("t1", "k");
        assert_eq!(v.unwrap(), Some(2.into()));
        // del 不存在的 key 或 table 返回 None
        let v = store.del("t1", "k");
        assert!(v.unwrap().is_none());
    }

    fn test_get_all(store: impl Storage) {
        assert!(store.set("t", "k1".to_string(), 1.into()).is_ok());
        assert!(store.set("t", "k2".to_string(), "two".into()).is_ok());
        let mut v = store.get_all("t").unwrap();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(v, vec![Kvpair::new("k1", 1), Kvpair::new("k2", "two")]);
    }

    fn test_get_iter(_store: impl Storage) {}
}
