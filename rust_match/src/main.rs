enum Coin {
    A,
    B,
    C,
    D,
}

//match 在只关心 一个 情况的场景中可能就有点啰嗦了。为此 Rust 提供了if let。
fn main() {
    println!("Hello, world!");
    let a = value_in_coin(Coin::A);
    println!("{}", a);
}

fn value_in_coin(coin: Coin) -> i32 {
    match coin {
        //不带分号 为表达式 如果有返回必须返回
        Coin::A => {
            print!("coin a  vlaue ");
            1
        }
        Coin::B => 2,
        Coin::C => 3,
        Coin::D => 4,
        _ => 5,
    }
}
//match with option
