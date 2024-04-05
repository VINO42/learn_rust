use std::mem::size_of_val;

fn main() {
    println!("Hello, world!");
}
// 修改2处 `assert_eq!` 让代码工作

fn f1() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),1); 

    let c2 = '中';
    assert_eq!(size_of_val(&c2),2); 

    println!("Success!")
} 

// 修改一行让代码正常打印
fn f2() {
    let c1 = "中";
    print_char(c1);
} 

fn print_char(c : &str) {
    println!("{}", c);
}


// 使成功打印
fn f3() {
    let _f: bool = false;

    let t = false;
    if !t {
        println!("Success!")
    }
} 


fn f4() {
    let f = false;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!")
}


fn f5() {
    let unit: () = ();
    // unit type doesn't occupy any memory space
    assert!(size_of_val(&unit) == 0);
}