use crate::{Error, Solution};

day!(Day08, 2018, 8, "Memory Maneuver");

impl Solution for Day08 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let license_file = input
            .split_whitespace()
            .map(|x| x.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        let root = Node::build(&license_file, 0).0;
        Ok(root.sum_metadata().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let license_file = input
            .split_whitespace()
            .map(|x| x.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        let root = Node::build(&license_file, 0).0;
        Ok(root.node_value().to_string())
    }
}

struct Node {
    children: Vec<Node>,
    metadata: Vec<u8>,
    node_value: Option<u32>,
}
impl Node {
    fn build(license_file: &[u8], mut i: usize) -> (Self, usize) {
        let children_count = license_file[i] as usize;
        let metadata_count = license_file[i + 1] as usize;
        i += 2;

        let mut children = Vec::new();
        let mut metadata = Vec::new();

        for _ in 0..children_count {
            let (child, new_i) = Self::build(license_file, i);
            children.push(child);
            i = new_i;
        }

        for _ in 0..metadata_count {
            metadata.push(license_file[i]);
            i += 1;
        }

        let node = Self {
            children,
            metadata,
            node_value: None,
        };

        (node, i)
    }

    fn sum_metadata(&self) -> u32 {
        self.metadata.iter().map(|x| *x as u32).sum::<u32>()
            + self.children.iter().map(|x| x.sum_metadata()).sum::<u32>()
    }

    fn node_value(&self) -> u32 {
        if let Some(value) = self.node_value {
            value
        } else if self.children.is_empty() {
            self.metadata.iter().map(|x| *x as u32).sum()
        } else {
            let mut sum = 0;
            for &meta_value in &self.metadata {
                if (meta_value as usize) <= self.children.len() && meta_value != 0 {
                    sum += self.children[meta_value as usize - 1].node_value();
                }
            }
            sum
        }
    }
}
