
fn main() {
    //用枚举替代结构体还有另一个优势：每个成员可以处理不同类型和数量的数据
    println!("Hello, world!");


    let home: ip_addr = ip_addr {
        kind: IpAddrKind::v4,
        addr: String::from("127.0.0.1"),
    };
    let loopBack: ip_addr = ip_addr {
        kind: IpAddrKind::v6,
        addr: String::from("::1"),
    };
    //Options
    let some_nums: Option<i32> = Some(5);
    let some_str: Option<String> = Some(String::from("123"));
    //使用match进行匹配 实现算术运算
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six {:?}",six);
    println!("Nome {:?}",none);
    //使用::调用枚举成员
    let m = Message::Write(String::from("hello"));
    m.call();
}

struct ip_addr {
    kind: IpAddrKind,
    addr: String,
}

enum IpAddrKind {
    v4,
    v6,
}
enum IpAddrKind_enum{
    //指定枚举成员类型 tuple类型指定
    v4(u8,u8,u8,u8),
    v6(String),
}
enum IpAddrKind_struct{
    v4(IpV4Addr),
    v6(IpV6Addr),
}
struct IpV4Addr{
//定义成员
}
struct IpV6Addr{
//定义成员
}

//复杂枚举
enum Message {
    //没有关联任何数据。
    Quit,
    //包含一个匿名结构体
    Move { x: i32, y: i32 },
    //包含单独一个 String.
    Write(String),
    //包含三个 i32。
    ChangeColor(i32, i32, i32),
}
struct QuitMessage; //无成员struct
//声明成员 struct
struct MoveMessage {
    x: i32,
    y: i32,
}
//元组 struct
struct WriteMessage(String); // tuple struct
//多数据结构元组struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
//关联函数
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
//option的加法实现

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
