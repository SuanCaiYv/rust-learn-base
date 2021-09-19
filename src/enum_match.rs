pub enum Company {
    APPLE(String),
    XIAOMI(String),
    HUAWEI(String),
    OPPO(String),
    VIVO(String)
}

// 枚举可以包含字段，类似结构体，但是不同的枚举都是同一个类型，这点和结构体不同
pub enum MsgStatus {
    // 正常信息
    OK(String),
    // 错误信息
    ERR(String),
    // 状态码，Rust支持枚举不同类型的值
    OTHER(i32)
}

impl MsgStatus {
    // 枚举类型也可以有方法
    pub fn print(&self) {
    }
}

impl Company {
    pub fn value(self) -> String {
        // 通过match来进行值获取
        match self {
            Company::APPLE(val) => val,
            Company::XIAOMI(val) => val,
            Company::HUAWEI(val) => val,
            Company::OPPO(val) => val,
            Company::VIVO(val) => val
        }
    }
}

pub fn set_phone() {
    let apple = Company::APPLE(String::from("Apple"));
    println!("{}", apple.value())
}

pub fn increment(op: Option<i32>) -> i32 {
    // 使用Option进行处理
    match op {
        Some(val) => val+1,
        None => 0
    }
}

pub fn num_fmt(val: i32) {
    match val {
        1 => {
            println!("one")
        }
        2 => {
            println!("two")
        }
        // 类似default
        _ => {
            println!("others")
        }
    }
}

// 只接受1
pub fn only_one(op: Option<i32>) {
    if let Some(1) = op {
        println!("only accept one")
    } else {
        println!("get not one")
    }
}