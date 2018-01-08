enum Coin {
    A,
    B,
    C,
    D,
}


fn main() {
    println!("Hello, world!");
    let a = value_in_coin(Coin::A);
    println!("{}", a);
}

fn value_in_coin(coin: Coin) -> i32 {
    match coin {
        Coin::A => {
            println!("coin a  vlaue 1");
            1
        }
        Coin::B => 2,
        Coin::C => 3,
        Coin::D => 4,
        _ => 5,
    }
}