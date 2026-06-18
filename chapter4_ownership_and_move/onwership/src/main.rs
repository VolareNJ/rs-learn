fn print_padovan()
{
    let mut padovan = vec![1,1,1]; //分配
    for i in 3..10
    {
        let next = padovan[i-3] + padovan[i-2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
} //丢弃

#[test]
fn test_box()
{
    let point = Box::new((0.625, 0.5)); //分配
    let label = format!("{:?}", point); //分配
    assert_eq!(label, "(0.625, 0.5)");
}//丢弃

fn main()
{
    print_padovan();
}
