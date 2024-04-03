#![allow(unused_variables)]
#![allow(dead_code)]

//memory stack test
// fn main() {
//     let n1 = 1;
//     let n2 = 2;
// }

// fn ex1_1(){
//     let n3 = 3;
//     ex1_2();
// }

// fn ex1_2(){
//     let n4 = 4;
// }

//pointer test

// fn main() {
//     let mut n1 = 1;
//     let n2 = &mut n1;
//     println!("{}",n2);
//     *n2 = 20;

//     println!("{}",n2);
// }

// fn main() {
//     let mut n1 = 1;
//     plus_ten(&mut n1);

//     println!("{}",n1);
// }

// fn plus_ten(a: &mut i32) {
//     *a = *a + 10;
// }

//memory heap tset
//rust have auto clean heap and stack after end scope

// fn main() {
//     let n1 = Box::new(1);
//     let n2 = Box::new(2);

//     exh_1()
// }

// fn exh_1() {
//     let n3 = Box::new(3);
//     exh_2()
// }

// fn exh_2() {
//     let n4 = Box::new(4);
// }

// //Owner Ship

// fn main(){
//     let name = String::from("tdev");

//     say_hello(name);

//     //cannot use name ownership change to name param in func say_hello

//     // println!(name);

// }

// fn say_hello(name :String){
//     println!("name: {}",name);
// }

//Ref and Heap

// fn main() {
//     let name = String::from("TDEV");

//     say_hello(&name);
//     //use ref ownership not change name
//     println!("name : {}", name);
// }

// fn say_hello(name: &String) {
//     println!("Name: {}", *name)
// }
