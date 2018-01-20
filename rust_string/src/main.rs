fn main() {
    println!("Hello, world!");
    let mut s1 = String::from("hello");
    println!("{}", s1);
    let s2 = String::from("rust");

    let s3 = s1 + &s2;

    println!("{}", s3);

    //slice get per elements

    let s1_slice_1 = &s3[0..2];
    println!("{}", s1_slice_1);

    // loop chars in string
    for c in s3.chars() {
        print!("{},", c);
    }
    println!("");
    // loop per char`s bytes
    for byte in s3.bytes() {
        print!("{}|",byte);
    }
    println!("");
    //reverse bytes to string
    //229, 177, 140
    let s4 ="屌";
//    println!("{:?}",s4.bytes());
    let sparkle_diao=vec![229,177,140];
    let sparkle_diao_str=String::from_utf8_lossy(&sparkle_diao);
    println!("{}",sparkle_diao_str);
    //To be Continue;
}
