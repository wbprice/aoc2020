use std::fs;
fn main() {
    let input: Vec<u32> = fs::read_to_string("input")
        .expect("Couldn't read input")
        .lines()
        .map(|value| value.parse().expect("Failed to parse a number"))
        .collect();

    let output = twenty_twenty_finder_multiplier(&input);
    dbg!(output);

    let output = twenty_twenty_finder_multiplier_v2(&input);
    dbg!(output);
}

fn twenty_twenty_finder_multiplier(input: &[u32]) -> Option<u32> {
    for &x in input {
        for &y in input {
            if x + y == 2020 {
                return Some(x * y);
            }
        }
    }
    None
}

fn twenty_twenty_finder_multiplier_v2(input: &[u32]) -> Option<u32> {
    for &x in input {
        for &y in input {
            for &z in input {
                if x + y + z == 2020 {
                    return Some(x * y * z);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_finds_two_entries_that_equal_2020_and_multiplies_them() {
        let input: Vec<u32> = r#"
1721
979
366
299
675
1456
"#
        .split_whitespace()
        .map(|value| value.parse().unwrap())
        .collect();

        let output = twenty_twenty_finder_multiplier(&input);
        assert_eq!(output, Some(514579))
    }
}
