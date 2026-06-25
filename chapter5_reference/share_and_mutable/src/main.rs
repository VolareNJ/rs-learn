fn main()
{
    println!("Hello, world!");
}

/*
#[test]
fn test_dangling_ptr()
{
    let v = vec![4,8,19,27,34,10];
    let r = &v;
    let aside = v; //移动语义
                   //在共享引用的整个生命周期内，引用的目标都是只读，包括所有者。因此不能使用移动语义
    assert_eq!(r[0], 4); //err
                         //r指向的是v本身，不是v的缓冲区。虽然v的缓冲区内容还在原来的位置，但是v本身已经回到未初始化状态了
}
*/

#[test]
fn test_dangling_ptr()
{
    let v = vec![1,2,3,4,5];
    {
        let r = &v;
        assert_eq!(r[0], 1);
    }
    let aside = v;
}


fn extend(vec: &mut Vec<f64>, slice: &[f64])
{
    for elt in slice
    {
        vec.push(*elt);
    }
}

/*
#[test]
fn test_extend()
{
    let mut wave = Vec::new();
    let head = vec![0.0,1.0];
    let tail = vec![0.0,-1.0];

    extend(&mut wave, &head);
    extend(&mut wave, &tail);

    assert_eq!(wave, vec![0.0,1.0,0.0,-1.0]);

    extend(&mut wave, &wave);//err：&mut wave添加元素时，会分配新的缓冲区，使得&wave的缓冲区被释放
                             //rust会报错：可变引用和共享引用的生命周期不能重叠。从而规避此问题
    assert_eq!(wave, vec![0.0,1.0,0.0,-1.0,0.0,1.0,0.0,-1.0]);
}
*/


