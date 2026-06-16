

fn build_vector() -> Vec<i16>
{
    let mut v = Vec::new();
    v.push(10i16);
    v.push(20i16);
    return v;
}

fn main()
{
    // println!("{}", (-4).abs());
    println!("{}", (-4_i32).abs());
    println!("{}", -4_i32.abs());
    println!("{}", i32::abs(-4));

    let mut i = 1_i32;
    loop
    {
        i = i.checked_mul(10).expect("multiplication overflowed");
    }
}

#[test]
fn test_convert()
{
    assert_eq!(2_u16.pow(4), 16);
    assert_eq!((-4_i32).abs(), 4);
    assert_eq!(0b101101_u8.count_ones(), 4);
}

#[test]
fn test_check()
{
    assert_eq!(10_u8.checked_add(20), Some(30));
    assert_eq!(100_u8.checked_add(200), None);
}

#[test]
fn test_wrapping()
{
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392); // ret = 250000 % 2.pow(16); // 因为u16占16位
                                                  // 这是针对unsigned的情况
                                                  
    assert_eq!(500_i16.wrapping_mul(500), -12144); // 针对有符号，则可能回绕为负数
    
    assert_eq!(5_i16.wrapping_shl(17), 10); // 相当于 5 << 17，由于i16，就相当于 5 << 1
}

#[test]
fn test_saturating()
{
    assert_eq!(32760_i16.saturating_add(10), 32767);
    assert_eq!((-32760_i16).saturating_sub(10), -32768);
}

#[test]
fn test_overflowing()
{
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));
    assert_eq!(5_u16.overflowing_shl(17), (10, true));
}
