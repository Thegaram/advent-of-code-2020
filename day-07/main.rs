use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufRead};

type Graph = HashMap<String, Vec<(String, u32)>>;
const TARGET: &str = "shiny gold";

fn read_graph() -> io::Result<Graph> {
    let outer_re = Regex::new(r"^(.*) bags contain ").unwrap();
    let inner_re = Regex::new(r"(\d+) (.*?) bag").unwrap();

    let stdin = io::stdin();
    let mut graph: Graph = HashMap::new();

    for line in stdin.lock().lines() {
        let line = line?;

        let cap = outer_re.captures(&line).unwrap();
        let outer = cap[1].to_owned();

        graph.entry(outer.clone()).or_insert(vec![]);

        for cap in inner_re.captures_iter(&line) {
            let number = cap[1].parse::<u32>().unwrap();
            let inner = cap[2].to_owned();

            graph.get_mut(&outer).unwrap().push((inner, number));
        }
    }

    Ok(graph)
}

fn is_reachable(from: &str, target: &str, graph: &Graph) -> bool {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut queue: VecDeque<&str> = VecDeque::new();
    queue.push_back(from);

    loop {
        let current = match queue.pop_front() {
            None => break,
            Some(c) if c == target => return true,
            Some(c) if visited.contains(c) => continue,
            Some(c) => c,
        };

        visited.insert(current);

        if let Some(nodes) = graph.get(current) {
            for (node, _) in nodes {
                queue.push_back(&node);
            }
        }
    }

    false
}

fn count_bags(from: &str, graph: &Graph) -> u32 {
    match graph.get(from) {
        None => 1,
        Some(bags) => bags
            .iter()
            .fold(1, |acc, (b, n)| acc + n * count_bags(b, graph)),
    }
}

fn puzzle_1(bag: &str, graph: &Graph) -> usize {
    graph
        .keys()
        .filter(|k| *k != bag)
        .map(|k| is_reachable(k, bag, &graph))
        .filter(|r| *r)
        .count()
}

fn puzzle_2(bag: &str, graph: &Graph) -> u32 {
    count_bags(bag, &graph) - 1
}

fn main() -> io::Result<()> {
    let graph = read_graph()?;
    println!("puzzle #1 = {:?}", puzzle_1(TARGET, &graph));
    println!("puzzle #2 = {:?}", puzzle_2(TARGET, &graph));
    Ok(())
}
