/// Rust把字符串常量设计成&str，即对于在常量池中的这个字符串的引用
/// 个人猜测，Rust对于常量池字符串的回收有自己的一套方式
pub fn test_string() {
    // 下面这两个写法完全一样
    let s1 = "aaa".to_string();
    let s2 = String::from("aaa");
    let s3 = "bbb";
    let mut s4 = "aaa".to_string();
    s4.push_str(s3);
    println!("{}", s4);
    // 因为获取的是借用，所有权还在，所以s3可以继续使用
    println!("{}", s3);

    let s5 = "ccc".to_string();
    let s6 = "ddd";
    let s7 = s5 + s6;
    // 到这里，s5所有权转移到了s7上，同时s6依旧保留所有权
    println!("{}", s7);

    let s8 = "qwer".to_string();
    let s9 = "asdf".to_string();
    let s10 = "zxcv".to_string();
    let s11 = s8 + &s9 + &s10;
    // 与下面这个完全等价
    // let s11 = format!("{}{}{}", s8, s9, s10);
    // Rust不支持String类型的索引，所以下面这个语法是错误的
    // s11[0];
    // 可以考虑使用这种语法
    let s12 = s11.as_bytes();
    s12[0];
}

/// 返回参数必须是&str而不能是str，Rust不支持这种语法
fn f(s: &String) -> &str {
    // 以下这段代码会报错，因为发生了悬垂引用
    // let tmp = String::from("aaa");
    // 这里我们仅仅获取字符串的子串的引用，子串所有权同父串，所以到这里就被释放了。
    // return &tmp[0..1];
    // 这里面s只能是借用，否则还是会发生刚刚说的所有权消失，s被释放，子串引用失效的现象。
    &s[0..1]
}