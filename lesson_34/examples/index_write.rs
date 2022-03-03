// use std::sync::mpsc::Sender;
// use std::thread;
//
// pub struct IndexInner {
//     index: Index,
//     reader: IndexReader,
//     config: IndexConfig,
//     updater: Sender<Input>,
// }
//
// pub struct IndexUpdater {
//     sender: Sender<Input>,
//     t2s: bool,
//     schema: Schema,
// }
//
// impl Indexer {
//     pub fn open_or_create(config: IndexConfig) -> Result<Self> {
//         let schema = config.schema.clone();
//         let index = if let Some(dir) = &config.path {
//             fs::create_dir_all(dir)?;
//             let dir = MmapDirectory::open(dir)?;
//             Index::open_or_create(dir, schema.clone())?
//         } else {
//             Index::create_in_ram(schema.clone)
//         };
//
//         Self::set_tokenizer(&index, &config);
//         let mut writer = index.writer(config.writer_memory)?;
//
//         let (s, r) = unbounded::<Imput>();
//
//         // 启动一个线程，从 channel 的 reader 中读取数据
//         thread::spawn(move || {
//             for input in r { // 然后用 index writer 处理这个 input
//                 if let Err(e) = input.process(&mut writer, &schema) { warn!("Failed to process input. Error: {:?}", e); }
//             }
//         });
//
//         Self::new(index, config, s)
//     }
//
//     pub fn get_updater(&self) -> IndexUpdater {
//         let t2s = TextLanguage::Chinese(true) == self.config.text_lang;
//         IndexUpdater::new(self.updater.clone(), self.index.schema(), t2s)
//     }
// }
//
fn main() {}