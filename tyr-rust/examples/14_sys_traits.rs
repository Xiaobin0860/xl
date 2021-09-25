#[derive(Debug, Clone)]
struct Developer {
    name: String,
    age: u8,
    lan: Language,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum Language {
    Rust,
    Elixir,
    CPlusPlus,
}

use std::{fmt, slice};

// 注意这里，我们实现了 Copy，这是因为 *mut u8/usize 都支持 Copy
#[derive(Clone)]
struct RawBuffer {
    // 裸指针用 *const / *mut 来表述，这和引用的 & 不同
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
        let data = unsafe { Box::from_raw(slice::from_raw_parts_mut(self.ptr, self.len)) };
        println!("drop {:?}, data={:?}", self, data);
        drop(data)
    }
}

impl fmt::Debug for RawBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = self.as_ref();
        write!(f, "{:p}: {:?}", self.ptr, data)
    }
}

impl AsRef<[u8]> for RawBuffer {
    fn as_ref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

fn use_buffer(buf: &RawBuffer) {
    println!("buf to die: {:?}", buf);

    // 这里不用特意 drop，写出来只是为了说明 Copy 出来的 buf 被 Drop 了
    drop(buf)
}

use std::{
    sync::{Arc, Mutex},
    thread,
};

// Arc<Mutext<T>> 可以多线程共享且修改数据
fn arc_mutext_is_send_sync() {
    let a = Arc::new(Mutex::new(1));
    let b = a.clone();
    let c = a.clone();
    let handle = thread::spawn(move || {
        let mut g = c.lock().unwrap();
        *g += 1;
    });

    {
        let mut g = b.lock().unwrap();
        *g += 1;
    }

    handle.join().unwrap();
    println!("a= {:?}", a);
}

/*今天学习了必须要掌握的Trait
    内存：
    Clone: clone, clone_from, #[derive(Clone)]
    Copy: 标记trait, 用作trait bound类型安全检查, #[derive(Copy)]
    Drop: drop资源回收
    Copy和Drop互斥
    标记trait：
    Sized: 标记有具体大小类型, 编译器自动加, ?Size摆脱约束
    Send: 线程间移动
    Sync: 线程间共享
    Unpin: 自引用类型
    类型转换：
    From<T>, Into<T>, TryFrom<T>, TryInto<T>: 值类型到值类型转换
    AsRef<T>, AsMut<T>: 引用类型到引用类型转换
    操作符：
    运算符trait: 为自己的类型重载运算符
    Deref, DerefMut: 自动调用
    其他：
    Debug: 调试印, 派生宏直接生成 {:?}
    Display: 用户显示 {}
    Default: 为类型提供缺省值
*/
fn main() {
    let dev = Developer {
        name: "xl000".to_string(),
        age: 30,
        lan: Language::Rust,
    };
    let dev1 = dev.clone();
    println!("{:?}, addr of dev name: {:p}", dev, dev.name.as_str());
    println!("{:?}, addr of dev1 name: {:p}", dev1, dev1.name.as_str());

    let data = vec![1, 2, 3, 4];
    let buf: RawBuffer = data.into();
    //Drop trait 会跟 Copy trait 冲突，要么不实现 Copy，要么不实现 Drop
    // 因为 buf 允许 Copy，所以这里 Copy 了一份
    use_buffer(&buf);
    // buf 还能用
    println!("buf: {:?}", buf);

    arc_mutext_is_send_sync();
}
