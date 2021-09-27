use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    val: i32,
    next: List,
}

struct List {
    size: i32,
    // 这样直接定义是不行的，因为Rust无法定义指针，所以只能定义值，而值定义会触发递归类型
    // next: Node,
    node: Box<Node>
}

struct Tmp {
    val: i32,
    name: String,
}

struct MyBox<T> {
    val: T
}

impl<T> MyBox<T> {
    fn new(v: T) -> MyBox<T> {
        MyBox {
            val: v,
        }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    // 重载解引用运算符
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl<T> Drop for MyBox<T> {

    // 自定义析构函数，会在变量离开作用域时，自动调用
    fn drop(&mut self) {
        println!("dropped!")
    }
}

pub fn test_box() {
    let b1 = Box::new(Tmp {
        val: 12,
        name: "cauliflower".to_string(),
    });
    // Box里面保存的是一个指针，指向在堆上分配的对象
    // 同时它实现了Deref这个trait，所以即使它是个指针，我们也可以把它当成对象引用来使用
    println!("{}", b1.name)
}

struct Tmp1 {
    val: i32,
}

struct Tmp2 {
    val: *mut Tmp1,
}

struct Tmp3 {
    val: *mut Tmp1,
}

struct Tmp4 {
    val: Rc<Tmp1>,
}

struct Tmp5 {
    val: Rc<Tmp1>,
}

struct MutRef<T> {
    val: RefCell<Vec<T>>,
}

impl<T> MutRef<T> {
    fn add(&self, v: T) {
        // 如果不用RefCell，这里会报错，因为self是不可变的，所以禁止进行字段的更新
        self.val.borrow_mut().push(v)
    }
}

pub fn test_deref() {
    let a = 12;
    let b = &a;
    assert_eq!(a, 12);
    assert_eq!(*b, 12);
    // 现在来重新整理一下引用与借用，Rust的引用就是C++里面的指针，可以理解成指针不拥有所有权，但是拥有值
    // 值的所有权只能被变量所拥有，每一个值在任一时刻都有唯一一个变量，最开始的变量就是创建这个值的变量
    // 后面可能根据所有权转移被转移到另一个变量，此时这个新的变量就是这个值的当下变量。
    // 对指针解引用只会得到值，而不会触发所有权获取。

    let c = Box::new(12);
    // 因为Box重载了解引用'*'运算符，所以对于Box的解引用是等同于普通指针的解引用的。
    assert_eq!(*c, 12);

    let d = MyBox::new(12);
    // 解引用生效，因为最后等同于调用*(d.deref())
    assert_eq!(*d, 12);
    // 此外，Rust会自动在必要的地方进行强制类型转换的触发

    // 提前释放值，我们无法手动调用.drop()，但是可以提醒编译器在这里释放空间，因为我们用完了资源，想早点节省空间
    drop(d);

    let mut tmp1 = Tmp1 {
        val: 12,
    };
    // 取消注释会报错，因为我们提前释放了值，造成后面的悬垂引用
    // drop(tmp1);
    let tmp2 = Tmp2 {
        val: &mut tmp1,
    };
    let tmp3 = Tmp3 {
        val: &mut tmp1,
    };
    // tmp1的所有权转移到ref1
    let ref1 = Rc::new(tmp1);
    let tmp4 = Tmp4 {
        val: Rc::clone(&ref1),
    };
    {
        let tmp5 = Tmp5 {
            val: Rc::clone(&ref1),
        };
        println!("count: {}", Rc::strong_count(&ref1));
    }
    println!("count: {}", Rc::strong_count(&ref1));
    // 每次变量离开作用域，计数器-1，计数器变为0时，释放资源

    // RefCell提供了对不可变引用执行更新的方法，即，实现了运行时借用检查，绕过了编译期的限制
    let mut mr1 = MutRef {
        val: RefCell::new(vec![0])
    };
    mr1.add(1);
    // RefCell提供两个不同的借用策略，分别是不可变借用borrow()和可变借用borrow_mut()
    // RefCell打破了编译时所有权借用检查，放到了运行时，同时维护两个计数器，任何时刻可变借用计数器必须 <= 1
    // 且当不可变计数器 > 0时，可变计数器必须等于0，否则panic。
    // 说白了就是它在运行时使用unsafe操作在内部维护借用规则：任一时刻只能有多个不可变引用/一个可变引用。
    // 这里我们还可以组合RefCell和Rc来实现类似volatile的操作，即对某个值更新，然后所有拥有它的所有权的方法均能看到更新；
    // 因为Rc保证所有拥有所有权的方法得到的一定是不可变借用，所以不存在数据竞争和借用规则冲突问题。
    for i in mr1.val.borrow().iter() {
        println!("{}", i)
    }

    // 最后，处理Rc的循环引用导致的内存泄漏可以考虑使用Weak<T>引用，即虚引用来实现，!todo()
}