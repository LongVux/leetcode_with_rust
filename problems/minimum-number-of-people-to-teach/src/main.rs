use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn minimum_teachings(
        n: i32,
        languages: Vec<Vec<i32>>,
        friendships: Vec<Vec<i32>>,
    ) -> i32 {
        let mut knows: Vec<HashSet<i32>> = Vec::with_capacity(languages.len());
        for langs in languages {
            let set: HashSet<i32> = langs.into_iter().collect();
            knows.push(set);
        }

        let mut need = vec![false; knows.len()];
        for pair in friendships {
            let u = (pair[0] - 1) as usize;
            let v = (pair[1] - 1) as usize;
            if !shares_language(&knows[u], &knows[v]) {
                need[u] = true;
                need[v] = true;
            }
        }

        let mut people: Vec<usize> = Vec::new();
        for (i, &flag) in need.iter().enumerate() {
            if flag {
                people.push(i);
            }
        }
        if people.is_empty() {
            return 0;
        }

        let mut best = i32::MAX;
        for lang in 1..=n {
            let mut already = 0;
            for &p in &people {
                if knows[p].contains(&lang) {
                    already += 1;
                }
            }
            let teach = (people.len() - already) as i32;
            if teach < best {
                best = teach;
            }
        }

        best
    }
}

fn shares_language(a: &HashSet<i32>, b: &HashSet<i32>) -> bool {
    if a.len() > b.len() {
        return shares_language(b, a);
    }
    for lang in a {
        if b.contains(lang) {
            return true;
        }
    }
    false
}

fn main() {
    let n = 2;
    let languages = vec![vec![1], vec![2], vec![1, 2]];
    let friendships = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
    let ans = Solution::minimum_teachings(n, languages, friendships);
    println!("ans = {}", ans);
    assert_eq!(ans, 1);

    let n = 3;
    let languages = vec![vec![1, 2], vec![2, 3], vec![1, 3]];
    let friendships = vec![vec![1, 2], vec![2, 3], vec![1, 3]];
    let ans = Solution::minimum_teachings(n, languages, friendships);
    println!("ans = {}", ans);
    assert_eq!(ans, 0);
}
