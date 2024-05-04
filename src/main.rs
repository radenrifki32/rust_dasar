use core::str;

fn main() {
    println!("Hello, world!");
}
#[test]
fn hello_test(){
 println!("Hello, Test!")   
}

#[test]
fn test_variable(){
    let name: &str = "Raden Muhamad Rifki";
 println!("Hello {}",name)   
}
#[test]
fn test_mutable(){
    let mut name: &str = "Raden Muhamad Rifki";
    println!("Hello{}",name);
    name = "Budi";
    println!("Hello{}",name);
}
#[test]
fn static_typing(){
    let name: &str = "Raden Muhamad Rifki";
    println!("Hello {}",name);
    // name = 10;
    println!("Hello {}",name);
}

#[test]
fn shadowing(){
    let  name: &str = "Raden Muhamad Rifki";
    println!("Hello{}",name);
    let name = 10;
    println!("Hello{}",name);
}

/*
INI KOMENTAR DARI SATU BARIS
INI KOMENTAR DARI SATU BARIS
INI KOMENTAR DARI SATU BARIS
*/
#[test]
fn comment(){
    //ini Comentar
    println!("Hello"); // ini komentar
}
#[test]
fn explicit(){
    let age: i32 = 32;
    println!("{}",age);
}
#[test]
fn number(){
    let a: i32 = 10;
    println!("{}",a);
    let b: f32 = 10.5;
    println!("{}",b)
}
#[test]
fn number_conversion(){
    let a: i8 = 10;
    println!("{}",a);
    let b: i16 = a as i16;
    println!("{}",b);
    let c: i32 = a as i32;
    println!("{}",c);
    let d: i64 = 100000000;
    let e : i8 = d as i8;
    println!("{}",e);
}
#[test]
fn numeric_operator(){
    let a: i32 = 10;
    let b: i32 = 10;
    let c = a + b;
    println!("{}",c);

}
#[test]
fn boolean(){
    let a: bool = true;
    let b: bool = false;
    println!("{} {}",a,b);

}
#[test]
fn tuple () {
    let mut data : (i32, f64, bool) = (10 ,10.5, true);
    println!("{:?}",data);
    println!("{}", data.0);
    //destructuring tuple
    let (a, b, c) = data;
    println!("{} {} {}", a, b, c);
    data.0 = 20;
    data.1 = 11.5;
    data.2 = false;
    println!("{:?}", data);

}

fn unit () {
    println!("Hello");
}
#[test]
fn test () {
    let result: () = unit();
    println!("{:?}", result);
}
#[test]
fn array() {
    let mut array : [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);
    array[0] = 10;
    let length: usize = array.len();
    println!("{}", length); 
}

#[test]
fn array_dua_dimensi() {
   let matrix : [[i32;2];2] = [
    [1,2],
    [3,5]
   ];
   println!("{}", matrix[0][1]);
}

const MINIMUM: i32 = 14;
#[test]
fn constant () {
    const MAXIMUM: i32 = 13;
    print!("{} {}",MAXIMUM,MINIMUM);
}

#[test]
fn stack_heap () {
    function_a();
    function_b();
}
fn function_a(){
    let a = 10;
    let b = String::from("Raden Muhamad");
    println!("{} {}", a, b);
}

fn function_b() {
    let a = 10;
    let b = String::from("Rifki");
    println!("{} {}", a, b);

}

#[test]
fn string_type() {
    let mut  name: String = String::from("Raden Muhamaad");
    println!("{}", name);
    name.push_str(" Rifki");
    println!("{}", name);
    let new_name = name.replace("Rifki", "ganteng");
    println!("{}", name);
    println!("{}", new_name);

}

#[test]
fn data_copy() {
    let a = 10;
    let mut b: i32 = a;
    b = 30;
    println!("{} {}", a, b);
    //ini hanyaa copy owner,jadi kalau di simpan di stack dia membuat value baru
}

#[test]
fn ownership_movement() {
    let name1:String = String::from("value");
    println!("{}", name1);
    let name2:String = name1;
    println!("{}", name2);
    // println!("{}", name1); ini udh pindahh ownershipnya karna di heap disimpannya dan akan di hapus jika sudah keluar dari scope
}
#[test]
fn clone() {
    let name: String = String::from("value");
    let name2  = name.clone();
    println!("{} {}", name, name2);
}
#[test]
fn if_expression() {
    let value = 7;
    let result: &str = if value >= 8 {
        "good"
    } else if value >= 6 {
     "Not Bad" 
    } else {
    "Very Bad" 
    };
    println!("{}", result);
}
#[test]
fn loop_expression() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }
        println!("{}", counter)
    }
}
#[test]
fn loop_return_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }

    };
    println!("{}", result);
}
#[test]
fn loop_label() {
    let mut number = 1;
    'outer: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer;
            }
            println!("{} x {} = {}",number,i,number * i);
            i += 1;
            if i > 10 {
                break;
            }
        }
        number += 1;
    }
}
#[test]
fn while_loop() {
    let mut counter = 0;
    while counter <= 10 {
        if counter % 2 == 0 {
            println!("{}", counter);
        }
        counter += 1;
    }
}
#[test]
fn array_iteration() {
    let array : [&str; 5] = ["A","B","C","D","E"];
    let mut index = 0;
    while index < array.len() {
        println!("{}" , array[index]);
        index += 1;
    }
}
#[test]
fn array_iteration_for_loop() {
    let array : [&str; 5] = ["A","B","C","D","E"];
   for value in array {
    println!("VALUE : {}", value);
   }
}

#[test]
fn range() {
    let range = 0..5;
    println!("Start :{}" ,range.start);
    println!("End :{}" ,range.end);
    let array : [&str; 5] = ["A","B","C","D","E"];

    for i in range {
        println!("{}" , array[i]);
    }
}
fn say_hello () {
    println!("Say Hello");
}

#[test]
fn test_say_hello() {
    say_hello();
}

fn say_goodbye (firstname : &str, lastname :&str ) {
    println!("Goodbye {} {}" , firstname, lastname);
}
#[test]
fn test_say_goodbye() {
    say_goodbye("Raden", "Muhamad");
    say_goodbye("Joko", "Widodo");

}

fn factorial_loop(n : i32) -> i32 {
    if n < 1 {
        return 0;
    }
    let mut result = 1;
    for i in 1..= n {
        result += i;
    }
    result
}
#[test]
fn test_factorial_loop() {
    let result :i32 = factorial_loop(5);
    println!("{}",result);
}
fn print_text (value:String, times: u32) {
    if times == 0 {
        return;
    } else {
        println!("{}", value);
    }
    print_text(value, times - 1);

}
#[test]
fn test_print_text() {
    print_text(String::from("Rifki"), 10);
}

fn  factorial_recursive(n : u32) -> u32 {
    if n <= 1 {
        return 1;
    }
    n * factorial_recursive(n -1 )
}
#[test]
fn test_factorial_recursive() {
    let result = factorial_recursive(5);
    println!("{}", result);
}

fn print_number (number : i32) {
    println!("Number {}" ,number);
}
fn hi (name : String) {
    println!("Name{}", name);
}

#[test]
fn test_hi() {
    let number = 10;
    print_number(number);
    println!("{}", number);
    let name = String::from("Rifki");
    hi(name);
    // println!("{}", name);
}

fn full_name (firstname : &String, lastname :&String) -> String {
      format!("{} {}", firstname, lastname)
}
#[test]
fn test_full_name() {
    let firstname = String::from("Raden");
    let lastname = String::from("Rifki");
    let fullname = full_name(&firstname, &lastname);
    println!("{}", fullname); // ini bisa karna ownership nya sudah pindah kepada yang memanggil functionnya
    println!("{}", firstname);
    println!("{}", lastname);

}

fn change_value (value:&mut String) {
    value.push_str("string");
}
#[test]
fn test_change_value() {
    let mut value = String::from("value");
    change_value(&mut value);
    println!("{}",value);
}