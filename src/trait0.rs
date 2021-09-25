pub trait Info {
    fn info(&self) -> String;

    fn print(&self);

    fn hello(&self) {
        println!("Hello! I'm Default");
        // 默认方法中可以调用别的方法
        self.print();
    }
}

pub trait Str {
    fn to_str(&self) -> String;
}

pub struct Person {
    name: String,
    age: i32,
    address: String,
}

// Person实现了Info这个trait
impl Info for Person {
    fn info(&self) -> String {
        format!("My name: {}, age: {}, and how to find me: {}", self.name, self.age, self.address)
    }

    fn print(&self) {
        println!("I'm: {}", self.name)
    }
}

// Person实现了Str这个trait
impl Str for Person {
    fn to_str(&self) -> String {
        format!("{}-{}-{}", self.name, self.age, self.address)
    }
}

pub fn test_trait() {
    let p1 = Person {
        name: "aaa".to_string(),
        age: 21,
        address: "bbb".to_string(),
    };
    p1.hello();
    trait_as_param2(&p1);
    trait_as_param2(&p1);
    trait_as_param3(&p1);
    trait_as_param4(&p1);
}

fn trait_as_param1(val: &impl Info) {
    val.print();
}

fn trait_as_param2<T: Info>(val: &T) {
    val.print();
}

fn trait_as_param3<T: Info + Str>(val: &T) {
    println!("{}", val.to_str());
    val.hello();
}

fn trait_as_param4<T>(val: &T)
    where T: Info + Str
{
    println!("{}", val.to_str());
    val.hello();
}