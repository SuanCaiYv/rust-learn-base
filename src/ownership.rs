/// 尝试所有权转移
pub fn move_ownership(str : String) {
    println!("already moved ownership");
}

/// 转移所有权同时通过返回值转移回去
pub fn move_ownership_and_giveback(str : String) -> String {
    println!("already moved ownership and giveback");
    str
}

/// 通过引用使用值，不会取得所有权，引用默认为不可更改的
pub fn reference_ownership(str : &String) {
    println!("just reference ownership")
}

/// 通过可变引用追加值
pub fn reference_ownership_mut(str : &mut String) {
    println!("just reference ownership mut");
    str.push_str(" add msg");
}

/// 一些笔记
pub fn note_chap4() {
    println!("对于同一个变量的引用，在同一作用域下，只能同时存在多个 不可变引用或一个可变引用");
    println!("不可以使用一个值已经被释放了的引用(空指针)");
}

/// 使用切片获取部分序列的所有权，同样适用于元组和数组
pub fn sub_str(str : &String, mut from : usize, mut to : usize) -> &str {
    let len = str.len();
    if to > len {
        to = len
    }
    if from <= 0 {
        from = 0
    }
    &str[from..to]
}