fn main()
{
    let mut s1 = 3;
    let mut s2 = 2;

    if s1 > s2
    {
        std::mem::swap(&mut s1, &mut s2)
    }
}
