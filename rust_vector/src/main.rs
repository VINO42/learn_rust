fn main() {
    println!("Hello, world!");
//vector
    //声明 空的vector
    let v: Vec<i32> = Vec::new();
    //声明有元素的vector
    let mut v1 = vec![1, 2, 3, 4];
    println!("{:?}", v);
    println!("{:?}", v1);
    //打印指定位置元素
    //这里不borrow可以拿到
    println!("{}", &v1[0]);
    //push 操作
    v1.push(14);

    println!("{:?}", v1);
    //pop
    v1.pop();
    println!("{:?}", v1);
    //遍历 vector 这里 in borrow
    for v_value in &v1 {
        println!("{}", v_value);
    }
    //borrow 不borrow那么 v1的所有权已经收回
    let zero: &i32 = &v1[0];
    println!("{}", zero);
    //使用get方法得到指定索引元素
    let zero_option: Option<&i32> = v1.get(0);
    //角标越界
    //panicked at 'index out of bounds: the len is 4 but the index is 1000'
    //let array_out_of_bound= &v1[1000];
    //如果使用 get 取某个位置元素 角标越界的话 返回的 是None其实是个option的默认值
    let does_not_exsit = &v1.get(10000);
    println!("{:?}", does_not_exsit);
    //vec mutable
    //会报错 因为 v_m.push后 borrow的[0]已经发生变化
//    let mut v_m = vec![123, 456, 789];
//    let v_m_0 = &v_m[0];
//    v_m.push(9527);
    //解决办法 v_m_0 mutable 或者给 v_m_0一个scope
    let mut v_m = vec![123, 456, 789];
    println!("v_m_before={:?}", v_m);
    {
        let v_m_0 = &v_m[0];
        println!("v_m_0={:?}", v_m_0);
    }
    v_m.push(9527);
    println!("v_m_after={:?}", v_m);
    //在子与中进行更改vec
    let mut v_scope = vec![9, 0, 2, 3, 5];
    println!("before_scope={:?}", v_scope);
    push_vec(&mut v_scope);
    println!("after_scope={:?}", v_scope);
    //使用vec存储不同的数据类型
    let v_dif_data_type = vec![
        special_data_type::Int(45),
        special_data_type::Float(22.3f32),
        special_data_type::Text(String::from("String value"))
    ];
    //遍历复杂类型
    for element in v_dif_data_type {
        println!("{:?}",element);
    }
}

fn push_vec(x: &mut Vec<i32>) {
    x.push(12);
}
#[derive(Debug)]
enum special_data_type {
    Int(i32),
    Float(f32),
    Text(String),
}
//创建一个vec 进行遍历 然后 match