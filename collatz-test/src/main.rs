use std::io;
use rand::Rng;

fn collatz(number:u32, count:u32){
    println!("==At rount {}, number {} ==",count, number);
    if number == 2 {
        println!("*** END: at rount {}, number 1 ==", count + 1);
    } else if number%2 == 0 {
        //even: n/2
        collatz(number/2, count + 1)
    }
    else{
        //odd: 3n+1
        collatz(3*number + 1, count + 1)
    };
}

fn read_input_int() -> u32 {
    let out:u32;
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).expect("Error input?");
    if ! input_str.trim().is_empty() {
        let input_num = input_str.trim().parse().unwrap_or(0);
        if (input_num > 3) && (input_num <= 99999 ){
            out = input_num;}
        else {
            out = rand::thread_rng().gen_range(3..1000);
            println!("Invalid input, try with random number {}", out);
        }
    }else{
        out = rand::thread_rng().gen_range(3..1000);
        println!("Invalid input, try with random number {}", out);
    }
    println!("Input: {}", out);
    out
}

fn main() {
    println!("Please input a nature number (3 ~ 99999), or keep blank for random");
    collatz(read_input_int() , 0);
}
