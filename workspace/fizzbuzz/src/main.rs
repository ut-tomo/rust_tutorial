/*
FizzBuzz問題 in Rust
3の倍数 → "Fizz"
5の倍数 → "Buzz"
3の倍数かつ5の倍数 → "FizzBuzz"
それ以外 → 数字そのまま

1からスタートで, FizzBuzz関数が終端の値を受ける
(1) while と if
(2) for と match
(3) matchのパターンでタプル使用
*/

fn fizzbuzz1(end: i32) -> () {
    let mut t: i32 = 1;
    while t <= end{
        if t%3 == 0 && t%5 == 0{
            println!("FizzBuzz")
        }
        else if t%3 == 0 {
            println!("Fizz")
        }
        else if t%5 == 0{
            println!("Buzz")
        }
        else{
            println!{"{}", t}
        }
        t += 1
    }
}

fn fizzbuzz2(end: i32) -> (){
    for i in 1..=end{
        match i % 15 {
            0 => println!("FizzBuzz"),
            3 | 6 | 9 | 12 => println!("Fizz"),
            5 | 10 => println!("Buzz"),
            _ => println!("{}", i)
        }
    }
}

fn fizzbuzz3(end: i32) -> () {
    for i in 1..=end{
        match (i%3, i%5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"), 
            (_, 0) => println!("Buzz"),
            _ => println!("{}", i)
        }
    }
}
fn main(){
    println!("------------------(1)------------------");
    fizzbuzz1(30);
    println!("------------------(2)------------------");
    fizzbuzz2(30);
    println!("------------------(3)------------------");
    fizzbuzz3(30);
}