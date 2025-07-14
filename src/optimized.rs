use crate::common::*;

struct OptimizedSolution;

impl Solvable for OptimizedSolution {
    fn solve(s: &str) -> String {
        let mut arr_s: Vec<char> = s.chars().collect();
        let mut lower_idxs: Vec<usize> = Vec::new();
        let mut upper_idxs: Vec<usize> = Vec::new();

        for i in 0..arr_s.len() {
            if arr_s[i] == 'b' {
                let j = lower_idxs.pop();
                if !j.is_none() {
                    arr_s[j.unwrap()] = ' ';
                }
                arr_s[i] = ' ';
            }
            else if arr_s[i] == 'B' {
                let j = upper_idxs.pop();
                if !j.is_none() {
                    arr_s[j.unwrap()] = ' ';
                }
                arr_s[i] = ' ';
            }
            else {
                if arr_s[i].is_ascii_lowercase() {
                    lower_idxs.push(i);
                }
                else if arr_s[i].is_ascii_uppercase() {
                    upper_idxs.push(i);
                }
                else { panic!("Only ascii chars supported") }
            }
        }

        //Make string from all non-empty chars
        let res = arr_s.iter().filter(|&c| *c != ' ').collect::<String>();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved_correctly(){
        let res = OptimizedSolution::solve("abcdefgijklmnopqrstuvwxyz");
        assert_eq!(res, "cdefgijklmnopqrstuvwxyz");

        let r1 = OptimizedSolution::solve("EeFfGgBb");
        assert_eq!(r1, "EeFf");

        let r2 = OptimizedSolution::solve("bbbbBBBB");
        assert_eq!(r2, "");
    }
}