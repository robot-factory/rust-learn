use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert((1,2), vec![1,2,3]);
    scores.insert((1,3), vec![4,0,9]);

    let key_name = (1,2);
    let result = scores.get(&key_name).unwrap();
    println!("{:?}",result);

    let n:i32 = 11;
    let n2 = n/2;
    println!("{}:{}",n,n2);
}
