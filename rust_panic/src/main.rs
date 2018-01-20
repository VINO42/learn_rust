use std::fs::File;
use std::io::ErrorKind;

fn main() {
    println!("Hello, world!");
    //build a panic
    //unrestore fault
//    let v = vec![1, 2, 3];
//    let c = v[1000];
    //restorable fault;

    /*let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There is no such file {:?}", error);
        }
    };*/

    //匹配不同的错误 如果文件不存在 那么创建文件 如果文件存在而在打开时候报错那么panic其他错误

    let f1 = File::open("hellorust.txt");
    let f1 = match f1 {
        //正常打开
        Ok(f1) => f1,
        //如果不存在那么就创建 同时判断在创建时候异常情况
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hellorust.txt") {
                Ok(fc) => {
                    println!("create th file");
                    fc
                }
                Err(e) => {
                    panic!("tried to create the file failed erro is {:?}", e)
                }
            }
        }
        //如果文件存在那么 处理文件打开异常错误
        Err(e) => {
            panic!("there was other problem  in opening the file {:?}", e)
        }
    };

}
