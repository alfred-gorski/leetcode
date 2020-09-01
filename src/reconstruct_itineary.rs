use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn dfs(f: String, d: &mut HashMap<&String, Vec<&String>>, ans: &mut Vec<String>) {
        while let Some(ff) = d.get_mut(&f).unwrap_or(&mut vec![]).pop() {
            Self::dfs(ff.to_string(), d, ans)
        }
        ans.push(f.to_string());
    }

    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut d: HashMap<&String, Vec<&String>> = HashMap::new();
        for t in 0..tickets.len() {
            d.entry(&tickets[t][0]).or_default().push(&tickets[t][1]);
        }
        for (_, f) in d.iter_mut() {
            f.sort_by(|a, b| b.cmp(a));
        }
        let mut ans = vec![];
        Self::dfs("JFK".to_string(), &mut d, &mut ans);
        ans.reverse();
        ans
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let inp = vec![
            vec![String::from("MUC"), String::from("LHR")],
            vec![String::from("JFK"), String::from("MUC")],
            vec![String::from("SFO"), String::from("SJC")],
            vec![String::from("LHR"), String::from("SFO")],
        ];
        let output = vec!["JFK", "MUC", "LHR", "SFO", "SJC"];
        assert_eq!(Solution::find_itinerary(inp), output);
    }
}
