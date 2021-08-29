/*
 * @author jon
 * @date 2021年08月28日
 */

pub fn data_type(){

    let guess:u32="12312".parse().expect("Not a number!");
    println!("  guess is :{}",guess);


    let i:&str="sad";
    println!("  cahr is :{}",i);
    //元组
    let tup:(i32,f64,u64)=(5000,3.1415,1000000);
    let (_x,_y,_z)=tup;
    println!("tup is :{}",_x);

    //数组
    let _a:[i32;5]=[1,2,3,4,5];
    
}