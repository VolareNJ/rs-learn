use std::rc::Rc;

fn g(x: Vec<i32>)
{
    println!("function run");
}

fn main()
{
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s; // 移动语义。相当于 auto t = std::move(s);
    //let u = s; //err
    let u = t.clone(); //移动构造

    let x = vec![10,20,30];
    for i in 0..3
    {
        // g(x); //err，因为传参传的是右值引用，触发移动语义，原先x不再拥有所有权
        g(x.clone());
    }

    let mut v = Vec::new();
    for i in 101..106
    {
        v.push(i.to_string());
    }
    //let third = v[2]; //err: 不能触发移动语义将vec里的东西移动到外面
    
    for mut s in t //消耗vec中的所有元素
    {
        s.push('!');
        println!("{}",s);
    }

    //copy类型：不调用移动构造，直接调用拷贝构造
    let num1 = 36;
    let num2 = num1;
    println!("{}",num1);

    let rc1 = Rc::new("abc".to_string());
    let rc2 = rc1.clone();
    let rc3 = rc1.clone();
    //共享但不可变
}
