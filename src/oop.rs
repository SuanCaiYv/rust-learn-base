use std::time::*;
use chrono::Local;

trait Print {
    fn print(&self);
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Print for Dog {
    fn print(&self) {
        println!("T'm a dog: {}", self.name);
    }
}

impl Print for Cat {
    fn print(&self) {
        println!("I'm a cat: {}", self.name);
    }
}

/// Rust要求trait对象必须是[对象安全]的，就是指想要作为trait对象的trait的所有方法必须同时满足：
/// 1. 返回值类型不为Self(实现了这个trait的类型的别名，即不返回这个方法接收者类型)。
/// 2. 方法没有范型参数。
pub fn test_oop() {
    // Vec的类型是trait对象。
    // 类似于C的void指针，指出所有的元素必须实现了Print这个trait，因为Rust不允许保存保存不同类型，所以我们使用指针形式。
    // dyn指出执行动态分派，即在运行时判断具体调用哪个方法实现
    // 虽然是动态分派，但是编译时依旧会进行类型检查，保证trait对象一定实现了指定trait。
    let mut v1: Vec<Box<dyn Print>>;
    let dog1 = Dog {
        name: "大黑狗".to_string(),
    };
    let cat1 = Cat {
        name: "小狸花".to_string(),
    };
    let cat2 = Cat {
        name: "小奶牛".to_string(),
    };
    let cat3 = Cat {
        name: "大理花".to_string(),
    };
    v1 = Vec::new();
    v1.push(Box::new(dog1));
    v1.push(Box::new(cat1));
    v1.push(Box::new(cat2));
    v1.push(Box::new(cat3));
    for print in v1.into_iter() {
        print.print();
    }
    println!("这些是我逝去的宠物们，仅此缅怀它们给我带来的快乐和幸福！")
}