fn main() {
    let s1 = String::from("Hallo");
    let s2 = s1.clone();
    println!("{} world!", s1);

    let s = String::from("Hello World!");
    take_ownership(s);

    let x = 5;
    makes_copy(x);

    println!("x = {}", x);

    let mut s = String::from("Hello");

    let len = calculate_length(&mut s);
    println!("s2: {}, len: {}", s, len);

    fn calculate_length(s: &mut String) -> usize {
        s.push_str(" World");
        s.len()
    }

    let s = String::from("sdsd");
    first_word(&s);

    let s = String::from("Chloe Gan");
    let first_word = first_word(&s[..]);
    let second_word = second_word(&s[..]);
    println!("{} {}", first_word, second_word);

    let s = String::from("Hello World");

    let hello = &s[..5];
    let world = &s[6..];
    println!("{} {}", hello, world);
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(num: i32) {
    println!("{}", num);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }
    &s[..]
}
