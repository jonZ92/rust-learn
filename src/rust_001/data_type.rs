/*
 * @author jon
 * @date 2021年08月28日
 */

pub fn data_type(){

    let guess:u32="12312".parse().expect("Not a number!");

    let i:&str="sad";
    println!("  guess is :{}",guess);
    println!("  cahr is :{}",i);
}