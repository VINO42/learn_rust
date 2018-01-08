fn main() {
    println!("Hello, world!");
    f1(12, 23);
    let s = String::from("hello world");
    let z = 5;
    let g = {
        let x = 4;
        x - 1
    };
    f1(12, g);
    let mut num = if true {
        5
    } else {
        6
    };
    println!("value of nums is{}", num);

    while num > 0 {
        println!(" pring onece ");
        num=num-1;
    }
}

fn f1(param: i32, x: i32) {
    println!("this is another function the param value is {}", param);
    println!("this is another function the x value is {}", x);
}