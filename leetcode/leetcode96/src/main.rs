
fn num_trees(n: i32) -> i32 {
    if n == 1 || n == 0{
        return 1;
    }
    let mut result = 0;
    for i in 0..n  {
        result += num_trees(i) * num_trees(n - 1 - i);
    }
    return result;
}

fn num_trees_impl(n: i32 ) -> i32 {
    if n < 3{
        return n;
    }
    let n = n as usize;
    let mut memo = vec![0i32;n+1];
    memo[0] = 0;
    memo[1] = 1;
    memo[2] = 2;
    for i in 3..n+1 {
        memo[i] = (0..i).map(|j| memo[j]*memo[i-1-j]).sum();
    }
    memo[n] 

}

fn main() {
    let r = num_trees(3);
    println!("3:{}", r);
}
