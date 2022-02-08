use crate::pb::{CommandRequest, CommandResponse, Hget, Kvpair};
use crate::pb::command_request::RequestData;
use crate::pb::value::Value;

mod pb;

fn main() {
    println!("Hello, world!");
}

// 对Command的处理的抽象
// pub trait CommandService {
//     /// 处理Command，返回Response
//     fn execute(self, store: &impl Storage) -> CommandResponse;
// }
//
// pub fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
//     match cmd.request_data {
//         Some(RequestData::Hget(param)) => param.execute(store),
//         Some(RequestData::Hgetall(param)) => param.execute(store),
//         Some(RequestData::Hset(param)) => param.execute(store),
//         None => KvError::InvalidCommand("Request has no data".into()).into(),
//         _ => KvError::Internal("Not implemented".into()).into(),
//     }
// }
//
//
// /// 对存储的抽象，我们不关心数据存在哪儿，但需要定义外界如何和存储打交道
// pub trait Storage {
//     /// 从一个 HashTable 里获取一个 key 的 value
//     fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
//     /// 从一个 HashTable 里设置一个 key 的 value，返回旧的 value
//     fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;
//     /// 查看 HashTable 中是否有 key
//     fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
//     /// 从 HashTable 中删除一个 key
//     fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
//     /// 遍历 HashTable，返回所有 kv pair（这个接口不好）
//     fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;
//     /// 遍历 HashTable，返回 kv pair 的 Iterator
//     fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item=Kvpair>>, KvError>;
// }
//
// impl CommandService for Hget {
//     fn execute(self, store: &impl Storage) -> CommandResponse {
//         match store.get(&self.table, &self.key) {
//             Ok(Some(v)) => v.into(),
//             Ok(None) => KvError::NotFound(self.table, self.key).into(),
//             Err(err) => err.into(),
//         }
//     }
// }
