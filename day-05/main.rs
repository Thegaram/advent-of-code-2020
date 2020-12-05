use std::io::{self, BufRead};

type Ticket = (u8, u8);

fn parse_binary(raw: &str) -> Result<u8, String> {
    if raw.len() > 8 {
        return Err(format!("Unexpected binary string: {}", raw));
    }

    let mut num = 0;

    for ch in raw.chars() {
        num = 2 * num
            + match ch {
                'F' => 0,
                'L' => 0,
                'B' => 1,
                'R' => 1,
                _ => return Err(format!("Unexpected binary string: {}", raw)),
            };
    }

    Ok(num)
}

fn parse_ticket(raw: &str) -> Result<Ticket, String> {
    if raw.len() != 10 {
        return Err(format!("Unexpected ticket format: {}", raw));
    }

    let row = parse_binary(&raw[..7])?;
    let col = parse_binary(&raw[7..])?;
    Ok((row, col))
}

fn seat_id(ticket: Ticket) -> u32 {
    let (row, col) = ticket;
    row as u32 * 8 + col as u32
}

fn puzzle_1(ids: &[u32]) -> Option<&u32> {
    ids.iter().max()
}

// assume `ids` is sorted
fn puzzle_2(ids: &[u32]) -> Option<u32> {
    let mut prev = ids[0];

    for id in &ids[1..] {
        if *id != prev + 1 {
            return Some(prev + 1);
        }

        prev = *id;
    }

    None
}

fn main() {
    let stdin = io::stdin();

    let mut ids: Vec<_> = stdin
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|s| parse_ticket(&s[..]))
        .map(Result::unwrap)
        .map(seat_id)
        .collect();

    ids.sort();

    println!("puzzle #1 = {:?}", puzzle_1(&ids));
    println!("puzzle #2 = {:?}", puzzle_2(&ids));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_binary() {
        assert_eq!(parse_binary("FFFFFFF"), Ok(0));
        assert_eq!(parse_binary("FBFBBFF"), Ok(44));
        assert_eq!(parse_binary("BBFBBFF"), Ok(108));
        assert_eq!(parse_binary("BBBBBBB"), Ok(127));

        assert_eq!(parse_binary("LLL"), Ok(0));
        assert_eq!(parse_binary("RLR"), Ok(5));
        assert_eq!(parse_binary("RRR"), Ok(7));

        assert!(parse_binary("FFFFFFFFF").is_err());
        assert!(parse_binary("LRX").is_err());
    }

    #[test]
    fn test_parse_ticket() {
        assert_eq!(parse_ticket("FBFBBFFRLR"), Ok((44, 5)));
        assert_eq!(parse_ticket("BFFFBBFRRR"), Ok((70, 7)));
        assert_eq!(parse_ticket("FFFBBBFRRR"), Ok((14, 7)));
        assert_eq!(parse_ticket("BBFFBBFRLL"), Ok((102, 4)));

        assert!(parse_ticket("BBFFBBRLL").is_err());
        assert!(parse_ticket("BBFFBBFRLLL").is_err());
    }

    #[test]
    fn test_seat_id() {
        assert_eq!(seat_id(parse_ticket("FBFBBFFRLR").unwrap()), 357);
        assert_eq!(seat_id(parse_ticket("BFFFBBFRRR").unwrap()), 567);
        assert_eq!(seat_id(parse_ticket("FFFBBBFRRR").unwrap()), 119);
        assert_eq!(seat_id(parse_ticket("BBFFBBFRLL").unwrap()), 820);
    }
}
