/*
 * @author jon 2021:08:29
 */


// 切片
pub mod slice_ {
    pub fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();// 字符转byte 数组
        for (i, &item) in bytes.iter().enumerate() { //迭代器
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
}