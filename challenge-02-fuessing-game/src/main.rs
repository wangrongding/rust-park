use rand::Rng; // Rng 是一个 trait，用于定义随机数生成器应实现的方法，它定义了 random_range 方法，用于生成指定范围内的随机数
use std::cmp::Ordering; // cmp 是 compare 的缩写，Ordering 是一个枚举
use std::io; // prelude，io 是 input/output 的缩写，它定义了标准输入输出流的功能

// 猜字游戏
fn main() {
    // 生成一个 1-100 的随机数，（1..101 创建了一个从 1 到 100 的范围,包含 1，但不包含 101）
    let secret_number = rand::thread_rng().gen_range(1..101);
    // {} 是占位符，用于将 secret_number 变量的值插入到字符串中
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Guess the number!");
        // 创建一个可变的字符串变量
        let mut guess = String::new();
        // 从标准输入流中读取用户输入的内容
        io::stdin()
            // 读取一行，将其存入一个字符串变量中, 其中 & 是取地址符号，用于将 guess 变量的地址传递给 read_line 函数（这是一个引用）
            .read_line(&mut guess)
            // expect 函数用于处理 io::Result 类型的返回值，如果 read_line 函数返回了一个 Err 值，expect 函数会导致程序崩溃，并显示传递给它的错误信息
            .expect("Failed to read line");

        // match 表达式由分支构成，每个分支包含一个模式和一些代码，这些分支被依次检查，直到模式匹配成功，对应的代码被执行
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            // cmp 方法用于比较两个值并返回一个 Ordering 类型的值，这个类型有三个值：Less、Greater 和 Equal
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // 退出循环
                break;
            }
        }
    }
}
