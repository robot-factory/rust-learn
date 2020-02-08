fn main() {
  let lists = vec![2,4,6,8,10];
  for (i,n) in lists.iter().enumerate() {
    println!("lists[{}]={}",i,n);
  }
}