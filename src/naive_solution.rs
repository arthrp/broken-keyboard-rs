use crate::common::*;

pub struct NaiveSolution;

#[derive(Copy, Clone)]
struct CustomChar {
    pos: usize,
    val: char
}

impl Solvable for NaiveSolution {
    fn solve(s: &str) -> String {
        let mut lower: Vec<CustomChar> = Vec::new();
        let mut upper: Vec<CustomChar> = Vec::new();
        let mut i: usize = 0;

        for c in s.chars() {
            if c == 'b' {
                lower.pop();
            }
            else if c == 'B' {
                upper.pop();
            }
            else {
                if c.is_ascii_lowercase(){
                    lower.push(CustomChar { val: c, pos: i } );
                }
                else if c.is_ascii_uppercase(){
                    upper.push(CustomChar { val: c, pos: i } );
                }
                else{ panic!("Only ascii chars supported") }
            }
            i += 1;
        }

        let mut res = String::from("");
        for i in 0..s.len() {
            let lower_target = lower.iter().find(|&&x| x.pos == i);

            if !lower_target.is_none() {
                res.push(lower_target.unwrap().val);
                continue;
            }
            
            let upper_target = upper.iter().find(|&&x| x.pos == i);

            if !upper_target.is_none() {
                res.push(upper_target.unwrap().val);
            }
        }

        res
    }
}

mod tests {
    use super::*;

    #[test]
    fn solved_correctly(){
        let res = NaiveSolution::solve("abcdefgijklmnopqrstuvwxyz");
        assert_eq!(res, "cdefgijklmnopqrstuvwxyz");

        let r1 = NaiveSolution::solve("EeFfGgBb");
        assert_eq!(r1, "EeFf");

        let r2 = NaiveSolution::solve("bbbbBBBB");
        assert_eq!(r2, "");
    }
}