/*
 * @author jon
 * @date 2021年08月28日
 */

// 函数定义
pub fn to_func(x:i32)->i32{

    //控制流语句
    let num =5;
    if num<10{
        println!("小于情况");
    }else{
        println!("大于情况");
    }

    //let 使用 if语句
    let cond:bool=true;
    //if语句返回值,三目运算符不存在
    let _number=if cond{
        5
    }else {
        6
    };


   // loop 循环语句
    loop{
        println!("跳出循环");
      if cond{
          break;
      }
    }
    let _kos= loop {
        println!("我在循环中");
        if cond{
            break "hello,跳出循环了";
        }
    };
    return x*5;
}