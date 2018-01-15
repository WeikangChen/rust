use std::ops::Add;
use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::HashSet;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Cord(i32, i32, i32, i32);
#[derive(Debug, Clone)]
struct Diff(i32, i32, i32, i32);

#[allow(dead_code)]
impl Cord {
    fn print(&self) {
        println!("{:?}", self);
    }
}

impl Add<Diff> for Cord {
    type Output = Cord;
    fn add(self, other: Diff) -> Cord {
        Cord((self.0 + other.0 + 10) % 10, 
             (self.1 + other.1 + 10) % 10,
             (self.2 + other.2 + 10) % 10,
             (self.3 + other.3 + 10) % 10 
        )
    }
}

impl FromStr for Cord {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const RADIX: u32 = 10;
        let ds = s.chars()
            .take(4)
            .map(|x| x.to_digit(RADIX).unwrap() as i32)
            .collect::<Vec<i32>>();
        Ok(Cord(ds[0], ds[1], ds[2], ds[3]))
    }
}

fn open_lock(deadends: &[&str], target: &str) -> i32 {
    let deads = deadends
        .iter()
        .map(|x| x.parse::<Cord>().unwrap())
        .collect::<Vec<_>>();
    let tar = target.parse::<Cord>().unwrap();
    open_lock_impl(&deads, &tar).unwrap_or(-1)
}

fn open_lock_impl(deads: &[Cord], tar: &Cord) -> Option<i32> {
    let dirs = vec!(
        Diff(1, 0, 0, 0), Diff(-1, 0, 0, 0),
        Diff(0, 1, 0, 0), Diff( 0,-1, 0, 0),
        Diff(0, 0, 1, 0), Diff( 0, 0,-1, 0),
        Diff(0, 0, 0, 1), Diff( 0, 0, 0,-1),
    ); 

    let deads: HashSet<_> = deads.into_iter().collect();
    // println!("{:?}", deads);
    let src = "0000".parse::<Cord>().unwrap();
    if src == *tar {
        return Some(0);
    }
    if deads.contains(&src) {
        return None;
    }

    let mut visited = HashSet::new();
    visited.insert(src.clone());
   
    let mut nxt_cyc = vec!(src);
    let mut len = 0;
    while nxt_cyc.len() != 0 {
        // println!("{:?}", nxt_cyc);
        len += 1; 
        let cur_cyc = nxt_cyc;
        nxt_cyc = Vec::new();
        for cur in cur_cyc {
            for d in dirs.clone() {
                let nxt = cur.clone() + d;
                if nxt == *tar {
                    return Some(len);
                }
                if deads.contains(&nxt) {
                    continue;
                }
                if visited.contains(&nxt) {
                    continue;
                }
                visited.insert(nxt.clone());
                nxt_cyc.push(nxt);
            }
        }
    }
    return None;
}

fn main() {
    let deadends = &["0201","0101","0102","1212","2002"];
    let target = "0202";
    let dist = open_lock(deadends, target);
    println!("distance = {}", dist);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let deadends = &["0201","0101","0102","1212","2002"];
        let target = "0202";
        assert_eq!(6, open_lock(deadends, target));
    }
    #[test]
    fn test_2() {
        let deadends = &["8888"];
        let target = "0009";
        assert_eq!(1, open_lock(deadends, target));
    }
    #[test]
    fn test_3() {
        let deadends = &["0000"];
        let target = "8888";
        assert_eq!(-1, open_lock(deadends, target));
    }
    #[test]
    fn test_4() {
        let deadends = &["8887","8889","8878","8898","8788","8988","7888","9888"];
        let target = "8888";
        assert_eq!(-1, open_lock(deadends, target));
    }

}
