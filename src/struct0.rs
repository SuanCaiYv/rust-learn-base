pub struct Phone {
    // 定义结构体字段，并指出访问权限，默认private
    pub factory: String,
    pub series: String,
    pub seq_num: String,
    pub id: u64,
}

/// 结构体属性的所有权和结构体对象绑定，一旦属性所有权转移，必须转移对象所有权，无法做到保留对象所有权的同时，转移属性所有权
impl Phone {
    // 带self的则是实例方法
    pub fn display(&self) {
        println!("厂商: {}, 系列: {}, 序列号: {}, ID: {}", self.factory, self.series, self.seq_num, self.id)
    }

    // 不带self的方法，类似静态方法，不属于某一具体对象
    pub fn default() -> Phone {
        Phone {
            factory: String::from("xiaomi"),
            series: String::from("mix4"),
            seq_num: String::from("0001"),
            id: 0001
        }
    }
}

// 结构体元祖，类似于匿名结构体，同时指出访问权限
pub struct PhoneTuple(pub String, pub String, pub String, pub u64);

// 空结构体，一般用于trait标记
#[derive(Debug)]
struct PhoneMarked {}

pub fn build_phone(factory0: String, series0: String, seq_num0: String, id0: u64) -> Phone {
    // 构造一个结构体
    Phone {
        factory: factory0,
        series: series0,
        seq_num: seq_num0,
        id: id0
    }
}

pub fn build_phone_tuple(factory0: String, series0: String, seq_num0: String, id0: u64) -> PhoneTuple {
    // 构造一个匿名结构体
    PhoneTuple(factory0, series0, seq_num0, id0)
}