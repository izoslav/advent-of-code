use std::fs;

use array2d::Array2D;

#[derive(Debug, Clone)]
struct Line {
    sx: usize,
    sy: usize,
    dx: usize,
    dy: usize,
}

impl Line {
    fn is_horizontal_or_vertical(&self) -> bool {
        self.sx == self.dx || self.sy == self.dy
    }

    fn is_diagonal(&self) -> bool {
        let x_diff = (self.dx as i32 - self.sx as i32).abs();
        let y_diff = (self.dy as i32 - self.sy as i32).abs();

        x_diff == y_diff
    }

    fn len(&self) -> usize {
        let x_diff = (self.dx as i32 - self.sx as i32).abs();
        let y_diff = (self.dy as i32 - self.sy as i32).abs();

        x_diff.max(y_diff) as usize + 1
    }
}

fn main() {
    let clouds = fs::read_to_string("res/day05.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let coords = l.split("->").collect::<Vec<&str>>();

            let start = coords[0]
                .trim()
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let (sx, sy) = (start[0], start[1]);

            let destination = coords[1]
                .trim()
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let (dx, dy) = (destination[0], destination[1]);

            Line { sx, dx, sy, dy }
        })
        .collect::<Vec<Line>>();

    {
        let clouds = clouds
            .clone()
            .into_iter()
            .filter(|cloud| cloud.is_horizontal_or_vertical())
            .collect::<Vec<Line>>();

        let (bbsx, bbsy, bbdx, bbdy) = bounding_box(&clouds);
        let clouds = normalize(&clouds);

        let mut ocean = Array2D::filled_with(0, bbdx - bbsx, bbdy - bbsy);

        for cloud in &clouds {
            draw(&mut ocean, cloud);
        }

        let intersections = ocean.elements_row_major_iter().filter(|&f| *f > 1).count();

        println!("Part 1 answer: {}", intersections);
    }

    {
        let clouds = clouds
            .into_iter()
            .filter(|cloud| cloud.is_horizontal_or_vertical() || cloud.is_diagonal())
            .collect::<Vec<Line>>();

        let (bbsx, bbsy, bbdx, bbdy) = bounding_box(&clouds);
        let clouds = normalize(&clouds);

        let mut ocean = Array2D::filled_with(0, bbdx - bbsx, bbdy - bbsy);

        for cloud in &clouds {
            draw(&mut ocean, cloud);
        }

        let intersections = ocean.elements_row_major_iter().filter(|&f| *f > 1).count();

        println!("Part 2 answer: {}", intersections);
    }
}

fn normalize(clouds: &[Line]) -> Vec<Line> {
    let (bbsx, bbsy, _, _) = bounding_box(clouds);

    clouds
        .iter()
        .map(|cloud| Line {
            sx: cloud.sx - bbsx,
            sy: cloud.sy - bbsy,
            dx: cloud.dx - bbsx,
            dy: cloud.dy - bbsy,
        })
        .collect::<Vec<Line>>()
}

fn bounding_box(lines: &[Line]) -> (usize, usize, usize, usize) {
    let min_x = lines.iter().map(|l| l.sx.min(l.dx)).min().unwrap();

    let min_y = lines.iter().map(|l| l.sy.min(l.dy)).min().unwrap();

    let max_x = lines.iter().map(|l| l.sx.max(l.dx)).max().unwrap();

    let max_y = lines.iter().map(|l| l.sy.max(l.dy)).max().unwrap();

    (min_x, min_y, max_x + 1, max_y + 1)
}

fn draw(ocean: &mut Array2D<usize>, cloud: &Line) {
    if cloud.is_horizontal_or_vertical() {
        if cloud.sx == cloud.dx {
            let sy = cloud.sy.min(cloud.dy);
            let dy = cloud.sy.max(cloud.dy);

            for y in sy..=dy {
                *ocean.get_mut(cloud.sx, y).unwrap() += 1;
            }
        } else {
            let sx = cloud.sx.min(cloud.dx);
            let dx = cloud.sx.max(cloud.dx);

            for x in sx..=dx {
                *ocean.get_mut(x, cloud.sy).unwrap() += 1;
            }
        }
    } else if cloud.is_diagonal() {
        let delta_x: i32 = if cloud.dx > cloud.sx { 1 } else { -1 };
        let delta_y: i32 = if cloud.dy > cloud.sy { 1 } else { -1 };

        for i in 0..cloud.len() {
            let x = (cloud.sx as i32 + i as i32 * delta_x) as usize;
            let y = (cloud.sy as i32 + i as i32 * delta_y) as usize;

            *ocean.get_mut(x, y).unwrap() += 1;
        }
    }
}
