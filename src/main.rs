use core::slice;
use std::{any::type_name, array};

//example function
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

//match , switch case
fn calculate(x: i32, y: i32, opera: char) -> i32 {
    match opera {
        '+' => x + y,
        '-' => x - y,
        '*' => x * y,
        '/' => x / y,
        _ => panic!("Unsupport operator"),
    }
}

fn comparecase(x: i32, y: i32) -> String {
    //if else case
    if x > y {
        format!("{} > {}", x, y)
    } else {
        format!("{} > {}", y, x)
    }
}

fn sort_nubmer(mut arr: Vec<i32>) -> Vec<i32> {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] < arr[j + 1] {
                println!(
                    "Index:{} = {} < Index:{} = {}",
                    j,
                    arr[j],
                    j + 1,
                    arr[j + 1]
                );
                arr.swap(j, j + 1);
            }
        }
    }

    arr
}

fn main() {
    //example datatype & variable

    // let name = "tdev";
    // println!("{}", name);

    // let number = 10;
    // println!("Nuber : {}", number);

    // let z: bool = true;
    // let c = 'a';
    // println!("z = {}, c = {}", z, c);
    // println!(" type x  = {}", type_of(&c));

    // println!("Calculator 1 + 2 = {}", calculate(1, 2, '+'));
    // println!("Compare : {}", comparecase(40, 45));
    let number = [1,3,4,2,5,7,9,10,8];
    let vecnumber = number.to_vec();
    println!("Sort Number: {:?}", sort_nubmer(vecnumber));
}
