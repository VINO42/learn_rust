//别名
type NanoSencond = u64;
type Inch = u64;

#[allow(non_camel_case_types)]
type unit64_t = u64;
fn main() {
    println!("Hello, world!");
    let a: i32 = 32;
    let b: i32 = add_num(a);
    println!("{:?}", b);

    let mut vec = Vec::new();
    vec.push(a);
    print!("{:?}", vec[0]);
    //别名
    // let  nanoseconds :NanoSencond =5 as unit64_t;
    // let inches:Inch = 2 as  unit64_t;
    // print!("{}nanoseconds+{}inches={}unit?",nanoseconds,inches,nanoseconds+inches);

    // b=b-1;
    // print!("相减得到{:?}",b);
//---------------------------------------函数----------------------------
let  po = Point{x:32};

}
//发散函数 ! 作为任何类型进行返回
fn  diverages() ->! {
    panic!("this function never returns");
}
//普通函数
fn add_one(x: i32) ->i32 {
    x+1
}
//普通函数
fn add_num(x: i32) -> i32 {
    x + 1
}
//匿名函数
fn add_two(x:i32)->i32{x+2}

#[derive(Debug)]
struct Point {
   x: i32
}

// fn substract_num(x: i32,y:i32) ->i32 {
//      x-y
// }
