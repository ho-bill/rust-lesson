use std::io;
use rand::Rng;

fn multe_check(mut number: u64, index: u32) {
    println!("\n==Number：{} in rount {} ==", number, index);
    let mut counter = 1;
    let mut result = 1;
    while number > 0 {
        let digit = number % 10;
        if digit == 0 {
            number = 0;
        } else {
            number = number / 10;
        }
        result = result * digit;
        //println!("Number #{} = {}, Result={} ",counter, digit, result);
        counter = counter + 1;
    }
    println!("Result={} ", result);
    if result > 9 {
        println!("Call next round ...");
        multe_check(result, index + 1);
    } else{
        println!("Finished at round #{} ",index);
    }
}

fn main() {
    println!("Please input a nature number.");
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).expect("Error input?");
    //Convert string to number
    let input_num: u64 = match input_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("請輸入 1 ~ 500000000 的數字!");
            0
        }
    };
    if input_num > 0 && input_num < 500000000 {
        multe_check(input_num, 1);
    } else {
        let rand_num: u64 = rand::thread_rng().gen_range(1..400000000);
        println!("Not a valid number from input, run with a random number [{}] ", rand_num);
        multe_check(rand_num, 1);
    }
}