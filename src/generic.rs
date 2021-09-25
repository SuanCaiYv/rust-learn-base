pub struct DoubleVal<T, U> {
    pub x: T,
    pub y: U,
}

impl<T, U> DoubleVal<T, U>{
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    /// 某一范型方法可以接受另一个范型的参数
    fn swap<V, W>(self, val: DoubleVal<V, W>) -> DoubleVal<T, W> {
        DoubleVal {
            x: self.x,
            y: val.y,
        }
    }
}

// 只有当T和U都是i32类型是，这个方法才可见
impl DoubleVal<i32, i32> {
    fn sum(&self) -> i32 {
        self.x + self.y
    }
}

pub fn test_double_val() {
    let v1 = DoubleVal {
        x: 1,
        y: 2.0,
    };
    let v2 = DoubleVal {
        x: 3,
        y: String::from("aaa"),
    };
    println!("{}", v1.y);
    println!("{}", v2.x);
    let v3 = v1.swap(v2);
    println!("{}", v3.x);
    println!("{}", v3.y);
    let v01 = DoubleVal {
        x: 11,
        y: 20,
    };
    // 均为i32时的独有方法
    println!("{}", v01.sum());
}