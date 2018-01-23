fn main() {
    println!("Hello, world!");
    //Rctangle demo
    let rc = Rctangle { length: 10, width: 5 };
    //打印struct 使用:?
    println!("rc 的 area 是{:?}", rc);
    let area = rc.area();
    //impl
    let jim = Person {
        name: String::from("jim"),
        age: 18,
    };
    jim.say();
    let lilei = Person::init(String::from("lilei"), 20);
    lilei.say();
    //多参数函数
    let rect1 = Rctangle { length: 50, width: 30 };
    let rect2 = Rctangle { length: 40, width: 10 };
    let rect3 = Rctangle { length: 45, width: 60 };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

//定义结构体 相对于元组 结构体定义数据结构更为清晰
struct Person {
    name: String,
    age: i32,
}

//关联函数
//:: 语法来调用这个关联函数
impl Person {
    //类似构造器的关联函数
    fn init(name: String, age: i32) -> Person {
        Person { name, age }
    }
    //self 直接.函数就可以调用 ???
    fn say(&self) {
        println!("hello my name is {},Im {} old", &self.name, &self.age);
    }
}

//Rctangle` cannot be formatted using `:?`; if it is defined in your crate, add `#[derive(Debug)]` or manually implement it
//the trait `std::fmt::Debug` is not implemented for `Rctangle`
#[derive(Debug)]
struct Rctangle {
    length: i32,
    width: i32,
}

impl Rctangle {
    fn area(&self) -> i32 {
        //不带分号表示返回
        &self.length * &self.width
    }
    //如果是自身引用用self 如果其他的引用 other
    fn can_hold(&self, other: &Rctangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}
//元组结构体
struct Color(i32, i32, i32);
//没有任何字段的类单元结构体
struct tree();
//