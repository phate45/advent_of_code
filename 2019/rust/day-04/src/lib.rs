pub fn part1(_source: &str) -> String {
    let mut c = 0u32;

    numz(|n| {
        if verify(n) {
            c += 1;
        }
    });

    c.to_string()
}

pub fn part2(_source: &str) -> String {
    let mut c = 0u32;

    numz(|n| {
        if verify2(n) {
            c += 1;
        }
    });

    c.to_string()
}

fn numz(mut verifier: impl FnMut(u32)) {
    'main: for v1 in 4..=8 {
        for v2 in v1..=9 {
            for v3 in v2..=9 {
                for v4 in v3..=9 {
                    for v5 in v4..=9 {
                        for v6 in v5..=9 {
                            let num =
                                v1 * 10u32.pow(5)
                                + v2 * 10u32.pow(4)
                                + v3 * 10u32.pow(3)
                                + v4 * 100
                                + v5 * 10
                                + v6;

                            if num < 402328 {
                                continue;
                            }
                            if num >= 864247 {
                                break 'main;
                            }
                            verifier(num);
                        }
                    }
                }
            }
        }
    }
}

fn verify(number: u32) -> bool {
    let s = number.to_string();
    let mut v = s.chars().collect::<Vec<char>>();

    v.dedup();

    s.len() > v.len()
}

fn verify2(number: u32) -> bool {
    let mut last: char = 'a';
    let mut count: u8 = 0;

    for c in number.to_string().chars() {
        if last != c {
            if count == 2 {
                return true;
            }
            last = c;
            count = 1;
        } else {
            count += 1;
        }
    }

    count == 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(verify(112));
        assert!(!verify(123));
        assert!(verify(111111));
        assert!(!verify(123789));
    }

    #[test]
    fn test_math() {
        assert_eq!(10u32.pow(3), 1000);
        assert_eq!(10u64.pow(4), 10000);
        let a: u64 = 10;
        assert_eq!(a.pow(5), 100000);
    }

    #[test]
    fn test_part2() {
        assert!(verify2(112));
        assert!(verify2(111122));
        assert!(!verify2(1112));
        assert!(!verify2(123789));
    }
}
