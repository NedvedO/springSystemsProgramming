fn task1() {

    let operation = |a: i32, b: i32| a * b;

    println!("--- Task 1 ---");

    println!("Result: {}", operation(10, 5));

}

 

fn task2() {

    let mut tracker = 0;

 

    let mut update = || {

        tracker += 1;

        println!("Tracker: {}", tracker);

    };

 

    println!("--- Task 2 ---");

    update();

    update();

}

 

fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>

where

    F: Fn(i32) -> i32,

{

    vec.into_iter().map(f).collect()

}

 

fn task3() {

    let numbers = vec![1, 2, 3];

 

    let doubled = process_vector(numbers.clone(), |x| x * 2);

    let replaced = process_vector(numbers, |x| if x > 2 { 0 } else { x });

 

    println!("--- Task 3 ---");

    println!("Doubled: {:?}", doubled);

    println!("Replaced: {:?}", replaced);

}

 

use std::{thread, time::Duration};

 

struct ComputeCache<T>

where

    T: Fn() -> String,

{

    computation: T,

    value: Option<String>,

}

 

impl<T> ComputeCache<T>

where

    T: Fn() -> String,

{

    fn new(computation: T) -> Self {

        Self {

            computation,

            value: None,

        }

    }

 

    fn get_result(&mut self) -> String {

        match &self.value {

            Some(v) => {

                println!("Retrieved from cache instantly!");

                v.clone()

            }

            None => {

                let result = (self.computation)();

                self.value = Some(result.clone());

                result

            }

        }

    }

}

 

fn task5() {

    println!("--- Task 5 ---");

 

    let mut cache = ComputeCache::new(|| {

        println!("Computing...");

        thread::sleep(Duration::from_secs(2));

        "Hello, world!".to_string()

    });

 

    println!("First call:");

    println!("Result: {}", cache.get_result());

 

    println!("\nSecond call:");

    println!("Result: {}", cache.get_result());

}

 

fn main() {

    task1();

    task2();

    task3();

    task5();

}