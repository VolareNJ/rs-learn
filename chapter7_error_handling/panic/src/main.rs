fn main()
{
    println!("Hello, world!");
}

fn pirate_share(total: u64, crew_size: usize) -> u64
{
    let half = total / 2;
    half / crew_size as u64
}


#[test]
fn test_pirate_share()
{
    pirate_share(10, 0); //panic
}

/*
 *默认处理：产开调用栈
特殊情况：
1、展开时调用drop方法触发第二个panic，则强制终止
2、指定终止
*/
