fn main() {
    //Rust 使用数组存储相同类型的数据集。 [T; N] 表示一个拥有 T 类型，N 个元素的数组。数组的大小是固
   //定。 下面定义了一个长为3 类型为i32的数组
    let mut arr:[i32;3]=[0;3];
    arr [1]=1;
    arr [2]=2;
    assert_eq!([1,2],&arr[1..]);
    println!("Hello, world!");
    for x in &arr {
        println!("{}",x);
    }
    //动态数组
    //动态数组是一种基于堆内存申请的连续动态数据类型，拥有 O(1) 时间复杂度的索引、压入（push）、弹出
//（pop)。
//创建空Vec
}
