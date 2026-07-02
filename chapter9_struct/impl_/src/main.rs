//泛型
pub struct Queue<T>
{
    older: Vec<T>,
    younger: Vec<T>
}

impl<T> Queue<T>
{
    pub fn push_back(&mut self, t: T)
    {
        self.younger.push(t);
    }

    pub fn pop(&mut self) ->Option<T>
    {
        if self.older.is_empty()
        {
            if self.younger.is_empty()
            {
                return None;
            }
            
            std::mem::swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        self.older.pop()
    }

    pub fn new() -> Self //不接收self的关联函数
                         //返回大驼峰Self，等价于Queue<T>
    {
        Queue{older: Vec::new(), younger: Vec::new()}
    }
}

pub struct Vector2
{
    x: f32,
    y: f32
}

impl Vector2 //关联常量
{
    const ZERO: Vector2 = Vector2{x: 0.0, y: 0.0};
    const UNIT: Vector2 = Vector2{x: 1.0, y: 0.0};
}

struct Polynomial<const N: usize> //接受常量作为泛型参数。只能是整数，char和bool
{
    coefficients: [f64; N]
}

impl<const N: usize> Polynomial<N>
{
    fn new(coefficients: [f64; N]) -> Polynomial<N>
    {
        Polynomial{coefficients}
    }

    fn eval(&self, x: f64) -> f64
    {
        let mut sum = 0.0;
        for i in (0..N).rev()
        {
            sum = self.coefficients[i] + x * sum;
        }
        sum
    }
}

//先生命周期，再类型，再整数
struct LumpOfReferences<'a, T, const N: usize>
{
    the_lump: [&'a T; N]
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point
{
    x: f64,
    y: f64
}

fn main()
{
    println!("Hello, world!");
}

#[test]
fn test_queue()
{
    //let mut q = Queue{older: Vec::new(), younger: Vec::new()};
    let mut q  = Queue::<char>::new(); //等价于上面

    q.push_back('0');
    q.push_back('1');
    assert_eq!(q.pop(), Some('0'));

    q.push_back('i');
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('i'));
    assert_eq!(q.pop(), None);
}
