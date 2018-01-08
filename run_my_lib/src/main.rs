extern crate myutil;

fn main() {
    let man = myutil::install("jim", 32);
    man.say();
}
