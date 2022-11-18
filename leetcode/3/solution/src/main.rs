mod solution {
    use std::collections::HashSet;

    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut occ: HashSet<char> = HashSet::new();
        let cv: Vec<char> = s.chars().collect();
        let (mut r, mut ans, n) = (0, 0, s.len());
        for i in 0..n {
            if i != 0 {
                occ.remove(&cv[i - 1]);
            }
            while r < n && !occ.contains(&cv[r]) {
                occ.insert(cv[r]);
                r += 1;
            }
            ans = ans.max(r - i);
        }
        ans as i32
    }
}

fn main() {
    let rnt = solution::length_of_longest_substring(String::from("pwwkew"));
    println!("{}", rnt);
}
