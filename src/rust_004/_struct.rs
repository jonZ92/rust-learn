/*
 * @author jon 2021:08:29
 */

pub mod _struct {
    //想象成 c\c++ 结构体
    pub struct User {
        pub user_name: String,
        pub email: String,
        pub sign_in_count: u64,
        pub active: bool,
    }

    pub fn struct_func() {
        let _user = User {
            email: String::from("zjon234@gmail.com"),
            user_name: String::from("zjon234"),
            active: true,
            sign_in_count: 1,
        };

        println!("email:{}", _user.email);
        println!("user_name:{}", _user.user_name);
        println!("active:{}", _user.active);
        println!("sign_in_count:{}", _user.sign_in_count);
    }

    pub fn _get_struct_func(_email: String, _user_name: String) -> User {
        User {
            email: _email,
            user_name: _user_name,
            active: false,
            sign_in_count: 1000,
        }
    }


    #[derive(Debug)]
    pub struct Rectangle {
        pub width: u32,
        pub height: u32,
    }

    // IMPL 来定义 结构体方法,&self 感觉想 this 指针
    impl Rectangle {
        pub fn area(&self) -> u32 {
            self.width * self.height
        }
        pub fn can_hold(&self, _other: &Rectangle) -> bool {
            self.width > _other.width && self.height > _other.height
        }
    }

    impl Rectangle {
        /*
         允许在 impl 块中定义 不 以 self 作为参数的函数。这被称为 关联函数,
         因为它们与结构体相关联。它们仍是函数而不是方法，因为它们并不作用于一个结构体的实例。
         */
        pub fn square(_seiz: u32) -> Rectangle {
            Rectangle { width: _seiz, height: _seiz }
        }
    }
}













