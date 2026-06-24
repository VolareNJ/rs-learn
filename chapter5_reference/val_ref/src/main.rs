use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table)
{
    for (artist, works) in table //table已经是指针，无需再取地址
    {
        println!("works by {}:", artist);
        for work in works
        {
            println!(" {}", work);
        }
    }
}

fn main()
{
    let mut table = Table::new();
    table.insert("a".to_string(), vec!["a1".to_string(), "a2".to_string()]);
    table.insert("b".to_string(), vec!["b1".to_string(), "b2".to_string()]);
    table.insert("c".to_string(), vec!["c1".to_string(), "c2".to_string()]);

    show(&table);

}
