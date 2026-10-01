use crate::{Error, Solution};

day!(Day13, 2017, 13, "Packet Scanners");

impl Solution for Day13 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        Ok(Firewall::new(input).severity().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        Ok(Firewall::new(input).safe_passage().to_string())
    }
}

struct Firewall {
    layers: Vec<Layer>,
}
impl Firewall {
    fn new(input: &str) -> Self {
        let layers = input.lines().map(Layer::new).collect();
        Self { layers }
    }
    fn severity(&self) -> usize {
        self.layers.iter().fold(0, |acc, layer| {
            let time = layer.depth;
            if time % layer.scanner_return_period == 0 {
                acc + layer.depth * layer.range
            } else {
                acc
            }
        })
    }
    fn safe_passage(&self) -> usize {
        'outer: for delay in 0.. {
            for layer in &self.layers {
                let time = layer.depth + delay;
                if time % layer.scanner_return_period == 0 {
                    continue 'outer;
                }
            }
            return delay;
        }
        unreachable!("The previous loop can only end by returning a value.")
    }
}

struct Layer {
    depth: usize,
    range: usize,
    scanner_return_period: usize,
}
impl Layer {
    fn new(layer: &str) -> Self {
        let (depth, range) = layer.trim().split_once(": ").unwrap();
        let depth = depth.parse().unwrap();
        let range = range.parse().unwrap();
        let scanner_return_period = (range - 1) * 2;
        Self {
            depth,
            range,
            scanner_return_period,
        }
    }
}
