use std::collections::HashMap;
use std::thread;
use std::thread::sleep;
use std::time;
use std::time::UNIX_EPOCH;

/// 闭包像是一个匿名函数，但是功能又比匿名函数多一点
pub fn test_closure() {
    // 定义一个闭包
    let func1 = |x: i32, y: i32| -> i32 {
        x + y
    };
    println!("{}", func1(1, 2));
    // 闭包自带类型推断，所以下面这几个都是对的
    // fn  add_one_v1   (x: u32) -> u32 { x + 1 };
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x|             { x + 1 };
    // let add_one_v4 = |x|               x + 1  ;
    // 所有的闭包都实现了Fn/FnMut/FnOnce这三个之一，即，所有的闭包都是这三个的子类型或实现类型
    // Fn: 获取当前上下文中的不可变借用值，即，不取得值的所有权
    // FnMut: 获取可变的借用值，但是也不取得所有权
    // FnOnce: 获取值的所有权，所以只能被调用一次，因为一个值只有一个所有权
    let func2 = |x: i32, y: i32| -> i32 {
        sleep(time::Duration::from_secs(2));
        x+y
    };
    let tmp = String::from("cauliflower");
    // 通过move关键字强制获取所有权
    let func3 = move || -> i32 {
        tmp.len() as i32
    };
    // 下面这个会编译错误，因为闭包获得所有权发生在编译时，而不是运行时，所以并不是直到调用才转移所有权，而是在编译时就转移了。
    // "在定义闭包时将上下文的值的所有权移动进闭包"
    // println!("{}", tmp);
    println!("{}", func3());
    // 最后多嘴一些，闭包有三种获得上下文值的所有权的级别，分别是FnOnce/FnMut/Fn。它们的所有权大小依次递减。
    // 因为FnOnce所有权最大，所以所有的闭包默认实现了它。
    // 这里给出一个建议，就是闭包最好遵循尽可能获得小的所有权，即如果闭包不获取任何上下文的所有权且不更改其值，就用Fn
    // 如果需要更改值但是不获得所有权，则使用FnMut，如果需要所有权则使用FnOnce。这可以在定义时显式地指定。
    let v1;
    let v2;
    let v3;
    let curr1;
    let curr2;
    let curr3;
    let mut cache = Cache::new(func2);
    let curr = time::SystemTime::now().duration_since(UNIX_EPOCH).expect("err").as_millis();
    {
        v1 = cache.value(1, 2);
        curr1 = time::SystemTime::now().duration_since(UNIX_EPOCH).expect("err").as_millis() - curr;
        println!("{}-{}", v1, curr1);
    }
    {
        v2 = cache.value(3, 4);
        curr2 = time::SystemTime::now().duration_since(UNIX_EPOCH).expect("err").as_millis() - curr;
        println!("{}-{}", v2, curr2);
    }
    {
        v3 = cache.value(1, 2);
        curr3 = time::SystemTime::now().duration_since(UNIX_EPOCH).expect("err").as_millis() - curr;
        println!("{}-{}", v3, curr3);
    }
    {
        println!("{}", cache.size())
    }
}

/// 定义一个缓存器，用来缓存闭包的结果，此外，闭包可以看成是一种特殊的类型，因为它也存在类型的继承关系
struct Cache<T> where T: (Fn(i32, i32) -> i32) {
    closure: T,
    val_map: HashMap<String, i32>,
}

impl<T> Cache<T> where T: (Fn(i32, i32) -> i32) {
    fn new(closure0: T) -> Cache<T> {
        Cache {
            closure: closure0,
            val_map: HashMap::new(),
        }
    }

    fn value(&mut self, x: i32, y: i32) -> &i32 {
        let str = format!("{}+{}", x, y);
        self.val_map
            .entry(str)
            .or_insert((self.closure)(x, y))
    }

    fn size(&self) -> usize {
        self.val_map.len()
    }
}