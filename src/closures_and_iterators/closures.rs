use core::time;
use std::{collections::HashMap, thread};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShirtColor {
    Red, // (String),
    Blue, //(String)
}

pub struct Inventory {
    pub shirts: HashMap<String, u128>,
}
impl Inventory {
    pub fn most_stocked(&self) -> ShirtColor {
        let blue_count = self.shirts.get("Blue").copied().unwrap_or(0);
        let red_count = self.shirts.get("Red").copied().unwrap_or(0);
        if blue_count == 0 && red_count == 0 {
           ShirtColor::Red
        } else if red_count < blue_count {
            ShirtColor::Blue
        } else {
            ShirtColor::Red
        }
    }
    pub fn give_away(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| {
            self.most_stocked()
        })
    }
}

pub fn test_closure() {
    let store = Inventory{
        shirts: HashMap::from([
            ("Blue".to_string(), 20),
            ("Red".to_string(), 29)
        ])
    };

    let user_pref1 = Some(ShirtColor::Blue);
    let user_pref2 =None;
    let give_away_one = store.give_away(user_pref1);
    let give_away_two = store.give_away(user_pref2);
    println!("pref one: {:?}",give_away_one);
    println!("pref two: {:?}", give_away_two);
}

// defining closures
pub fn test_declaring_closures()  {
    let expensive_closure = |num : u32| -> u32 {
        println!("computing very late");
        thread::sleep(time::Duration::from_secs(3));
        num
    };
    // add one
    let add_one_closure = |x: u32| -> u32 {x + 1};
    let add_one  = |x: usize|  x + 1;
    let add_one_three  = |x: usize|  {x + 1};


}

pub fn ownership_taken(){
    let list = vec![1,2,3];
    println!("before defining closure {:?}", list );
    let only_borrows_closure = || println!("borrowing from closure {:?}", list);
    println!("before calling closure {:?}", list );
    only_borrows_closure();
    println!("after calling closure {:?}", list );
}

pub fn ownership_mutable_reference() {
    let mut list = vec![1,2,3,4];
    println!("before defining closure {:?}", list );
    let mut borrows_mutably = || list.push(5);
    // we cant print before calling closure
    // println!("before defining closure {:?}", list ); // will not compile
    borrows_mutably();
    println!("before defining closure {:?}", list ); 

}

pub fn fork_join_thread() {
    let mut list = vec![1,2,3,4];
    println!("before passing thread {:?}", list );
    // use move keyword to pass data to the thread
    thread::spawn(move || println!("printing from the thread {:?}", list))
    .join()
    .unwrap();

}

pub fn  generic_list_map<'a, TFrom,  TTo, F, 
>(source: & 'a mut [TFrom], handler: F) -> Vec<TTo>
where 
F : Fn(& mut TFrom) ->  TTo,
TFrom : PartialEq + PartialOrd,
TTo:  PartialEq + PartialOrd
{
    let mut result : Vec<TTo> = Vec::with_capacity(source.len());
    for item in source {
        let converted = handler(item);
        result.push(converted);
    }
    result
}

#[derive(Debug)]
pub struct Person {
    pub age : u32,
    pub height : f32
}

pub fn sort_by_key_example() {
    let mut people = vec![Person{age: 24, height:1.63 },
    Person{age: 24, height:1.63 },
    Person{age: 28, height:1.75 }
    ];
    let mut count = 0;
    people.sort_by_key(|person| {
        count+=1;
        person.age
    });
     println!("{:#?}, sorted in {count} operations", people);
}

