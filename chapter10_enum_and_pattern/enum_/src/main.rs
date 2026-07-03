fn compare(n: i32, m: i32) -> std::cmp::Ordering
{
    if n < m {std::cmp::Ordering::Less}
    else if n > m {std::cmp::Ordering::Greater}
    else {std::cmp::Ordering::Equal}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TimeUnit
{
    Seconds, Minutes, Hours, Days, Months, Years
}

impl TimeUnit
{
    fn plural(self) -> &'static str
    {
        match self
        {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years"
        }
    }

    fn singular(self) -> &'static str
    {
        self.plural().trim_end_matches('s')
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum RoughTime
{
    InThePast(TimeUnit, u32), //接受参数的是元组型变体
    JustNow,
    InTheFuture(TimeUnit, u32)
}

impl RoughTime
{
    fn to_eng(self) -> String
    {
        match self
        {
            RoughTime::InThePast(units, count) => format!("{} {} ago", count, units.plural()),
            RoughTime::JustNow => format!("just now"),
            RoughTime::InTheFuture(units, count) => format!("{} {} from now", count, units.plural())
        }
    }
}

enum BinaryTree<T> // 泛型枚举
{
    empty,
    NonEmpty(Box<TreeNode<T>>)
}

struct TreeNode<T>
{
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>
}

fn describe_point(x: i32, y: i32) -> &'static str
{
    use std::cmp::Ordering::*;
    match(x.cmp(&0), y.cmp(&0))
    {
        (Equal, Equal) => "at the origin",
        (_, Equal) => "on the x axis",
        (Equal, _) => "on the y axis",
        (Greater, Greater) => "in the first quadrant",
        (Less, Greater) => "in the second quadrant",
        _ => "somewhere else"
    }
}

fn main()
{
    println!("{:?}", compare(1,2));
}

#[test]
fn test_ordering()
{
    assert_eq!(compare(1,2), std::cmp::Ordering::Less);
}

#[test]
fn test_enum_with_data()
{
    let four_score_and_seven_years_ago = RoughTime::InThePast(TimeUnit::Years, 4 * 20 + 7);
    let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Hours, 3);
}

#[test]
fn test_rough_time_pattern_matching()
{
    let rt = RoughTime::InTheFuture(TimeUnit::Months, 1);
    assert_eq!(rt.to_eng(), "1 months from now");
}

#[test]
fn test_desc_point()
{
    let (x, y) = (1, 2);
    assert_eq!(describe_point(x, y), "in the first quadrant");
}

