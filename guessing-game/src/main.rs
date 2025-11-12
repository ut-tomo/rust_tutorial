use rand::Rng;
use std::io;

enum Compare {
    Correct,
    Smaller,
    Larger,
}

fn read_input() -> Option<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    let n: i32 = input.trim().parse().ok()?;
    if (1..=100).contains(&n) {
        Some(n)
    } else {
        None
    }
}

fn compare_guess(input: i32, answer: i32) -> Compare {
    use Compare::*;
    if input == answer {
        Correct
    } else if input < answer {
        Smaller
    } else {
        Larger
    }
}


fn main() {
    let mut rng = rand::thread_rng();
    let target = rng.gen_range(1..=100);
    let mut t = 0;

    loop{
        t += 1;
        println!("{}回目の予想(1~100の間の数字を入力してください)", t);
        
        match read_input(){
            Some(num) => match compare_guess(num, target){
                Compare::Correct => {
                    println!("{}回目で成功!", t);
                    break;
                }
                Compare::Smaller => println!("もっと大きいです"),
                Compare::Larger => println!("もっと小さいです")
            },
            None => println!("入力が無効です。1~100の数字を入力してください")
        }
    }
}