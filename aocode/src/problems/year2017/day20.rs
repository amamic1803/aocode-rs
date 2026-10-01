use crate::{Error, Solution};

day!(Day20, 2017, 20, "Particle Swarm");

impl Solution for Day20 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        // comparing particles by their (manhattan distance) acceleration, velocity, and position
        // as t goes to infinity, the particle with the lowest acceleration will be the closest to the origin
        // if there are multiple particles with the same acceleration, the one with the lowest velocity will be the closest
        // if there are multiple particles with the same acceleration and velocity, the one with the lowest position will be the closest
        // sort the particles by acceleration, velocity, and position, and return the index of the first particle

        let mut particles = input
            .lines()
            .map(Particle::new)
            .enumerate()
            .collect::<Vec<_>>();
        particles.sort_by_key(|(_, particle)| {
            (
                particle.acceleration[0].abs()
                    + particle.acceleration[1].abs()
                    + particle.acceleration[2].abs(),
                particle.velocity[0].abs()
                    + particle.velocity[1].abs()
                    + particle.velocity[2].abs(),
                particle.position[0].abs()
                    + particle.position[1].abs()
                    + particle.position[2].abs(),
            )
        });
        Ok(particles[0].0.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let particles = input.lines().map(Particle::new).collect::<Vec<_>>();

        // create collision table
        // axes are particles, and cells are the time at which the particles collide
        // None means ignore (destroyed particle), Some(None) means no collision, Some(Some(x)) means collision at time x
        // note that the collision table is symmetric
        let mut collision_table = vec![vec![Some(None); particles.len()]; particles.len()];

        // populate table with collision times
        for i in 0..particles.len() {
            for j in (i + 1)..particles.len() {
                if i != j {
                    collision_table[i][j] = Some(particles[i].collision(&particles[j]));
                    collision_table[j][i] = collision_table[i][j]; // symmetric
                }
            }
        }

        let mut collisions = Vec::new(); // temporary variable to store indices of particles that collide at the same time
        loop {
            // time of the next collision
            let mut min_time = None;

            // find the minimum time in the collision table
            for (i, row) in collision_table.iter().enumerate() {
                for time in row.iter().skip(i + 1).flatten().flatten() {
                    if min_time.is_none() || time < min_time.unwrap() {
                        min_time = Some(time);
                    }
                }
            }

            // if there are no more collisions, break
            if min_time.is_none() {
                break;
            }

            // store indices of particles that collide at the min_time
            collisions.clear();
            for (i, row) in collision_table.iter().enumerate() {
                if row.iter().any(|x| match x {
                    Some(Some(x)) => x == min_time.unwrap(),
                    _ => false,
                }) {
                    collisions.push(i);
                }
            }

            // fill rows and columns of the collision table with None for the particles that collide
            for &i in &collisions {
                collision_table[i].fill(None);
                for row in &mut collision_table {
                    row[i] = None;
                }
            }
        }

        // find surviving particles (rows with at least one Some)
        Ok(collision_table
            .into_iter()
            .filter(|particle| particle.iter().any(|x| x.is_some()))
            .count()
            .to_string())
    }
}

/// A particle with position, velocity, and acceleration
struct Particle {
    position: [i32; 3],
    velocity: [i32; 3],
    acceleration: [i32; 3],
}
impl Particle {
    /// Creates a new particle from a string
    fn new(input: &str) -> Self {
        let mut parts = input.split(", ");
        let mut position = parts
            .next()
            .unwrap()
            .trim_start_matches("p=<")
            .trim_end_matches('>')
            .split(',')
            .map(|x| x.parse::<i32>().unwrap());
        let mut velocity = parts
            .next()
            .unwrap()
            .trim_start_matches("v=<")
            .trim_end_matches('>')
            .split(',')
            .map(|x| x.parse::<i32>().unwrap());
        let mut acceleration = parts
            .next()
            .unwrap()
            .trim_start_matches("a=<")
            .trim_end_matches('>')
            .split(',')
            .map(|x| x.parse::<i32>().unwrap());

        Self {
            position: [
                position.next().unwrap(),
                position.next().unwrap(),
                position.next().unwrap(),
            ],
            velocity: [
                velocity.next().unwrap(),
                velocity.next().unwrap(),
                velocity.next().unwrap(),
            ],
            acceleration: [
                acceleration.next().unwrap(),
                acceleration.next().unwrap(),
                acceleration.next().unwrap(),
            ],
        }
    }

    /// Find if and when this particle collides with another particle
    fn collision(&self, other: &Self) -> Option<u16> {
        // solve the quadratic equation for each axis to find the collision time

        // solutions for each axis (None=solution for any time, Some([t1, t2])=solutions for a specific time)
        let mut collision_times = [Some([-1.0; 2]); 3];

        for i in 0..3 {
            // a = (a1 - a2) / 2
            // b = (v1 - v2) + a
            // c = p1 - p2

            let a = (self.acceleration[i] - other.acceleration[i]) as f32 / 2.0;
            let b = (self.velocity[i] - other.velocity[i]) as f32 + a;
            let c = (self.position[i] - other.position[i]) as f32;

            let d = b * b - 4.0 * a * c;

            // no solution, no collision
            if d < 0.0 {
                return None;
            }

            if a != 0.0 {
                // quadratic equation
                if let Some(collision_times) = collision_times.get_mut(i).unwrap() {
                    collision_times[0] = (-b + d.sqrt()) / (2.0 * a);
                    collision_times[1] = (-b - d.sqrt()) / (2.0 * a);
                }
            } else if b != 0.0 {
                // linear equation
                if let Some(collision_times) = collision_times.get_mut(i).unwrap() {
                    collision_times[0] = -c / b; // singular solution
                    // other value is already -1.0 and will be ignored
                }
            } else if c == 0.0 {
                // no acceleration, no velocity, no position difference, collision at any time
                collision_times[i] = None;
            } else {
                // c != 0, but c = 0 (in quadratic equation), never collides
                return None;
            }
        }

        // find the minimum time that all axes collide
        let mut min_time = None;

        // find first non None axis (if all are None, particles are equal, but we ignore that case)
        let base_pos = collision_times.iter().position(|x| x.is_some())?;

        // check solutions in base and see if other axes contain the same solution
        for t in collision_times[base_pos].unwrap() {
            // only consider integer times greater than 0
            if t > 0.0 && (t.round() - t).abs() < 1e-10 {
                // check if t is the solution for all axes
                let mut other_axes_sol = true;

                // for every non None axis, check if none of the solutions match t
                for axis in collision_times.iter().skip(base_pos).flatten() {
                    if axis.iter().all(|x| (x - t).abs() > 1e-10) {
                        other_axes_sol = false;
                        break;
                    }
                }

                // if t is the solution, update min_time
                if other_axes_sol {
                    min_time = Some(t.min(min_time.unwrap_or(f32::MAX)));
                }
            }
        }

        // return min_time as u16
        min_time.map(|x| x.round() as u16)
    }
}
