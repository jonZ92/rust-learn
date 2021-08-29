/*
 * @author jon
 * @date 2021年08月28日
 */


// #[warn(unused_imports)]
// use rust_learn::rust_001;

// #[warn(unused_imports)]
// use rust_learn::rust_002;

// #[warn(unused_imports)]
// use rust_learn::rust_003;

#[warn(unused_imports)]
use rust_learn::rust_004;


fn main() {
    //猜字游戏
    //rust_001::puess::guess_number();
    //变量学习
    //rust_001::variable::variable001();
    //数据类型
    //rust_001::data_type::data_type();
    //函数
    //let y:i32=rust_002::funct::to_func(5);
    //println!("y :{}",y);

    //所有权
    //rust_003::ownership::ownership1::test()

    //引用 堆内存
    // let _st1 = String::from("hello jon");
    // let _len_ = rust_003::ownership::ownership2::test(&_st1);
    // println!("The value of :{}", _st1);//如果 不使用引用,_st1 所有权不存在了
    // println!("The length of :{}", _len_);

    // //引用 栈内存
    // let _num1 = 515;
    // let _len1_ =rust_003::ownership::ownership2::test1(_num1);
    // println!("The value of :{}", _num1);
    // println!("The length of :{}", _len1_);//栈上不存在,正常运行


    //引用 栈内存
    // let _num2 = "hello";
    // let _len2_ =rust_003::ownership::ownership2::test2(_num2);
    // println!("The value of :{}", _num2);
    // println!("The length of :{}", _len2_);//可以正常运行


    //字符迭代题目
    // let  _sd=String::from("hello world");
    // let word=rust_003::slice::slice_::first_word(&_sd);
    // //_sd.clear(); //清空
    // println!("word  :{}",word);
    // //rust 切片
    // let _hello=&_sd[..5];
    // let _world=&_sd[6..];
    // println!("_hello  :{}",_hello);
    // println!("_world  :{}",_world);

    //结构体
    // rust_004::_struct::_struct::struct_func();
    // let _email = String::from("237889073@qq.com");
    // let _user_name = String::from("237889073");
    // let _u1 = rust_004::_struct::_struct::_get_struct_func(_email, _user_name);
    // println!("-----------------------------------");
    // println!("email:{}", _u1.email);
    // println!("user_name:{}", _u1.user_name);
    // println!("active:{}", _u1.active);
    // println!("sign_in_count:{}", _u1.sign_in_count);
}























