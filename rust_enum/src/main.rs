fn main() {
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
    let some_str: Option<String> = Some("123");
}

struct ip_addr {
    kind: IpAddrKind,
    addr: String,
}

enum IpAddrKind {
    v4,
    v6,
}

enum Option<T> {
    Some(T),
    None,
}