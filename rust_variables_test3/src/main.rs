use std::str::FromStr;
use std::*;

fn main() {
    //原生类型
    //布尔型
    let mut x: bool = false;
    x = true;
    println!("/{}", x);
    //布尔类型的应用
     let flag = true;
    if flag {
        println!("/哦 yes!");
    } else {
        println!("what!?");
    }
    //bool类型用在match语句中
    match flag {
        true => println!("yes !"),
        false => println!("oh no!"),
    }
    //还可以将字符串"true" 和"false"转为bool类型
    assert_eq!(FromStr::from_str("true"), Ok(true));
    assert_eq!(FromStr::from_str("false"), Ok(false));

    assert!(<bool as FromStr>::from_str("NOT EVEN A BOOL").is_err());
    //char 类型   rust中的char类型是1个字符长
    //使用单引号进行创建字符 可以将奇怪字符赋值给char
    let x = 'x';
    let two_heart = '★';
    println!("{}", x);
    println!("{}", two_heart);
    //数字类型
    //数字类型分为 有符号整数i8i16i32i64 isize
    //无符号整数: u8u16u32u64 usize
    //浮点整数 f32 f64
    //isize 和 usize 也称为可变大小类型或自适应类型
    //其大小依赖底层机器指针大小 32位电脑上就是32位 64位电脑上就是64位
    let x = 123; //i32
    let y = 1.23; //f64
    let c = -456; //i32;
    let a = 182838823483858392;
    println!("{:?}", a);
    println!("i8的最大值为{}", i8::MAX);
    println!("i8的最小值为{}", i8::MIN);
    println!("i16的最小值为{}", i16::MIN);
    println!("i16的最大值为{}", i16::MAX);
    println!("i32的最小值为{}", i32::MIN);
    println!("i32的最大值为{}", i32::MAX);
    println!("i64的最小值为{}", i64::MIN);
    println!("i64的最大值为{}", i64::MAX);

    println!("u8的最小值为{}", u8::MIN);
    println!("u8的最大值为{}", u8::MAX);
    println!("u16的最小值为{}", u16::MIN);
    println!("u16的最大值为{}", u16::MAX);
    println!("u32的最小值为{}", u16::MIN);
    println!("u32的最大值为{}", u16::MAX);
    println!("u64的最小值为{}", u16::MIN);
    println!("u64的最大值为{}", u16::MAX);

    println!("f32的最小值为{}", f32::MIN);
    println!("f64的最大值为{}", f64::MAX);
    //数组 array
    //数组被定义为[T;size] T类型size大小的数组
    let mut arr: [i32; 3] = [0; 3];
    arr[1] = 1;
    arr[2] = 2;
    println!("{:?}", arr);
    //使用arr.len()获得数组的元素数量
    println!("{:?}", arr.len());
    let names = ["a", "jim", "cat"];
    println!("the second name is {:?}", names[1]);
    //切片
    //一个切片的表达式为[T] 或&mut[T]
    let arr_1 = [1, 2, 3, 4, 6, 5, 7];
    let slice_complete = &arr_1[..]; //获取整个数组切片
    let slice_middle = &arr_1[1..4]; //获取该数组中间元素 切片遵循闭开原则就是左包右不包
    let slice_right = &arr_1[1..]; //获取从1位置开始右边的所有数组元素
    let slice_left = &arr_1[..3]; //获取到3位置的数组元素但是不包含该位
    assert_eq!(slice_middle, &[2, 3, 4]);
    //元组 tuples
    //每个元组的类型都是(T1,T2,...)
    let x = (1, "jim", 2.02);
    let y: (i32, &str, f64) = (1, "lily", 6.03);
    //如果两个元组的结构类型数量相同 那么可以互相进行赋值
    let mut z = (1, 2);
    let y = (2, 3);
    z = y;
    //通过let 结构进行元组中的每个类型进行赋值到具体的变量上
    //来访问元组中的字段 像不像 类中的属性
    let (ass, fss, dss) = (1, 2, 3);
    println!("/{:?}", ass);
    //也可以用索引来访问元组中元素
    let tt = (12, 34, 67);
    let x = tt.0;
    let y = tt.1;
    let ccc = tt.2;
    println!("{}", x);
    println!("{}", y);
    println!("{}", ccc);
    //把元组作为参数类型传入
    //str 原始字符串类型 不定长类型 rust中所有被双引号
    //包裹的都可以被称为&str
    //函数类型
    fn foo(x: i32) -> i32 {
        x
    }
    let x: fn(i32) -> i32 = foo;
    //这里就是定义了一个类型为函数类型(i32入参反参为i32)
    //的函数指针指向了foo这个函数
    fn zii(x: i8) -> i8 {
        x
    }
    let n: fn(i8) -> i8 = zii;

}

// trait we {
//      fn name(arg: Type) -> RetType {
//          unimplemented!();
//      }
// }
