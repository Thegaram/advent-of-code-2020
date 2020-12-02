use text_io::try_scan;

type Policy = (char, usize, usize);

fn is_valid_1(password: &String, policy: Policy) -> bool {
    let (letter, min, max) = policy;
    let count = password.chars().filter(|ch| *ch == letter).count();
    (count >= min) && (count <= max)
}

fn is_valid_2(password: &String, policy: Policy) -> bool {
    let (letter, id_1, id_2) = policy;

    let ch_1 = password.chars().nth(id_1 - 1).unwrap();
    let ch_2 = password.chars().nth(id_2 - 1).unwrap();

    (ch_1 == letter) ^ (ch_2 == letter)
}

fn read_line() -> Result<(String, Policy), text_io::Error> {
    let (min, max, letter, password): (usize, usize, char, String);
    try_scan!("{}-{} {}: {}", min, max, letter, password);
    Ok((password, (letter, min, max)))
}

fn main() {
    let mut count_valid_1 = 0;
    let mut count_valid_2 = 0;

    while let Ok((password, policy)) = read_line() {
        if is_valid_1(&password, policy) {
            count_valid_1 += 1;
        }

        if is_valid_2(&password, policy) {
            count_valid_2 += 1;
        }
    }

    println!("puzzle #1: {}", count_valid_1);
    println!("puzzle #2: {}", count_valid_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_1() {
        assert_eq!(is_valid_1(&"abcde".to_owned(), ('a', 1, 3)), true);
        assert_eq!(is_valid_1(&"cdefg".to_owned(), ('b', 1, 3)), false);
        assert_eq!(is_valid_1(&"ccccccccc".to_owned(), ('c', 2, 9)), true);
    }

    #[test]
    fn test_is_valid_2() {
        assert_eq!(is_valid_2(&"abcde".to_owned(), ('a', 1, 3)), true);
        assert_eq!(is_valid_2(&"cdefg".to_owned(), ('b', 1, 3)), false);
        assert_eq!(is_valid_2(&"ccccccccc".to_owned(), ('c', 2, 9)), false);
    }
}
