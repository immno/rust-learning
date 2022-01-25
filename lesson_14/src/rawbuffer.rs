use std::{fmt, slice};
use std::fmt::Formatter;

// #[derive(Clone, Copy)]
struct RawBuffer {
    ptr: *mut u8,
    len: usize,
}

impl From<Vec<u8>> for RawBuffer {
    fn from(vec: Vec<u8>) -> Self {
        let slice = vec.into_boxed_slice();
        Self {
            len: slice.len(),
            // into_raw 之后，Box 就不管这块内存的释放了，RawBuffer 需要处理释放
            ptr: Box::into_raw(slice) as *mut u8,
        }
    }
}

// 如果 RawBuffer 实现了 Drop trait，就可以在所有者退出时释放堆内存
// 然后，Drop trait 会跟 Copy trait 冲突，要么不实现 Copy，要么不实现 Drop
// 如果不实现 Drop，那么就会导致内存泄漏，但它不会对正确性有任何破坏
// 比如不会出现 use after free 这样的问题。
// 你可以试着把下面注释去掉，看看会出什么问题
impl Drop for RawBuffer {
    #[inline]
    fn drop(&mut self) {
        let data = unsafe {
            Box::from_raw(slice::from_raw_parts_mut(self.ptr, self.len))
        };
        drop(data)
    }
}


impl fmt::Debug for RawBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let data = self.as_ref();
        write!(f, "{:p}: {:?}", self.ptr, data)
    }
}

impl AsRef<[u8]> for RawBuffer {
    fn as_ref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

fn main() {
    let data = vec![1, 2, 3, 4];
    let buf: RawBuffer = data.into();
    use_buffer(buf);
    // println!("buf: {:?}", buf);
}

fn use_buffer(buf: RawBuffer) {
    println!("buf to die: {:?}", buf);
    drop(buf)
}

fn rc_is_not_send_and_sync() {
    let a = std::rc::Rc::new(1);
    let b = a.clone();
    let c = a.clone();
    // std::thread::spawn(move || {
    //     println!("c={:?}", c);
    // })
}

fn refcell_is_send(){
    let a =  std::cell::RefCell::new(1);
    std::thread::spawn(move || {
        println!("a={:?}", a);
    });
}

fn refcell_is_not_sync(){
    // RefCell 现在有多个 Arc 持有它，虽然 Arc 是 Send/Sync，但 RefCell 不是 Sync
    let a = std::sync::Arc::new(1);
    let b = a.clone();
    let c = a.clone();
    // std::thread::spawn(move || {
    //     println!("c={:?}", c);
    // })
}