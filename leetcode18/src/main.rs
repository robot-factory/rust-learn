fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result:Vec<Vec<i32>> = Vec::new();
        result.push(nums);
        return result
}

fn main() {
    let v: Vec<i32> = vec![1, 0, -1, 0, -2, 2];
    let t = 0;
    let res = four_sum(v, t);
    for i in &res {
        println!("{:?}", i);
    }
}
