use std::io;
use rand::Rng;

fn lt_eq_gt <T: PartialOrd>(input: T, ans: T) -> i32{
    // 答えと同じなら0, 答えの方が大きければ1, 答えの方が小さければ2
    if input == ans {
        println!("Correct!");
        0
    }else if input < ans{
        println!("Correct Answer is larger than your guess");
        1
    }else{
        println!("Correct Answer is smaller than your guess");
        2
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let target = rng.gen_range(1..=100);
    println!("1~100の整数を入力してください");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input error");

        let num = match input.trim().parse() {
            Ok(n) if (1..=100).contains(&n) => n,
            _ => {
                eprintln!("1~100の整数を入力してください");
                continue;
            }
        };
        if lt_eq_gt(num, target)== 0 {
            break;
        }
    }

}
