fn main()
{
}

#[test]
fn test_ref()
{
    let x = 10;
    let r = &x;
    assert!(*r == 10);
}

#[test]
fn test_mut_ref()
{
    let mut y = 32;
    let m = &mut y;
    *m += 32;
    assert_eq!(*m, 64);
}

struct Anime { name: &'static str, bechdel_pass: bool }

#[test]
fn underlying_deref()
{
    let aria = Anime {name: "Aria: The Animation", bechdel_pass: true};
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");
    assert_eq!((*anime_ref).name, "Aria: The Animation"); // 等价
}

#[test]
fn comp_ref()
{
    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    assert!(rrx <= rry);
    assert!(rrx == rry);
}

fn factorial(n: usize) -> usize
{
    return (1..n+1).product();
}

#[test]
fn right_value_ref()
{
    let r = &factorial(6); //使用一个匿名变量保存值，再将匿名变量地址初始化给r
    assert_eq!(r + &1009, 1729); //可以对右值取地址，然后在计算时隐式解引用
}
