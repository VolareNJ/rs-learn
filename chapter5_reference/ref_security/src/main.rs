fn main()
{
    println!("Hello, world!");
}

#[test]
fn borrow_local_var()
{
    let r;
    {
        let x = 1;
        //&x生命周期开始
        r = &x;
        //&x生命周期结束
    }
    //assert_eq!(*r, 1); //err
    //1. 引用的生命周期必须小于所引用对象的生命周期
    //2. 如果引用存储在一个变量中，那么那么引用类型必须在这个变量的整个生命周期内可用
}

static mut STASH: &i32 = &128;

// fn f<'a>(p: &'a i32) { unsafe {STASH = p;}}
// 'a为：对于任意生命周期
// err：p为'a生命周期，最小可能仅有f的生命周期。但STASH具有statc生命周期，违反规则2。如果p被提前释放，那么会造成STASH悬垂指针

fn f(p: &'static i32) { unsafe {STASH = p;}}
static WORTH_POINTING_AT: i32 = 1000;

#[test]
fn ref_as_param()
{
    f(&WORTH_POINTING_AT);
}

fn g<'a>(p: &'a i32)
{}

#[test]
fn throw_ref_to_func()
{
    let x = 10;
    g(&x);
}

fn smallest<'a>(v: &'a [i32]) -> &'a i32
{
    let mut s = &v[0];
    for r in &v[1..]
    {
        if *r < *s
        {
            s = r;
        }
    }
    return s;
}

#[test]
fn return_ref()
{
    let s;
    {
        let parabola = [9,4,1,0,1,4,9];
        s = smallest(&parabola);
        assert_eq!(*s, 0);
    }
    //assert_eq!(*s, 0); //err
}

#[test]
fn struct_incl_ref()
{
    struct S<'a>
    {
        r: &'a i32
    }

    let s;
    {
        let x = 10;
        s = S{r: &x};
        // 引用时，等号右边的生命周期应该大于等于等号左边的生命周期（等号左边的生命周期应该小于等于等号右边的生命周期）
        // 匿名变量S{r: &x}的生命周期为'a，&x传进来时，'a生命周期应该小于等于x生命周期
        // 再将匿名变量赋值给s。但s的生命周期大于匿名变量，矛盾
    }

    // assert_eq!(*s.r, 10); //err
}

/*
struct S<'a>
{
    x: &'a i32,
    y: &'a i32
}
*/

struct S<'a, 'b>
{
    x: &'a i32,
    y: &'b i32
}

#[test]
fn test_multi_lifecycle()
{
    let x = 10;
    let r;
    {
        let y = 20;
        {
            // let s = S {&x, &y};
            let s = S { x:&x, y:&y }; //x和y必须显式标出
            r = s.x; //等式右边的生命周期一定要大于等于左边
                     //x的生命周期大于等于r，满足。但是y生命周期小于r，那么这个结构体跟随谁的生命周期？
                     //因为x和y拥有相同的生命周期'a。解决方法是改类定义，让他们具有不同的生命周期
                     //因此可以发现，'a只是定义的一个生命周期别名，不是any的意思
        }
    }
    println!("{}",r);
}
