/*
 * @author jon
 * @date 2021年08月28日
 */


pub fn variable001() {
    // mut  允许变量修改值。否则 变量不能重复赋值
    let mut x = 5;
    println!("the value of x is =====> :{}", x);
    x = 13;
    println!("the value of x is =====> :{}", x);


    //重复使用变量定义
    let i = 5;
    let i = i + 1;
    let i = i * 2;
    println!("The value of i is: {}", i);


}