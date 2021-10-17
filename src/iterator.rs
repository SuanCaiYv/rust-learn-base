struct NumList {
    list: Vec<i32>,
    idx: usize,
}

impl Iterator for NumList {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.list.len() {
            None
        } else {
            let val = self.list[self.idx];
            self.idx += 1;
            Some(val)
        }
    }
}


pub fn test_iterator() {
    let mut v1 = vec![1, 2, 3, 4, 5];
    // 迭代器是惰性求值的，只要你不调用，不使用迭代器来获取元素，那就没有任何数据产生
    let iter1 = v1.iter_mut();
    for i in iter1 {
        println!("{}", i)
    }
    // iter()是不获得序列的元素的所有权的，into_iter()获得所有权，iter_mut()返回可变借用，也是不获得所有权的
    // 借用只是不获得所有权，不代表不能更新值
    // 此外，标准库提供了一堆适配器迭代器，即建立在iter()之上，进行其他操作的迭代器，比如map()，filter()，sum()
    // 可以理解成流式处理，iter()提供了类似for-each()的产生源的作用
    // 可以传入一个闭包(lambda表达式)，实现对序列的更新，得到新的序列，Java的流式处理还记得嘛
    // 自定义迭代器很简单，实现了迭代器接口的next()方法即可
    let mut list = NumList {
        list: Vec::new(),
        idx: 0,
    };
    for i in 0..10 {
        list.list.push(i)
    }
    loop {
        match list.next() {
            None => {
                break
            }
            Some(val) => {
                println!("{}", val)
            }
        }
    }
}