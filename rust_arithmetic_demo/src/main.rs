use std::ops::{Add,Sub};
fn main() {
 /**chapter6运算符*/
 //一元运算符
 //-:取负值 实现了std::ops::Neg
 //*:解引用 实现了std::ops::Deref 或std::ops::DerefMut
 // !:取反  如!false就是真 如果这个符号对数字类型进行使用的话
 //       那么将会把每一位都置反 
 //&和&mut: 租借,borrow,向一个owner租借其使用权,分别
            //租借一个只读使用权和 读写使用权
//二元运算符
 //+:加法实现了std::ops::Add;
 //-:减法实现了std::ops::Sub
 ///:除法实现了std::ops::Div;
 //*:乘法实现了std::ops::Mul
// %:取余实现了std::ops::Rem
//位运算符
  //&:与操作符.实现了std::ops::BitAnd
  //|:或操作符. 实现了std::ops::BitOr
  //^:异或操作符,实现了std::ops::BitXor
  //<<:左移运算符.实现了std::ops::Shl
  //>>:右移运算符.实现了std::ops::Shr
//惰性运算符 短路运算符
//&&｜｜　！
//比较运算符
//==和!= 实现的是PartialEq, </>/>=和<=实现的是PartialOrd

//类型转换运算符
    //as 是rust中的类型转换运算符
//重载运算符

//格式化字符串  核心有5个宏 2个trait
//format!("\{:?}") format_args!("\{:?}")
//print!("\{:?}"); println!("\{:?}");
//write!(, "\{:?}")
//两个trait Debug Display
//核心的宏是format!
format_demon();

   for (i, j) in (0..5).enumerate() {
    println!("i = {} and j = {}", i, j);
    }
    loop {
        println!("无聊的死循环");
    }

}
// fn avg(vals:&[f64])->f64{
//     let sum:f64=sum(vals);
//     let num :f64=len(vals) as f64;
//     sum/num
// }
fn  format_demon(){
    let s = format!("今天是{0}年{1}月{2}日,{week:?},气温{3:>0width$} ~ {4:>0width$} 摄氏度。",2017,3,19,week="Sunday",width=2);
    println!("{}",s);
}
