fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let nums: [i32; 10] = [12, 7, 9, 10, 15, 22, 3, 5, 30, 11];

    // For loop: print even/odd or Fizz/Buzz/FizzBuzz rules
    for n in nums {
        if n % 3 == 0 && n % 5 == 0 {
            println!("{n}: FizzBuzz");
        } else if n % 3 == 0 {
            println!("{n}: Fizz");
        } else if n % 5 == 0 {
            println!("{n}: Buzz");
        } else {
            if is_even(n) {
                println!("{n}: even");
            } else {
                println!("{n}: odd");
            }
        }
    }

    // While loop: sum of all numbers
    let mut i: usize = 0;
    let mut sum: i32 = 0;
    while i < nums.len() {
        sum = sum + nums[i];
        i = i + 1;
    }
    println!("Sum = {sum}");

    // Loop: find largest number
    let mut idx: usize = 0;
    let mut largest: i32 = nums[0];
    loop {
        if nums[idx] > largest {
            largest = nums[idx];
        }

        idx = idx + 1;
        if idx >= nums.len() {
            break;
        }
    }
    println!("Largest = {largest}");
}