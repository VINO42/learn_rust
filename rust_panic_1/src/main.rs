use std::fs::File;

fn main() {
    println!("Hello, world!");
    //失败时 panic 的简写：unwrap 和 expect
//    let f1 = File::open("hellorust.txt").unwrap();
    //expect  可以指定自己的异常信息补充标记 将会更容易找到代码中的错误信息来自何处
    let f2 = File::open("hellorust.txt").expect("文件不存在!!!!");
}
