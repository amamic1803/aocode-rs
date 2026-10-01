use crate::{Error, Solution};

day!(Day04, 2024, 4, "Ceres Search");

impl Solution for Day04 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let xmas_word = ['X', 'M', 'A', 'S'];

        let text = parse_input(input);
        let mut xmas_count = 0;

        for (i, row) in text.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c != xmas_word[0] {
                    continue;
                }

                let left_padding = j >= 3;
                let right_padding = j + 3 < row.len();
                let top_padding = i >= 3;
                let bottom_padding = i + 3 < text.len();

                // E
                if right_padding && check_xmas(|k| row[j + k] == xmas_word[k]) {
                    xmas_count += 1;
                }

                // W
                if left_padding && check_xmas(|k| row[j - k] == xmas_word[k]) {
                    xmas_count += 1;
                }

                // N
                if top_padding && check_xmas(|k| text[i - k][j] == xmas_word[k]) {
                    xmas_count += 1;
                }

                // S
                if bottom_padding && check_xmas(|k| text[i + k][j] == xmas_word[k]) {
                    xmas_count += 1;
                }

                // NE
                if top_padding
                    && right_padding
                    && check_xmas(|k| text[i - k][j + k] == xmas_word[k])
                {
                    xmas_count += 1;
                }

                // NW
                if top_padding && left_padding && check_xmas(|k| text[i - k][j - k] == xmas_word[k])
                {
                    xmas_count += 1;
                }

                // SE
                if bottom_padding
                    && right_padding
                    && check_xmas(|k| text[i + k][j + k] == xmas_word[k])
                {
                    xmas_count += 1;
                }

                // SW
                if bottom_padding
                    && left_padding
                    && check_xmas(|k| text[i + k][j - k] == xmas_word[k])
                {
                    xmas_count += 1;
                }
            }
        }

        Ok(xmas_count.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mas_word = [['M', '.', 'M'], ['.', 'A', '.'], ['S', '.', 'S']];

        let text = parse_input(input);
        let mut mas_count = 0;

        for (i, row) in text.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c != 'M' {
                    continue;
                }

                let left_padding = j >= 2;
                let right_padding = j + 2 < row.len();
                let top_padding = i >= 2;
                let bottom_padding = i + 2 < text.len();

                // SE
                if bottom_padding
                    && right_padding
                    && check_x_mas(|m, n| text[i + m][j + n] == mas_word[m][n])
                {
                    mas_count += 1;
                }

                // SW
                if bottom_padding
                    && left_padding
                    && check_x_mas(|m, n| text[i + n][j - m] == mas_word[m][n])
                {
                    mas_count += 1;
                }

                // NW
                if top_padding
                    && left_padding
                    && check_x_mas(|m, n| text[i - m][j - n] == mas_word[m][n])
                {
                    mas_count += 1;
                }

                // NE
                if top_padding
                    && right_padding
                    && check_x_mas(|m, n| text[i - n][j + m] == mas_word[m][n])
                {
                    mas_count += 1;
                }
            }
        }

        Ok(mas_count.to_string())
    }
}

fn check_xmas<T>(check_fn: T) -> bool
where
    T: Fn(usize) -> bool,
{
    check_fn(1) && check_fn(2) && check_fn(3)
}

fn check_x_mas<T>(check_fn: T) -> bool
where
    T: Fn(usize, usize) -> bool,
{
    check_fn(0, 2) && check_fn(1, 1) && check_fn(2, 0) && check_fn(2, 2)
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}
