fn main() {
    println!("Hello, world!");
    //如果匹配只有一个或者两个的话那么使用 iflet比较合适
    //回顾Option
    let some_u8 = Some(2);
    match some_u8 {
        Some(2) => match_nu(some_u8),
        Some(_) => None,
        None => None,
    };
    let a = Coin::A;
    println!("{:?}", a);
    //原始match
    match a {
        Coin::A => print!("coin a  vlaue {:?}", a),
        Coin::B => {
            print!("coin b  vlaue ")
        }
        Coin::C => print!("coin c  vlaue "),
        _ => print!("nothing"),
    }
    let mut coin = Coin::D(President::maozedong);
    //match两个
    let mut count = 0;
    match coin {
        Coin::D(President) => println!("the pic  is {:?}", President),
        _ => count += 1,
    }
    //if let实现
    coin = Coin::D(President::xijinping);
    if let Coin::D(President) = coin {
        println!("the pic  is{:?}", President);
    } else {
        count += 1;
    }
}

fn match_nu(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => {
            println!("{:?}", Some(x + 1));
            Some(x + 1)
        }
        None => None,
    }
}

#[derive(Debug)]
enum Coin {
    A,
    B,
    C,
    D(President),
}

#[derive(Debug)]
enum President {
    maozedong,
    dengxiaoping,
    jiangzemin,
    hujingtao,
    xijinping,
}