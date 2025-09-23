//! Mathematical functions

use std::borrow::Borrow;

/// A character representation of hexadecimal digits.
pub const HEX_DIGITS: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

/// Find the solution to a system of congruences using the Chinese Remainder Theorem.
/// For a system of congruences:
/// x ≡ a1 (mod m1)
/// x ≡ a2 (mod m2)
/// ...
/// x ≡ an (mod mn)
/// Where m1, m2, ..., mn are pairwise coprime, the solution x is unique modulo M, where M = m1 * m2 * ... * mn.
/// # Arguments
/// * `congruences` - The congruences (tuples). Each tuple contains the (remainder, modulus). The slice should be sorted by modulus in descending order for the best performance.
/// # Returns
/// * `u64` - The solution to the system of congruences.
/// # Panics
/// If there are no congruences. There must be at least 1 congruence.
/// Note that this function will run infinitely if the moduli are not pairwise coprime.
pub fn chinese_remainder_theorem<T, U>(congruences: U) -> u64
where
    T: Borrow<(u64, u64)>,
    U: IntoIterator<Item = T>,
{
    let mut congruences = congruences.into_iter();
    let &(mut solution, mut modulus) = congruences
        .next()
        .expect("There must be at least 1 congruence.")
        .borrow();

    for congruence in congruences {
        let (remainder, modulo) = congruence.borrow();
        while solution % modulo != *remainder {
            solution += modulus;
        }
        modulus *= modulo;
    }

    solution
}

/// Finds the Manhattan distance between two locations.
/// Manhattan distance is the sum of the absolute differences of x and y coordinates.
/// # Arguments
/// * `loc1` - The first location as a tuple of (x, y).
/// * `loc2` - The second location as a tuple of (x, y).
/// # Returns
/// * `u64` - The Manhattan distance.
pub fn manhattan_distance(loc1: (i64, i64), loc2: (i64, i64)) -> u64 {
    loc1.0.abs_diff(loc2.0) + loc1.1.abs_diff(loc2.1)
}

/// Get the area of a polygon using the shoelace formula
pub fn shoelace(points: &[(i64, i64)]) -> f64 {
    let mut area = 0;

    for i in 0..(points.len() - 1) {
        area += points[i].0 * points[i + 1].1;
        area -= points[i].1 * points[i + 1].0;
    }
    area += points[points.len() - 1].0 * points[0].1;
    area -= points[points.len() - 1].1 * points[0].0;

    area = area.abs();

    area as f64 / 2.0
}

/// Get the number of boundary points of a polygon (points that are on the edge of the polygon)
pub fn boundary_points(points: &[(i64, i64)]) -> i64 {
    let mut points_count = points.len() as i64;

    for i in 0..(points.len() - 1) {
        // one of the coordinates is the same, so we can just add the difference of both coordinates
        // the result will be the distance between the two points
        // since we already counted all edge points, we just need to subtract 1
        // to get the number of points between the two points
        points_count += (points[i].0.abs_diff(points[i + 1].0)
            + points[i].1.abs_diff(points[i + 1].1)
            - 1) as i64;
    }
    points_count += (points[0].0.abs_diff(points[points.len() - 1].0)
        + points[0].1.abs_diff(points[points.len() - 1].1)
        - 1) as i64;

    points_count
}

/// Calculate the number of interior points of a polygon, using shoelace algorithm and pick's theorem
pub fn interior_points(points: &[(i64, i64)], bound_points: i64) -> i64 {
    let area = shoelace(points);

    // area = i + b/2 - 1
    // i = interior points
    // b = boundary points
    // i = area - b/2 + 1

    (area - (bound_points as f64) / 2.0 + 1.0) as i64
}
