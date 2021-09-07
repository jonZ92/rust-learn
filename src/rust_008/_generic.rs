/*
 * @author jon 2021:09:07
 */


pub mod generic {
    //排序 。T:PartialOrd+Copy 泛型什么鬼？
    pub fn _max<T:PartialOrd+Copy>(array: &[T]) -> T {
        let mut max_index = 0;
        let mut i = 1;
        while i < array.len() {
            if array[i] > array[max_index] {
                max_index = i;
            }
            i += 1;
        }
        array[max_index]
    }
}