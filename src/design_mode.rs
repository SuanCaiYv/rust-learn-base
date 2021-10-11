/// 状态模式，指的是根据结构体的字段的状态，产生不同的行为。
/// 不可反驳性指的是，某个变量可以被赋予任意的值，而反之则指的是，某个表达式，只能被赋予同一类型的值。
/// 这里举一个小例子，最普通的赋值语句let x = ???就是一个不可反驳性，因为x可以被赋予任意类型
/// 而 let Some(x) = None就是可反驳性，因为x只能被赋予Some()类型的值
/// 此外，因为可反驳性赋值表达式运算结果可能为false，所以我们可以使用if let语句进行赋值错误处理
pub fn design_mode_note() {
    let a = Some(12);
    let b: Option<i32>;
    b = None;
    if let Some(x) = b {
    } else {
        println!("get assign err")
    }
}