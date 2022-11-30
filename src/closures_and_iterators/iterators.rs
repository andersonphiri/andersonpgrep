use std::vec;

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

//
//
//

pub fn intro_iteration() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {val}");
    }
}

pub fn method_that_consume_iterator() {
    let v = vec![1, 2, 3];
    let v_iter = v.iter();
    let total: i32 = v_iter.sum(); // after this line, we are not able to use v_iter because it has
                                   // been consumed by sum method
}

pub fn method_that_produce_iterator() {
    let v = vec![1, 2, 3];
    let v2: Vec<_> = v.iter().map(|x| x + 3).collect();
    // filter items
    let v3 = vec![1, 2, 3];
    let od_nums: Vec<_> = v3.iter().filter(|x| (**x) & 1 == 1).collect();
}
//
// //
