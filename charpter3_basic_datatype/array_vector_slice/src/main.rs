fn print2(n: &Vec<f64>)
{
    for ele in n
    {
        println!("{}",ele);
    }
}

fn print1(n: &[f64])
{
    for ele in n
    {
        println!("{}",ele);
    }
}


fn main() 
{
    /*
    let languages: Vec<String> = std::env::args().skip(1).collect();
    for l in languages
    {
        println!("{}: {}", l,
            if l.len() % 2 == 0
            { "functional" } else { "imperative" });
    }
    */

    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];
    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    print1(&a);
    print1(&v);
    print2(&v);
    print1(&v[0..2]);
}

#[test]
fn test_array()
{
    let lazy_caterer: [u32;6] = [1,2,4,7,11,16];
    let taxonomy = ["Animalia","Arthropoda","Insecta"];
    assert_eq!(lazy_caterer[3],7);
    assert_eq!(taxonomy.len(),3);

    let mut sieve = [true;10000];
    for i in 2..100
    {
        if sieve[i]
        {
            let mut j = i * i;
            while j < 10000
            {
                sieve[j] = false;
                j += i;
            }
        }
    }
    assert!(sieve[211]);
    assert!(!sieve[9876]);

    let mut chaos = [3,5,4,1,2];
    chaos.sort(); // sort为切片方法，但是array在调用时，隐式转换为了切片
    assert_eq!(chaos, [1,2,3,4,5]);
}

#[test]
fn test_vector()
{
    let mut primes = vec![2,3,5,7];
    assert_eq!(primes.iter().product::<i32>(), 210);

    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);

    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("no");
    pal.push("pets");
    assert_eq!(pal, vec!["step","on","no","pets"]);

    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0,1,2,3,4]);

    let mut v1 = Vec::with_capacity(2);
    assert_eq!(v1.capacity(), 2);
    v1.push(1);
    v1.push(2);
    v1.push(3);
    assert!(v1.capacity() >= 3);

    let mut v2 = vec![10,20,30,40,50];
    v2.insert(3,35);
    assert_eq!(v2, [10,20,30,35,40,50]);
    v2.remove(1);
    assert_eq!(v2, [10,30,35,40,50]);

    let mut v3 = vec!["Snow Puff", "Glass Gem"];
    assert_eq!(v3.pop(), Some("Glass Gem"));
    assert_eq!(v3.pop(), Some("Snow Puff"));
    assert_eq!(v3.pop(), None);
}
