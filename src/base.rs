pub fn var() {
    // 定义一个不可变int
    let var1 = 10;
    // 定义一个不可变long
    let var2 :i64 = 20;
    // 定义一个可变int
    let mut var3 = 15;
    var3 = 25;
    // 重载了var1的类型和值，依旧不可变
    let var1 = "str1";
    // 定义一个常量
    const NUM_OF_STR :i32 = 4;
    let var4 = 1.0;
    let var5 = true;
    // Rust的char编码为4字节，Unicode编码，不同于其他语言
    let var6 = '😊';
    // 定义一个元组
    let tup1 = (1, 2, 3);
    // 元组元素类型可以不一致，但是不可变
    let tup2 = (1, 2.0, false);
    // 定义一个数组，数组类型必须一致，依旧不可变
    let arr1 = [1, 2, 3];
}

pub fn add(x :i32, y :i32) -> i32 {
    // 利用表达式作为返回值
    // 除了显式地使用return之外，表达式最后一个没有分号的语句将作为返回值
    // 表达式同理
    x + y
}

pub fn control() {
    let arr1 = [1, 2, 3];
    let mut idx = 0;
    let ans1 = loop {
        if idx >= arr1.len() {
            break idx * 2;
        }
        print!("arr1: {} ", arr1[idx]);
        idx += 1;
    };
    println!("\n{}", ans1);
    for num in arr1 {
        println!("{}", num)
    }
}