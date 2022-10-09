
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.sort_unstable_by(|a, b| b.cmp(a));
        for i in 2..n {
            let (x, y, z) = (nums[i], nums[i-1], nums[i-2]);
            if z < y + x {
                return x + y + z;
            }
        }
        0
    }

fn main() {
    let arr = vec![1,1,2];
    let result: i32 = largest_perimeter(arr);
    println!("Result: {:?}", result);
}
