use crate::{Error, Solution};
use itertools::Itertools;

day!(Day05, 2023, 5, "If You Give A Seed A Fertilizer");

impl Solution for Day05 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        Ok(Garden::new(input).closest_location1().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        Ok(Garden::new(input).closest_location2().to_string())
    }
}

struct Garden {
    seeds: Vec<u64>,
    seed_to_soil_map: Vec<Map>,
    soil_to_fertilizer_map: Vec<Map>,
    fertilizer_to_water_map: Vec<Map>,
    water_to_light_map: Vec<Map>,
    light_to_temperature_map: Vec<Map>,
    temperature_to_humidity_map: Vec<Map>,
    humidity_to_location_map: Vec<Map>,
}
impl Garden {
    fn new(input: &str) -> Self {
        let mut input = input.trim().lines();

        let seeds_str = input.next().unwrap().trim_start_matches("seeds:").trim();
        let seeds = seeds_str
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let mut line = "";
        while !line.starts_with("seed-to-soil map:") {
            line = input.next().unwrap();
        }
        line = input.next().unwrap();
        let mut seed_to_soil_map = Vec::new();
        while !line.is_empty() {
            let (dest, src, len) = line.split_whitespace().collect_tuple().unwrap();
            seed_to_soil_map.push(Map::new(
                src.parse().unwrap(),
                dest.parse().unwrap(),
                len.parse().unwrap(),
            ));
            line = input.next().unwrap();
        }

        while !line.starts_with("soil-to-fertilizer map:") {
            line = input.next().unwrap();
        }
        line = input.next().unwrap();
        let mut soil_to_fertilizer_map = Vec::new();
        while !line.is_empty() {
            let (dest, src, len) = line.split_whitespace().collect_tuple().unwrap();
            soil_to_fertilizer_map.push(Map::new(
                src.parse().unwrap(),
                dest.parse().unwrap(),
                len.parse().unwrap(),
            ));
            line = input.next().unwrap();
        }

        while !line.starts_with("fertilizer-to-water map:") {
            line = input.next().unwrap();
        }
        line = input.next().unwrap();
        let mut fertilizer_to_water_map = Vec::new();
        while !line.is_empty() {
            let (dest, src, len) = line.split_whitespace().collect_tuple().unwrap();
            fertilizer_to_water_map.push(Map::new(
                src.parse().unwrap(),
                dest.parse().unwrap(),
                len.parse().unwrap(),
            ));
            line = input.next().unwrap();
        }

        while !line.starts_with("water-to-light map:") {
            line = input.next().unwrap();
        }
        line = input.next().unwrap();
        let mut water_to_light_map = Vec::new();
        while !line.is_empty() {
            let (dest, src, len) = line.split_whitespace().collect_tuple().unwrap();
            water_to_light_map.push(Map::new(
                src.parse().unwrap(),
                dest.parse().unwrap(),
                len.parse().unwrap(),
            ));
            line = input.next().unwrap();
        }

        while !line.starts_with("light-to-temperature map:") {
            line = input.next().unwrap();
        }
        line = input.next().unwrap();
        let mut light_to_temperature_map = Vec::new();
        while !line.is_empty() {
            let (dest, src, len) = line.split_whitespace().collect_tuple().unwrap();
            light_to_temperature_map.push(Map::new(
                src.parse().unwrap(),
                dest.parse().unwrap(),
                len.parse().unwrap(),
            ));
            line = input.next().unwrap();
        }

        while !line.starts_with("temperature-to-humidity map:") {
            line = input.next().unwrap();
        }
        line = input.next().unwrap();
        let mut temperature_to_humidity_map = Vec::new();
        while !line.is_empty() {
            let (dest, src, len) = line.split_whitespace().collect_tuple().unwrap();
            temperature_to_humidity_map.push(Map::new(
                src.parse().unwrap(),
                dest.parse().unwrap(),
                len.parse().unwrap(),
            ));
            line = input.next().unwrap();
        }

        while !line.starts_with("humidity-to-location map:") {
            line = input.next().unwrap();
        }
        line = input.next().unwrap();
        let mut humidity_to_location_map = Vec::new();
        while !line.is_empty() {
            let (dest, src, len) = line.split_whitespace().collect_tuple().unwrap();
            humidity_to_location_map.push(Map::new(
                src.parse().unwrap(),
                dest.parse().unwrap(),
                len.parse().unwrap(),
            ));
            line = input.next().unwrap_or("");
        }

        Self {
            seeds,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        }
    }

    fn closest_location1(&self) -> u64 {
        self.seeds
            .iter()
            .map(|&seed| self.seed_to_location(seed))
            .min()
            .unwrap()
    }

    fn seed_to_location(&self, seed: u64) -> u64 {
        let soil = Self::calculate_map(seed, &self.seed_to_soil_map);
        let fertilizer = Self::calculate_map(soil, &self.soil_to_fertilizer_map);
        let water = Self::calculate_map(fertilizer, &self.fertilizer_to_water_map);
        let light = Self::calculate_map(water, &self.water_to_light_map);
        let temperature = Self::calculate_map(light, &self.light_to_temperature_map);
        let humidity = Self::calculate_map(temperature, &self.temperature_to_humidity_map);
        Self::calculate_map(humidity, &self.humidity_to_location_map)
    }

    fn calculate_map(value: u64, maps: &[Map]) -> u64 {
        for map in maps {
            if let Some(converted) = map.convert(value) {
                return converted;
            }
        }
        value
    }

    fn closest_location2(&self) -> u64 {
        let mut intervals = Vec::new();
        for (start, len) in self.seeds.iter().tuples() {
            intervals.push(Interval::new(*start, *start + *len));
        }

        intervals = Self::calculate_map_intervals(intervals, &self.seed_to_soil_map);
        intervals = Self::calculate_map_intervals(intervals, &self.soil_to_fertilizer_map);
        intervals = Self::calculate_map_intervals(intervals, &self.fertilizer_to_water_map);
        intervals = Self::calculate_map_intervals(intervals, &self.water_to_light_map);
        intervals = Self::calculate_map_intervals(intervals, &self.light_to_temperature_map);
        intervals = Self::calculate_map_intervals(intervals, &self.temperature_to_humidity_map);
        intervals = Self::calculate_map_intervals(intervals, &self.humidity_to_location_map);

        intervals
            .into_iter()
            .map(|interval| interval.start)
            .min()
            .unwrap()
    }

    fn calculate_map_intervals(mut intervals: Vec<Interval>, maps: &[Map]) -> Vec<Interval> {
        let mut output_intervals = Vec::new();

        while let Some(working_interval) = intervals.pop() {
            let mut changed = false;

            for map in maps {
                if map.src_start <= working_interval.start
                    && working_interval.end <= (map.src_start + map.range_len - 1)
                {
                    // interval is completely contained in map (or equal to map)

                    output_intervals.push(Interval::new(
                        map.dest_start + (working_interval.start - map.src_start),
                        map.dest_start + (working_interval.end - map.src_start),
                    ));
                    changed = true;
                    break;
                } else if working_interval.start <= map.src_start
                    && (map.src_start + map.range_len - 1) <= working_interval.end
                {
                    // interval is completely containing map (it can't be equal here)

                    // if it is zero this wouldn't work because of underflow, otherwise logic is correct
                    if map.src_start > 0 {
                        let interval_left =
                            Interval::new(working_interval.start, map.src_start - 1);
                        if interval_left.start <= interval_left.end {
                            intervals.push(interval_left);
                        }
                    }

                    // translate whole map to output (because it is contained in interval)
                    output_intervals.push(Interval::new(
                        map.dest_start,
                        map.dest_start + map.range_len - 1,
                    ));

                    let interval_right =
                        Interval::new(map.src_start + map.range_len, working_interval.end);
                    if interval_right.start <= interval_right.end {
                        intervals.push(interval_right);
                    }

                    changed = true;
                    break;
                } else if map.src_start <= working_interval.start
                    && working_interval.start <= (map.src_start + map.range_len - 1)
                {
                    // interval starts in map (it is not contained in map)

                    // translate part of map to output (because it is contained in interval)
                    output_intervals.push(Interval::new(
                        map.dest_start + (working_interval.start - map.src_start),
                        map.dest_start + map.range_len - 1,
                    ));

                    let interval_right =
                        Interval::new(map.src_start + map.range_len, working_interval.end);
                    intervals.push(interval_right);

                    changed = true;
                    break;
                } else if map.src_start <= working_interval.end
                    && working_interval.end <= (map.src_start + map.range_len - 1)
                {
                    // interval ends in map (it is not contained in map)

                    let interval_left = Interval::new(working_interval.start, map.src_start - 1);
                    intervals.push(interval_left);

                    // translate part of map to output (because it is contained in interval)
                    output_intervals.push(Interval::new(
                        map.dest_start,
                        map.dest_start + (working_interval.end - map.src_start),
                    ));

                    changed = true;
                    break;
                }
            }

            if !changed {
                output_intervals.push(working_interval);
            }
        }

        output_intervals
    }
}
struct Map {
    src_start: u64,
    dest_start: u64,
    range_len: u64,
}
impl Map {
    fn new(src_start: u64, dest_start: u64, range_len: u64) -> Self {
        Self {
            src_start,
            dest_start,
            range_len,
        }
    }
    fn convert(&self, value: u64) -> Option<u64> {
        if (self.src_start..(self.src_start + self.range_len)).contains(&value) {
            Some(self.dest_start + (value - self.src_start))
        } else {
            None
        }
    }
}

struct Interval {
    start: u64,
    end: u64, // inclusive
}
impl Interval {
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }
}
