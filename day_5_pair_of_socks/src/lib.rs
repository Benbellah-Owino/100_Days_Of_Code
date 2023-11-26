pub mod socks_pair {
    #[derive(Debug)]
    struct item {
        val: char,
        number: i32,
    }

    pub fn pair(socks: String) -> i32 {
        let mut v: Vec<char> = Vec::new();
        let mut items: Vec<item> = Vec::new();
        let mut pairs = 0;

        for j in socks.chars() {
            if !v.contains(&j) {
                v.push(j);
                items.push(item { val: j, number: 1 });
            } else {
                let pos = v.iter().position(|n| n == &j).unwrap();
                //println!("{pos}",);
                items[pos].number += 1;
            }
        }

        for i in &items {
            if i.number % 2 == 0 {
                pairs += i.number / 2;
            }
        }

        pairs
    }
}

#[cfg(test)]
mod tests {
    use crate::socks_pair::pair;

    #[test]
    fn test() {
        assert_eq!(1, pair(String::from("AA")));
    }
    #[test]
    fn test2() {
        assert_eq!(2, pair(String::from("ABABC")));
    }
    #[test]
    fn test3() {
        assert_eq!(4, pair(String::from("CABBACCC")));
    }
}
