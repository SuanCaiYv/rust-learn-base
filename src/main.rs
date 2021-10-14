mod base;
mod ownership;
mod struct0;
mod enum_match;
mod vector;
mod string0;
mod generic;
mod trait0;
mod lifetime0;
mod function;
mod iterator;
mod pointer;
mod thread0;
mod test;
mod oop;
mod design_mode;
mod unsafe0;

use std::io;
use crate::base::*;
use crate::ownership::*;
use crate::struct0::*;
use crate::enum_match::*;
use crate::vector::*;
use crate::struct0::*;
use crate::string0::*;
use crate::generic::*;
use crate::trait0::*;
use crate::lifetime0::*;
use crate::function::*;
use crate::iterator::*;
use crate::pointer::*;
use crate::thread0::*;
use crate::test::test;
use crate::oop::*;
use crate::design_mode::*;
use crate::unsafe0::*;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, Rust.")
}

fn chap1_3() {
    println!("{}", add(1, 2));
    var();
    control();
}

fn chap4() {
    let str1 = String::from("aaa");
    move_ownership(str1);
    // 取消注释，这里会报错，因为所有权已经转移
    // println!("{}", str1)
    let str2 = String::from("bbb");
    let str3 = move_ownership_and_giveback(str2);
    println!("{}", str3);
    let str4 = String::from("ccc");
    reference_ownership(&str4);
    println!("{}", str4);
    let mut str5 = String::from("ddd");
    reference_ownership_mut(&mut str5);
    println!("{}", str5);
    note_chap4();
}

fn chap5() {
    let phone1 = build_phone(String::from("apple"), String::from("13 pro"), String::from("0001"), 0001);
    let phone2 = build_phone_tuple(String::from("apple"), String::from("13"), String::from("0002"), 0002);
    let phone3 = Phone::default();
    println!("{}", phone2.2);
    phone1.display();
    phone3.display();
}

fn chap6() {
    set_phone();
    let ans1 = increment(Option::Some(3));
    println!("{}", ans1);
    let ans2 = increment(Option::None);
    println!("{}", ans2);
    num_fmt(1);
    num_fmt(2);
    num_fmt(3);
    only_one(Some(1));
    only_one(Some(2));
}

fn chap8() {
    just_display_vec();
    test_string();
}

fn chap9() {
    test_double_val();
    test_trait();
    test_lifetime();
}

fn chap13() {
    test_closure();
    test_iterator();
}

fn chap15() {
    test_box();
    test_deref();
}

fn chap16() {
    test_thread();
}

fn chap17() {
    test_oop();
}

fn chap18() {
    design_mode_note();
}
