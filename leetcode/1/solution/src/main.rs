mod solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut heap: HashMap<i32, i32> = HashMap::new();
        for (idx, val) in nums.iter().enumerate() {
            match heap.get(&(target - val)) {
                Some(idj) => return vec![idx as i32, *idj],
                None => {
                    heap.insert(*val, idx as i32);
                }
            }
        }
        panic!("error");
    }
}
fn main() {
    println!("{:?}", solution::two_sum(vec![2, 7, 11, 15], 9));
}
