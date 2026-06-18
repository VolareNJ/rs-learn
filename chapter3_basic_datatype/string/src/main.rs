fn main()
{
    println!(r###"
        This raw str starts with 'r###'.
        Therefore it does not end until we reach a  quote mark('"')
        followed immediately by 3 pound signs ('###'):"###);
}


#[test]
fn test_byte_arr()
{
    let method: &[u8;3] = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);
}

#[test]
fn test_string()
{
    let noodles: String = "noodles".to_string();
    let oodles: &str = &noodles[1..];
    let poodles: &str = "O_o";

    assert_eq!("O_o".len(), 3);
    assert_eq!("O_o".chars().count(), 3);
}

#[test]
fn test_string_creation()
{
    let error_message: String = "too many pets".to_string();
    let msg2: String = format!("{} {} {}",1,2,3).to_string();
    let bits = vec!["veni", "vidi", "vici"];
    assert_eq!(bits.concat(), "venividivici");
    assert_eq!(bits.join(", "), "veni, vidi, vici");
}
