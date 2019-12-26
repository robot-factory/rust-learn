fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    // s.clear(); // 错误!

    println!("the first word is: {}", word);
}
