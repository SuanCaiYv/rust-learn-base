pub fn just_display_vec() {
    let mut vec1 = vec![1, 2, 3];
    for i in 0..vec1.len() {
        print!("{} ", vec1[i])
    }
    println!();
    // get()和下标访问不同的地方在于，它返回一个Option，这样可以减少panic的可能性
    let val1 = vec1.get(1);
    if val1.is_some() {
        println!("{}", val1.unwrap())
    } else {
        println!("{}", -1)
    }
    // 这里会编译失败，因为禁止在获得引用的情况下，进行数据添加，可能会触发扩容，结果是元素被移动
    // let first = &vec1[0];
    // vec1.push(4);
    // println!("{}", first);
    let mut vec2 = vec![1, 2, 3];
    for val in &mut vec2 {
        *val += 1;
    }
    for val in vec2 {
        print!("{} ", val);
    }
    // Vector的所有权被回收，里面的元素也会被回收
    println!();
    let mut vec3 = Vec::new();
    // 通过枚举类型实现向Vector中添加不同的值
    vec3.push(MultiHttpMsg::Ok(String::from("passed")));
    vec3.push(MultiHttpMsg::ClientErr(String::from("token err")));
    vec3.push(MultiHttpMsg::IntervalErr(String::from("OOM")));
    vec3.push(MultiHttpMsg::Code(404));
    for msg in vec3 {
        println!("{}", msg.to_string())
    }
}

enum MultiHttpMsg {
    Ok(String),
    ClientErr(String),
    IntervalErr(String),
    Code(i32)
}

impl MultiHttpMsg {
    fn to_string(self) -> String {
        match self {
            MultiHttpMsg::Ok(val) => val,
            MultiHttpMsg::ClientErr(val) => val,
            MultiHttpMsg::IntervalErr(val) => val,
            MultiHttpMsg::Code(val) => val.to_string()
        }
    }
}