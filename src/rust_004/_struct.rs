/*
 * @author jon 2021:08:29
 */

pub mod _struct {
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
}