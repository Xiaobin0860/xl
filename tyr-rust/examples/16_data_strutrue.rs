use itertools::Itertools;
use std::fmt;
use std::iter::FromIterator;

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let vec = vec![1, 2, 3, 4, 5];
    let s1 = &arr[..2];
    let s2 = &vec[..2];
    println!("s1: {:?}, s2: {:?}", s1, s2);

    // &[T] 和 &[T] 是否相等取决于长度和内容是否相等
    assert_eq!(s1, s2);
    // &[T] 可以和 Vec<T>/[T;n] 比较，也会看长度和内容
    assert_eq!(&arr[..], vec);
    assert_eq!(&vec[..], arr);

    let v = vec![1, 2, 3, 4];
    // Vec 实现了 Deref，&Vec<T> 会被自动解引用为 &[T]，符合接口定义
    print_slice(&v);
    // 直接是 &[T]，符合接口定义
    print_slice(&v[..]);
    // &Vec<T> 支持 AsRef<[T]>
    print_slice1(&v);
    // &[T] 支持 AsRef<[T]>
    print_slice1(&v[..]);
    // Vec<T> 也支持 AsRef<[T]>
    print_slice1(v);
    let arr = [1, 2, 3, 4];
    // 数组虽没有实现 Deref，但它的解引用就是 &[T]
    print_slice(&arr);
    print_slice(&arr[..]);
    print_slice1(&arr);
    print_slice1(&arr[..]);
    print_slice1(arr);

    // 这里 Vec<T> 在调用 iter() 时被解引用成 &[T]，所以可以访问 iter()
    let result = vec![1, 2, 3, 4]
        .iter()
        .map(|v| v * v)
        .filter(|v| *v < 16)
        .take(2)
        .collect::<Vec<_>>();
    println!("{:?}", result);

    let err_str = "bad happened";
    let input = vec![Ok(21), Err(err_str), Ok(7)];
    let it = input
        .into_iter()
        .filter_map_ok(|i| if i > 10 { Some(i * 2) } else { None });
    let result: Vec<_> = it.collect();
    println!("{:?}", result);
    assert_eq!(vec![Ok(42), Err(err_str)], result);

    let s = String::from("hello");
    // &String 会被解引用成 &str
    print_slice2(&s);
    // &s[..] 和 s.as_str() 一样，都会得到 &str
    print_slice2(&s[..]);
    // String 支持 AsRef<str>
    print_slice3(&s);
    print_slice3(&s[..]);
    print_slice3(s.clone());
    //cannot infer type for type parameter `U` declared on the function `print_slice4`
    // print_slice4(&s);
    // String 也实现了 AsRef<[u8]>，所以下面的代码成立
    // 打印出来是 [104, 101, 108, 108, 111]
    print_slice1(&s);
    print_slice1(&s[..]);
    print_slice1(s);

    let arr = ['h', 'e', 'l', 'l', 'o'];
    let vec = vec!['h', 'e', 'l', 'l', 'o'];
    let s = String::from("hello");
    let s1 = &arr[1..3];
    let s2 = &vec[1..3];
    // &str 本身就是一个特殊的 slice
    let s3 = &s[1..3];
    println!("s1: {:?}, s2: {:?}, s3: {:?}", s1, s2, s3);
    // &[char] 和 &[char] 是否相等取决于长度和内容是否相等
    assert_eq!(s1, s2);
    // &[char] 和 &str 不能直接对比，我们把 s3 变成 Vec<char>
    assert_eq!(s2, s3.chars().collect::<Vec<_>>());
    // &[char] 可以通过迭代器转换成 String，String 和 &str 可以直接对比
    assert_eq!(String::from_iter(s2), s3);
}

fn print_slice<T: fmt::Debug>(v: &[T]) {
    println!("{:?}", v);
}

fn print_slice1<T, U>(v: T)
where
    T: AsRef<[U]>,
    U: fmt::Debug,
{
    println!("{:?}", v.as_ref());
}

fn print_slice2(s: &str) {
    println!("{:?}", s);
}

fn print_slice3<T: AsRef<str>>(s: T) {
    println!("{:?}", s.as_ref());
}

#[allow(dead_code)]
fn print_slice4<T, U>(s: T)
where
    T: AsRef<U>,
    U: fmt::Debug,
{
    println!("{:?}", s.as_ref());
}
