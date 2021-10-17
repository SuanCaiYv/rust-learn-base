use std::ops::{Add, Deref};
use std::ptr::drop_in_place;
use std::thread::*;

/// Rust的unsafe提供了五大功能：
/// 1. 解引用裸指针(类似C的指针)
/// 2. 调用不安全的函数或方法，调用其他语言
/// 3. 访问或修改可变静态变量(Rust对于静态变量涉及到数据竞争，所以默认不可修改)
/// 4. 实现不安全的trait
/// 5. 访问union中的字段(与C交互)
///
/// 这里需要说一下，unsafe{}代码仅仅关闭了Rust的内存安全检查，但是其他检查还在，比如借用检查(更多是对Rust类型的检查)，生命周期检查
/// 更多请见[《Rust死灵书》](https://doc.rust-lang.org/nomicon/index.html)
struct TmpHeap {
    nums: Vec<i32>,
}

// 定义一个unsafe函数，整个函数体都是unsafe的
unsafe fn increment(ptr: *mut i32) {
    *ptr += 1
}

static mut COUNT: i32 = 0;

// 当trait包含unsafe方法时，这就是一个不安全的trait
unsafe trait UnsafePrint {
    unsafe fn print(&self);
}

// 所以对于这样的trait的实现，也要unsafe的加持
unsafe impl UnsafePrint for TmpHeap {
    unsafe fn print(&self) {
        println!("ok.")
    }
}

// 关联类型trait，无论有多少个泛型类型，只会实现一次，把泛型放到trait内部处理，所以可以无歧义的调用
trait Val {
    type RET;
    fn val(&self) -> &Self::RET;
}

// 泛型trait，会出现对于每一个具体泛型类型均实现一遍的问题，然后通过标注指明具体调用的是哪一个实现
trait Value<T> {
    fn value(&self) -> &T;
}

struct GenericType<T> {
    val: T,
}

impl<T> Val for GenericType<T> {
    type RET = T;

    fn val(&self) -> &Self::RET {
        &self.val
    }
}

impl<T> Value<T> for GenericType<T> {
    fn value(&self) -> &T {
        &self.val
    }
}

struct Complex {
    a: i64,
    i: i64,
}

// 通过默认类型语法指出，当泛型类型缺失时，使用哪种类型
trait Multi<OTHER=Self> {
    type RET;

    fn multi(self, other: OTHER) -> Self::RET;
}

impl Multi for Complex {
    type RET = Complex;

    fn multi(self, other: Self) -> Self::RET {
        Complex {
            a: self.a * other.a - self.i * other.i,
            i: self.i * other.a + self.a * other.i,
        }
    }
}

fn f1<T>(t: T) {}

// 指出可以传递一个T类型的指针，指针大小是确定的，所以这是可行的
fn f2<T: ?Sized>(t: &T) {}

fn inc(a: i32, b: i32) -> i32 {
    a + b
}

// 不同于闭包，闭包是一个实现了trait的匿名类型的方法；而函数是一个类型，而不是一个trait，它实现了三个闭包的trait(Fn, FnMut和FnOnce)
// fn(参数列表) -> (返回值或空)  指出了一个函数的类型，类似Golang，通过参数列表+返回值进行函数类型区分
fn add_twice(func: fn(i32, i32) -> i32, val: i32) -> i32 {
    func(val, 1) + func(val, 1)
}

// 每一个闭包都实现了三个闭包trait之一，所以可以通过Box进行包装返回，但是无法直接返回。
// 结合Box类型语法，我们可以大胆猜测一下，每一个闭包都是一个实现了三个trait之一的类型的方法，但是这个类型是匿名的，所以我们
// 返回一个包装进了Box的闭包本质是返回了一个包装了这个匿名类型的Box。
fn closure_ret() -> Box<dyn Fn(i32) -> i32> {
    return Box::new(|val| -> i32 {val+1})
}

// 而因为函数本身就是一个类型，所有的函数都是具体的类型(当然这些类型可能不同)，所以可以被直接返回，闭包不是类型，他是匿名类型的方法，需要借助匿名类型来返回
fn func_ret() -> fn(i32) -> i32 {
    fn tmp(a: i32) -> i32 {
        a+1
    };
    tmp
}

/// 当有多个trait的方法名一样时，且实现类型也有同名方法，则可以使用完全限定语法调用：
/// <Type as Trait>::function(receiver_if_method, next_arg, ...);
///
/// 此外，Rust支持别名，即对于一个很复杂的类型，可以使用别名来引用
///
/// Rust还支持返回无类型'!'，对，这个❕就是一个特殊的类型，它表示没有类型，对于continue/panic之类的，可以返回!作为返回值类型，此时编译器会忽略此时的返回值
/// 在返回两个不同的类型时，其中一个返回!是很有用的，因为会被忽略，所以实际返回值类型是另一个返回值得类型
pub fn test_unsafe() {
    let mut val = 12;
    // 有两种裸指针：*const T/*mut T；分别是不可变和可变的，前面的'*'是裸指针类型的一部分
    // 可以在普通代码里创建裸指针，但是无法解引用
    let ptr1 = &val as *const i32;
    // 这里需要注意，裸指针允许可变和不可变指针同时存在，此外无法实现自动垃圾清理
    let ptr2 = &mut val as *mut i32;
    let ptr3 = &mut TmpHeap {
        nums: Vec::new()
    } as *mut TmpHeap;
    unsafe {
        println!("{}", (*ptr3).nums.len());
        *ptr2 += 1;
        println!("{}", *ptr1);
        // 调用unsafe函数
        increment(ptr2);
        println!("{}", *ptr1)
    }
    // 调用其他语言这个暂时我应该用不到，所以不演示了

    // 全局变量会被多个线程共享，因此可能造成潜在的竞争问题，所以Rust不推荐使用，更不用说修改了，但是没法修改的全局变量就是常量
    // 这一点有点冲突我们使用全局变量的意图了，所以引入了unsafe{}来处理
    let handle1 = spawn(|| {
        unsafe {
            for i in 0..10 {
                // 在unsafe中修改全局变量
                COUNT += 1;
            };
        };
    });
    handle1.join();
    unsafe {
        // 在unsafe中访问全局变量
        println!("{}", COUNT)
    }
    let a = GenericType {
        val: 12,
    };
    println!("{}", a.val());
    println!("{}", a.value());
    let b = GenericType {
        val: "abc".to_string(),
    };
    println!("{}", b.val());
    println!("{}", b.value());
    // Rust是强类型静态的语言，必须在编译时知道类型大小，如果不知道则会报错，所以默认情况下，每个类型都实现了Sized这个trait
    // 这个trait指出这个类型的大小是确定的，而有时我们真的没法确定，就需要传递一个此类型的指针，指针大小是固定的u64，所以此时
    // 传递&T会被认为传递一个一个T: ?Sized类型的参数，其中?表示T可能实现了Sized也可能没实现。

    // 直到今天，我才意识到Rust定义指针类型用的是t: &T这种语法，而不是t: *T；后者是C语言的方式，属实丢人了
    let c: &Complex;
    c = &Complex {
        a: 1,
        i: 2,
    };
    println!("{}", func_ret()(1))
}