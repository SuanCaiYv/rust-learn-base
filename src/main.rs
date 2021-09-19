mod base;
mod ownership;
mod struct0;
mod enum_match;
mod vector;

use std::io;
use crate::base::*;
use crate::ownership::*;
use crate::struct0::*;
use crate::enum_match::*;
use std::io::{stdin, Read};

fn main() {
    chap6()
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
