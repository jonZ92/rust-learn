/*
 * @author jon 2021:09:01
 */


pub enum IpAddrKind {
    V4,
    V6,
}

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}


pub fn _query_value(coin: Coin){

    if let Coin::Quarter=coin{

        println!("State quarter from ");
    }else {
        println!("hello !");
    }

}
