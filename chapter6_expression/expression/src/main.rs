fn main()
{
    println!("Hello, world!");
}

#[test]
fn test_lazy_initialisation()
{
    let name;
    if true
    {
        name = "nick".to_string();
    }
    else
    {
        name = "juddy".to_string();
    }
    //不需要声明mut，因为这不是copy，而是初始化
    assert_eq!(name, "nick");
}

#[test]
fn test_shadowing()
{
    let line: i32 = 1;
    let line: f64 = 1.0;
}

#[test]
fn test_match()
{
    let code = 0;
    println!
        ("{}",
            match code
            {
                0=>"ok",
                1=>"ok1",
                2=>"ok2",
                _=>"no"
            }
        )
}

#[test]
fn test_if()
{
    //let a: i32 = if true {1}; //err：如果不满足，会返回()
    let a: i32 = if true {1} else {0};
}

#[test]
fn test_if_let()
{
    let tar;
    let var = true;
    if let var = false
    {
        tar = 1;
    }
    else
    {
        tar = 0
    }
    assert_eq!(tar, 1);
}

#[test]
fn test_loop()
{
    let mut a: i32 = 0;
    let ret = 'a: loop
    {
        a+=1;

        let mut b: i32 = 0;
        'b: loop //循环可以有生命周期
        {
            b += 1;
            if b == 5
            {
                break 'a "b break"; // 按生命周期break
                                    // 解决了多层循环退出必须用goto的问题
            }
            else
            {
                break 'a "b break"
            }
        }

        if a == 5
        {
            break 'a "a break"; //可以在break后面加返回值
        }
    };
    assert_eq!(ret, "b break");
}
