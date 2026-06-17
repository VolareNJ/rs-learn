fn const_shared_ref(a: &i32)
{
    println!("{}", a);
}

fn mutable_unique_ref(a: &mut i32)
{
    *a+=1;
    println!("{}", a);
}

fn main()
{
    let mut i = 1_i32;

    const_shared_ref(&i);
    mutable_unique_ref(&mut i);

    let t = (12,"eggs");
    let b = Box::new(t);
}
