use std::ops::Mul;
use std::cmp::PartialOrd;
fn main() {
    //在函数中使用泛型
    println!("Hello, world!");
    let i32_arr = vec![1, 3, 5, 72, 1, 324, 66, 778];
    let largest = find_largest(&i32_arr);
    println!("the largest in arr is {}", largest);
    let largest = generic_largest(&i32_arr);
    println!("the largest in generic is {}", largest);
    //结构体中的泛型
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.get_seuqare());
}

//寻找i32数组中的最大值
fn find_largest(list: &[i32]) -> i32 {
    let mut largest: i32 = list[0];
    for &element in list {
        if (element > largest) {
            largest = element;
        }
    }
    largest
}

//error
//   = note: an implementation of `std::cmp::PartialOrd` might be missing for `&T`
 fn generic_largest<T:PartialOrd+Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &li in list {
        if (li > largest) {
            largest = li;
        }
    }
    largest
}

//枚举中的泛型
enum Country<T, E> {
    America(T),
    China(E),
}

//结构体中的泛型
struct Point<T, E> {
    x: T,
    y: E,
}

impl<T, E> Point<T, E> {
    fn get_seuqare(&self) -> &T {
        &self.x //* &self.y
    }
}