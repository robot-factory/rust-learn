use std::cmp::Ordering;

//　为了提高性能，把所有都放到一个函数
fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    let len = nums.len();
        if len < 4 {
            return result;
        }
    nums.sort();
    for i in 0..len-3 {
        if i > 0 && nums[i] == nums[i-1] {
            continue;
        }
        for j in i+1..len-2 {
            if j > i + 1 && nums[j] == nums[j-1] {
                continue;
            }
            let (mut pre, mut post) = (j+1, len -1);
            while pre < post {
                if pre > j + 1 && nums[pre-1] == nums[pre] {
                    pre += 1;
                    continue
                }
                let sum =nums[i] + nums[j] + nums[pre] + nums[post];
                match sum.cmp(&target) {
                    Ordering::Equal => {
                        result.push(vec![nums[i], nums[j], nums[pre], nums[post]]);
                        pre += 1;
                        post -= 1;
                    },
                    Ordering::Greater => {
                        post -= 1;
                    },
                    Ordering::Less => {
                        pre += 1;
                    }
                }
            }
        }
    }
    result
}

// fn three_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
//     let len = nums.len();
//     let mut result: Vec<Vec<i32>> = vec![];
//     for i in 0..len-2 {
//         let (mut pre, mut post) = (i+1, len -1);
//         while pre < post {
//             let sum =nums[i] + nums[pre] + nums[post];
//             match sum.cmp(&target) {
//                 Ordering::Equal => {
//                     result.push(vec![nums[i], nums[pre], nums[post]]);
//                     pre += 1;
//                 },
//                 Ordering::Greater => {
//                     post -= 1;
//                 },
//                 Ordering::Less => {
//                     pre += 1;
//                 }
//             }
//         }
//     }
//     result
// }

// fn two_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
//     let mut result: Vec<Vec<i32>> = vec![];
//     let len = nums.len();
//     let (mut pre, mut post) = (0, len - 1);
//     while pre < post {
//         let sum = nums[pre] + nums[post];
//         match sum.cmp(&target) {
//             Ordering::Equal => {
//                 result.push(vec![nums[pre], nums[post]]);
//                 pre += 1;
//             }
//             Ordering::Greater => {
//                 post -= 1;
//             }
//             Ordering::Less => {
//                 pre += 1;
//             }
//         }
//     }
//     result
// }

fn main() {
    let v: Vec<i32> = vec![1, 0, -1, 0, -2, 2];
    let target: i32 = 0;
    let result = four_sum(v, target);
    // for i in &result {
    //     print!("{:?}", i);
    // }
    // v.sort();
    // let result = four_sum(&v[..], target);
    println!("{:#?}", result);
}
