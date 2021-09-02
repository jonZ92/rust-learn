/*
 * @author jon 2021:09:02
 */


pub mod _vec{
    pub fn _vec(){
        let mut _v :Vec<i32>=Vec::new();
        _v.push(0);
        _v.push(1);
        _v.push(2);
        _v.push(3);
        _v.push(4);
        _v.push(5);
        _v.push(6);
        _v.push(7);
        _v.push(8);
        _v.push(9);


        let _third:&i32=&_v[1];

        match _v.get(1) {
            Some(_third)=>println!("The third element is :{}", _third),
            None => println!("There is no third element."),
        }

        //vec 集合 宏
        let mut _ve=vec![];
        _ve=_v;

        for i in &_ve{ // _ve 借给 for 循环用
            println!("{}", i);
        }

        println!("------------------");

        println!("hello  :{}", _ve[1]);

    }
}