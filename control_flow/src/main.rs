fn fibonaci(n: i32) -> i32 {
    if n <= 0 {
        1
    } else {
        fibonaci(n - 1) + fibonaci(n - 2)
    }
}

#[derive(Debug)]
struct User {
    height: u32,
    age: i32,
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    length: i32,
}

fn main() {
    let mut score = 0;

    for num in (1..10).rev() {
        score = score + num;
        println!("{}", num);
        println!("{}", score);
    }
    println!("fibonacci---------------");

    for nums in (0..10) {
        println!("{}", fibonaci(nums));
    }
    //struct
    let user = User {
        height: 185,
        age: 32,
    };
    let a_rectangle = Rectangle {
        width: 12,
        length: 22,
    };
    println!("{:?}", user);
    println!("the value of rectangle is {}", cal(a_rectangle.width, a_rectangle.length));
    //method syntax
    let b_rectangle = Rectangle {
        width: 10,
        length: 10,
    };
    println!("the value of rectangle is {}", b_rectangle.area());
    println!("the value of rectangle  minus  is {}", b_rectangle.lminusw());
    println!("the value of rectangle  custom method  is {}", b_rectangle.custom(32));
    println!("can is {}", a_rectangle.can_hold(&b_rectangle));
    println!("associated functions square {:?}", Rectangle::square(4, 4));
    //枚举类型
    let four = ip_add_type::v4;
    let six = ip_add_type::v6;
    let home: ipAddr = ipAddr {
        kind: ip_add_type::v4,
        addr: String::from("127.0.0.1"),
    };
    let loopback: ipAddr = ipAddr {
        kind: ip_add_type::v6,
        addr: String::from("::1"),
    };
}

#[derive(Debug)]
struct ipAddr {
    kind: ip_add_type,
    addr: String,
}

impl ipAddr {}

enum ip_add_type {
    v4,
    v6,
}

impl Rectangle {
    //成员方法
    fn area(&self) -> i32 {
        self.width * self.length
    }
    fn lminusw(&self) -> i32 {
        self.length - self.width
    }
    fn custom(&self, x: i32) -> i32 {
        x * (self.length - self.width + 2)
    }
    fn can_hold(self, other: &Rectangle) -> bool {
        (self.width > other.width && self.length > other.length) || (self.width > other.length && self.length > other.width)
    }
    //类似 静态构造函数

    fn square(width: i32, length: i32) -> Rectangle {
        Rectangle {
            width,
            length,
        }
    }
}

//计算 面积
fn cal(width: i32, length: i32) -> i32 {
    width * length
}



