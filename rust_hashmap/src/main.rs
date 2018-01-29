use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut m1 = HashMap::new();
    //another way to build a map
    // first  build key vec
    let key_vec = vec![String::from("blue"), String::from("red")];
    //second build value vec
    let val_vec = vec![12, 23];
    //then together
    // means like that key_vec`s iterator zip val_vec's iterator and transfer to map;
    // we do not know whats in the m2 ,so  the generic is _, means  everything?
    let mut m2: HashMap<_, _> = key_vec.iter().zip(val_vec.iter()).collect();
    //for each m2
    for (k, v) in &m2 {
        println!("{}:{}", k, v);
    }

    //set value to map;
    m1.insert(String::from("blue"), 10);
    m1.insert(String::from("yellow"), 24);
    println!("{:?}", m1);
    //get value from map
    let key1 = String::from("blue");
    let val1 = m1.get(&key1);
    println!("{:?}", &val1);
    //loop map
    for (key, value) in &m1 {
        println!("{}:{}", key, value);
    }
    //update map use or_insert()
    //ignore exist
    let mut m3 = HashMap::new();
    m3.insert(String::from("green"), 22);
    m3.entry(String::from("red")).or_insert(44);
    //update exist key
    m3.entry(String::from("red")).or_insert(67);
    println!("{:?}", m3);
    //base on an old key  to  update a  new value
    //通过哈希 map 储存单词和计数来统计出现次数
    let text = "hello world wonderful world";
    let mut text_map = HashMap::new();
    for word in text.split_whitespace() {
        let count = text_map.entry(word).or_insert(0);
        //*count  means the value point in map;
        *count += 1;
    }
    println!("{:?}", text_map);

    //给定一系列数字，使用 vector 并返回这个列表的平均数（mean, average）、中位数（排列数组后位于中间的值）和众数（mode，出现次数最多的值；这里哈希函数会很有帮助）。
    //将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，所以 “first” 会变成 “irst-fay”。元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。牢记 UTF-8 编码！
    //使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。接着让用户获取一个部门的所有员工的列表，或者公司每个部门的所有员工按照字母顺排序的列表。
}

