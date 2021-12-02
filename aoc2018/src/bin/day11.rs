use array2d::Array2D;

fn main() {
    let width = 300;
    let height = 300;
    let serial = 9005;

    let mut grid = Array2D::filled_with(0, width, height);

    for x in 0..width {
        for y in 0..height {
            *grid.get_mut(x, y).unwrap() = calculate_power(x, y, serial);
        }
    }

    let (max_x, max_y, _, _) = max_power(&grid, 3, 3);
    println!("Part 1 answer: {},{}", max_x, max_y);

    let (max_x, max_y, max_size, _) = max_power(&grid, 1, 25);
    println!("Part 2 answer: {},{},{}", max_x, max_y, max_size);
}

fn calculate_power(x: usize, y: usize, serial: i32) -> i32 {
    let rack_id = x as i32 + 10;
    let power_level = (((rack_id * y as i32 + serial) * rack_id) % 1000) / 100 - 5;

    power_level
}

fn max_power(
    grid: &Array2D<i32>,
    min_size: usize,
    size_limit: usize,
) -> (usize, usize, usize, i32) {
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;
    let mut max_size = usize::MIN;
    let mut max_power = i32::MIN;

    for size in min_size..size_limit + 1 {
        for x in 0..grid.column_len() - size + 1 {
            for y in 0..grid.row_len() - size + 1 {
                let mut sum = 0;

                for i in 0..size {
                    for j in 0..size {
                        sum += grid.get(x + i, y + j).unwrap();
                    }
                }

                if sum > max_power {
                    max_x = x;
                    max_y = y;
                    max_size = size;
                    max_power = sum;
                }
            }
        }
    }

    (max_x, max_y, max_size, max_power)
}
