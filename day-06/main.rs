use std::collections::HashSet;
use std::io::{self, Read};

type Declaration = Vec<HashSet<char>>;
type Problem = Vec<Declaration>;

fn read_declaration(chars: &mut impl Iterator<Item = char>) -> Declaration {
    let mut declaration = vec![HashSet::new()];

    for ch in chars {
        if ch == '\n' {
            if declaration.last().unwrap().is_empty() {
                break;
            }

            declaration.push(HashSet::new());
            continue;
        }

        declaration.last_mut().unwrap().insert(ch);
    }

    if declaration.last().unwrap().is_empty() {
        declaration.pop();
    }

    declaration
}

fn read_problem(chars: &mut impl Iterator<Item = char>) -> Problem {
    let mut problem = vec![];

    loop {
        match read_declaration(chars) {
            d if d.is_empty() => break,
            d => problem.push(d),
        }
    }

    problem
}

fn count_anyone(declaration: &Declaration) -> usize {
    declaration
        .iter()
        .fold(HashSet::new(), |acc, d| acc.union(d).cloned().collect())
        .len()
}

fn count_everyone(declaration: &Declaration) -> usize {
    declaration
        .iter()
        .skip(1)
        .fold(declaration[0].clone(), |acc, d| {
            acc.intersection(d).cloned().collect()
        })
        .len()
}

fn puzzle_1(problem: &Problem) -> usize {
    problem.iter().map(count_anyone).sum()
}

fn puzzle_2(problem: &Problem) -> usize {
    problem.iter().map(count_everyone).sum()
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let mut iter = buffer.chars();
    let problem = read_problem(&mut iter);

    println!("puzzle #1 = {:?}", puzzle_1(&problem));
    println!("puzzle #2 = {:?}", puzzle_2(&problem));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashset as set;

    #[test]
    fn test_read_declaration() {
        let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
        let mut iter = input.chars();

        assert_eq!(read_declaration(&mut iter), vec![set! {'a', 'b', 'c'}]);
        assert_eq!(
            read_declaration(&mut iter),
            vec![set! {'a'}, set! {'b'}, set! {'c'}]
        );
        assert_eq!(
            read_declaration(&mut iter),
            vec![set! {'a', 'b'}, set! {'a', 'c'}]
        );
        assert_eq!(
            read_declaration(&mut iter),
            vec![set! {'a'}, set! {'a'}, set! {'a'}, set! {'a'}]
        );
        assert_eq!(read_declaration(&mut iter), vec![set! {'b'}]);
    }

    #[test]
    fn test_count_anyone() {
        assert_eq!(count_anyone(&vec![set! {'a', 'b', 'c'}]), 3);
        assert_eq!(count_anyone(&vec![set! {'a'}, set! {'b'}, set! {'c'}]), 3);
        assert_eq!(count_anyone(&vec![set! {'a', 'b'}, set! {'a', 'c'}]), 3);
        assert_eq!(
            count_anyone(&vec![set! {'a'}, set! {'a'}, set! {'a'}, set! {'a'}]),
            1
        );
        assert_eq!(count_anyone(&vec![set! {'b'}]), 1);
    }

    #[test]
    fn test_count_everyone() {
        assert_eq!(count_everyone(&vec![set! {'a', 'b', 'c'}]), 3);
        assert_eq!(count_everyone(&vec![set! {'a'}, set! {'b'}, set! {'c'}]), 0);
        assert_eq!(count_everyone(&vec![set! {'a', 'b'}, set! {'a', 'c'}]), 1);
        assert_eq!(
            count_everyone(&vec![set! {'a'}, set! {'a'}, set! {'a'}, set! {'a'}]),
            1
        );
        assert_eq!(count_everyone(&vec![set! {'b'}]), 1);
    }
}
