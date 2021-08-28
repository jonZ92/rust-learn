/*
 * @author jon
 * @date 2021年08月28日
 */
use std::io;
//输入输出 标准库
use rand::Rng;
//随机数 库
use std::cmp::Ordering;


pub fn guess_number() {
    println!("guess th number !");

    //生成随机数
    let secret_number = rand::thread_rng().gen_range(1..101);
    //println!("this secret number is :{}",secret_number);

    loop {
        println!("please input your guess");

        //定义可变变量，mut 标志可变变量
        let mut guess = String::new();


        // 标准输入函数，把输入的数字赋值给 guess，& 引用赋值
        io::stdin().read_line(&mut guess).expect(" failed to read line");

        println!("you guess :{}", guess);

        //类型转换，错误就打印信息
        //let guess: u32 = guess.trim().parse().expect("please type a number");
        //类型转换，并处理错误
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,//转换正确，赋值
            //Err(_) => continue,//错误结束本次循环，进行下个循环
            Err(_) => {// 输入错误，提示用户，并且跳出本次循环
                println!("you entered the wrong characte!");
                continue;
            }
        };
        //现在完成猜数动作
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                //才对后跳出循环
                break;
            }
        }
    }
}