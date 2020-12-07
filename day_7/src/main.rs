use std::collections::{HashMap, HashSet};

const REAL_INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Debug, PartialEq, Eq)]
struct GraphNode<'a> {
    edges: HashMap<&'a str, u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GraphParseError<'a> {
    BadFormat(&'static str, &'a str),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Graph<'a> {
    nodes: HashMap<&'a str, GraphNode<'a>>,
}

// Allowing myself to use petgraph rather than this custom graph impl would make this solution so
// much shorter!
impl<'a> Graph<'a> {
    fn parse_single(s: &'a str) -> Result<(&str, GraphNode<'a>), GraphParseError> {
        // Expected format:
        //    "{id_1} {id_2} bags contain {edge_spec},*.
        let id_end = s.find(" bags contain ").ok_or(GraphParseError::BadFormat("Missing intial \" bags contain \" sentinel", s))?;
        let id = &s[..id_end];

        // Skip over the "bags contain" bit
        let edges_str = &s[id_end..];
        let edges_str = edges_str
            .strip_prefix(" bags contain ")
            .unwrap();

        if edges_str == "no other bags." {
            return Ok((
                id,
                GraphNode {
                    edges: HashMap::new(),
                },
            ));
        }

        let edges = edges_str
            .split(',')
            .map(str::trim)
            .map(|edge_str| {
                // edge_str should be of the form "N {id_1} {id_2} bag(s)"
                let n_end = edge_str.find(' ').ok_or(GraphParseError::BadFormat("Contains clause has no whitespace", s))?;
                let n = edge_str[..n_end]
                    .parse()
                    .map_err(|_| GraphParseError::BadFormat("Failed to parse number from a bag contains clause", s))?;
                let id = &edge_str[(n_end + 1)..];
                let id = id.strip_suffix(" bag")
                    .or(id.strip_suffix(" bag."))
                    .or(id.strip_suffix(" bags"))
                    .or(id.strip_suffix(" bags."))
                    .ok_or(GraphParseError::BadFormat("Unexpected suffix on bag contains clause", s))?;
                Ok((id, n))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok((id, GraphNode { edges }))
    }

    fn parse_input(s: &'a str) -> Result<Self, GraphParseError> {
        Ok(Self {
            nodes: s
                .lines()
                .map(Self::parse_single)
                .collect::<Result<_, _>>()?,
        })
    }
    
    fn invert_edges(&self) -> Self {
        let mut new_graph = Self {
            nodes: HashMap::new(),
        };

        for (source_id, node) in &self.nodes {
            for (dest_id, value) in &node.edges {
                let node = new_graph.nodes.entry(dest_id)
                    .or_insert(GraphNode { edges: HashMap::new() });
                node.edges.insert(source_id, *value);
            }
        }
        
        new_graph
    }
    
}

fn part_1(graph: &Graph) -> u32 {
    let graph = graph.invert_edges();
    
    let mut available_bags = HashSet::new();
    let mut to_check_set = HashSet::new();
    to_check_set.insert("shiny gold");
    
    while to_check_set.len() > 0 {
        // Remove the first one from the set
        let to_check = to_check_set.take(*to_check_set.iter().next().unwrap()).unwrap();
        
        // Add all of its children to the set of available outer bags
        // Any previously unseen children get added to the set of bags to descend into
        if let Some(node) = graph.nodes.get(to_check) {
            for id in node.edges.keys() {
                if !available_bags.contains(*id) {
                    available_bags.insert(*id);
                    to_check_set.insert(*id);
                }
            }
        }
    }
    
    available_bags.len() as u32
}

fn part_2(graph: &Graph) -> u32 {
    // Memoized recursive method
    fn recursive_value<'a, 'b>(cache: &'a mut HashMap<&'b str, u32>, graph: &'b Graph, id: &'b str) -> u32 {
        if let Some(cached) = cache.get(id) {
            *cached
        } else {
            let value = graph.nodes.get(id)
                .map(|node| node
                    .edges
                    .iter()
                    .map(|(&id, value)| *value * (1 + recursive_value(cache, graph, id)))
                    .sum()
                ).unwrap_or(0);
            
            cache.insert(id, value);
            value
        }
    }

    recursive_value(&mut HashMap::new(), graph, "shiny gold")
}

fn main() {
    let graph = Graph::parse_input(REAL_INPUT)
        .expect("The input failed to parse");

    println!(
        "There are {} bag colors that can eventually contain at least one shiny gold bag",
        part_1(&graph)
    );

    println!(
        "{} individual bags are required inside a singly shiny gold bag",
        part_2(&graph)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single() {
        let test_lines = [
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "faded blue bags contain no other bags.",
        ];

        assert_eq!(
            Graph::parse_single(test_lines[0]),
            Ok((
                "light red",
                GraphNode {
                    edges: [("bright white", 1), ("muted yellow", 2)]
                        .iter()
                        .cloned()
                        .collect()
                }
            ))
        );
        assert_eq!(
            Graph::parse_single(test_lines[1]),
            Ok((
                "bright white",
                GraphNode {
                    edges: [("shiny gold", 1),].iter().cloned().collect()
                }
            ))
        );
        assert_eq!(
            Graph::parse_single(test_lines[2]),
            Ok((
                "faded blue",
                GraphNode {
                    edges: HashMap::new(),
                }
            ))
        );
    }

    const EXAMPLE_INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    
    #[test]
    fn test_invert() {
        let graph = Graph::parse_input(EXAMPLE_INPUT)
            .expect("Valid input");
        
        let graph = graph.invert_edges();

        let shiny_gold = graph.nodes.get("shiny gold")
            .expect("\"shiny gold\" is not in the the inverted graph");
        assert_eq!(shiny_gold.edges.len(), 2);
        assert!(shiny_gold.edges.get("bright white").is_some());
        assert!(shiny_gold.edges.get("muted yellow").is_some());
    }
    
    #[test]
    fn test_part_1() {
        let example_graph = Graph::parse_input(EXAMPLE_INPUT).expect("Expected valid input");
        let real_graph = Graph::parse_input(REAL_INPUT).expect("Expected valid input");

        assert_eq!(part_1(&example_graph), 4);
        assert_eq!(part_1(&real_graph), 337);
    }
    
    #[test]
    fn test_part_2() {
        let example_graph = Graph::parse_input(EXAMPLE_INPUT).expect("Expected valid input");
        let real_graph = Graph::parse_input(REAL_INPUT).expect("Expected valid input");

        assert_eq!(part_2(&example_graph), 32);
        assert_eq!(part_2(&real_graph), 50100);
    }
}
