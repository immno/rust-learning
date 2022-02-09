pub use error::KvError;
pub use pb::abi::*;
pub use storage::*;
pub use service::*;

mod pb;
mod storage;
mod error;
mod service;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memtable_basic_interface_should_work() {
        let store = MemTable::new();
        test_basic_interface(store);
    }

    #[test]
    fn memtable_get_all_should_work() {
        let store = MemTable::new();
        test_get_all(store);
    }


    // #[test]
    // fn memtable_iter_should_work() {
    //     let store = MemTable::new();
    //     test_get_iter(store);
    // }

    fn test_basic_interface(store: impl Storage) {
        let v = store.set("t1", "hello".into(), "world".into());
        assert!(v.unwrap().is_none());

        let v1 = store.set("t1", "hello".into(), "world1".into());
        assert_eq!(v1, Ok(Some("world".into())));

        let v = store.get("t1", "hello");
        assert_eq!(v, Ok(Some("world1".into())));

        assert_eq!(Ok(None), store.get("t1", "hello1"));
        assert!(store.get("t2", "hello").unwrap().is_none());

        assert_eq!(store.contains("t1", "hello"), Ok(true));
        assert_eq!(store.contains("t1", "hello1"), Ok(false));
        assert_eq!(store.contains("t2", "hello"), Ok(false));

        let v = store.del("t1", "hello");
        assert_eq!(v, Ok(Some("world1".into())));

        assert_eq!(Ok(None), store.del("t1", "hello1"));
        assert_eq!(Ok(None), store.del("t2", "hello"));
    }

    fn test_get_all(store: impl Storage) {
        store.set("t2", "k1".into(), "v1".into()).unwrap();
        store.set("t2", "k2".into(), "v2".into()).unwrap();
        let mut data = store.get_all("t2").unwrap();
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(data, vec![Kvpair::new("k1", "v1".into()), Kvpair::new("k2", "v2".into())])
    }

    // fn test_get_iter(store: impl Storage) {
    //     store.set("t2", "k1".into(), "v1".into()).unwrap();
    //     store.set("t2", "k2".into(), "v2".into()).unwrap();
    //     let mut data: Vec<_> = store.get_iter("t2").unwrap().collect();
    //     data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    //     assert_eq!(data, vec![Kvpair::new("k1", "v1".into()), Kvpair::new("k2", "v2".into())])
    // }
}
