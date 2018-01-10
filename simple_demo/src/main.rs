fn main() {
    let a1 = 5;
    let a2:i32=5;
    assert_eq!(a1,a2 );
    let a3:u32= 6;
    println!("Hello, world!");
    println!("Hello {}",a3);
    println!("Hello {}",a2);
    println!("Hello {}",a1);
    let a4=[0,1,2,3,4];
    let t=true;
    let mut f:bool = false;
   println!("{:?}",t);
   println!("{:?}",f);
   f=true;
   println!("{:?}",f);
   //char类型
   let c='c';
   //整数型
   let x=42;
   let y:u32=123_4567;
   let z:f64=1.23e+2;
   let zero=z.abs_sub(123.4);
   let bin =0b11111_0000;
   let oct =0o7320_1546;
   let hex=0xf23a_b049;

   //字符串类型
   let str = "hello world!";
   let mut string = str.to_string();

   //数组与切片
   let arr=[0,1,2,3,4];
   let middle= &arr[1..4];
   let mut ten_zeros:[i64;10]=[0;10];

   //元组
   let tuple:(i32,&str)=(50,"hello");
   let (fifty,_)=tuple;
   let hello=tuple.1;
   // 指针
   let x =520;
   let raw =&x as *const i32;
   let points_at= unsafe{*raw};
   //函数指针
   fn foo(x:i32)->i32{x}
   let bar:fn(i32)->i32=foo;
   println!("{:?}",x);
   //Rust不提供原生类型之间的隐式转换，只能使用 as 关键字显式转换。 类型转换
   let decimal =65.3234234_2342;
   let integer=decimal as u8;
   let character= integer as char;
   //type aliases
   type NanoSecond=u64;
   type Point=(u8,u8);




}
