use std::{fs, collections::HashSet};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cuboid {
  x: (i32, i32),
  y: (i32, i32),
  z: (i32, i32),
}

//
// z
// ^
// | , y
// |/
// +-->x
//

impl Cuboid {
  fn new(sx: i32, dx: i32, sy: i32, dy: i32, sz: i32, dz: i32) -> Self {
    Cuboid { x: (sx, dx), y: (sy, dy), z: (sz, dz) }
  }

  fn is_valid(&self) -> bool {
    self.x.0 <= self.x.1 && self.y.0 <= self.y.1 && self.z.0 <= self.z.1
  }

  fn points(&self) -> usize {
    let w = (1 + self.x.1 - self.x.0) as usize;
    let h = (1 + self.y.1 - self.y.0) as usize;
    let d = (1 + self.z.1 - self.z.0) as usize;

    w * h * d
  }

  fn intersects(&self, other: &Self) -> bool {
    (self.x.0 <= other.x.1 && self.x.1 >= other.x.0) &&
    (self.y.0 <= other.y.1 && self.y.1 >= other.y.0) &&
    (self.z.0 <= other.z.1 && self.z.1 >= other.z.0)
  }

  fn diff(&self, other: &Self) -> Vec<Self> {
    vec![
      // top
      Self::new(self.x.0, self.x.1, self.y.0, self.y.1, other.z.1 + 1, self.z.1),
      // bottom
      Self::new(self.x.0, self.x.1, self.y.0, self.y.1, self.z.0, other.z.0 - 1),
      // front
      Self::new(self.x.0, self.x.1, self.y.0, other.y.0 - 1, other.z.0.max(self.z.0), other.z.1.min(self.z.1)),
      // back
      Self::new(self.x.0, self.x.1, other.y.1 + 1, self.y.1, other.z.0.max(self.z.0), other.z.1.min(self.z.1)),
      // left
      Self::new(self.x.0, other.x.0 - 1, other.y.0.max(self.y.0), other.y.1.min(self.y.1), other.z.0.max(self.z.0), other.z.1.min(self.z.1)),
      // right
      Self::new(other.x.1 + 1, self.x.1, other.y.0.max(self.y.0), other.y.1.min(self.y.1), other.z.0.max(self.z.0), other.z.1.min(self.z.1)),
    ]
    .into_iter()
    .filter(|c| c.is_valid())
    .collect_vec()
  }
}

fn main() {
  let steps = fs::read_to_string("res/day22.txt")
    .unwrap()
    .lines()
    .map(|line| {
      let on = &line[0..2] == "on";
      let ranges = &line[3..]
        .trim()
        .split(',')
        .map(|s| s[2..]
          .split("..")
          .map(|n| n.parse::<i32>().unwrap())
          .collect_vec()
        )
        .collect_vec();

      (
        on,
        (ranges[0][0], ranges[0][1]),
        (ranges[1][0], ranges[1][1]),
        (ranges[2][0], ranges[2][1])
      )
    })
    .collect_vec();

  let mut cuboids = HashSet::<Cuboid>::new();

  // for (on, x, y, z) in steps {
  //   let new_cuboid = Cuboid::new(x.0, x.1, y.0, y.1, z.0, z.1);
    
  //   let to_modify = cuboids
  //     .iter()
  //     .filter(|c| c.intersects(&new_cuboid))
  //     .map(|c| c.clone())
  //     .collect_vec();

  //   if on {
  //     if to_modify.len() == 0 {
  //       cuboids.insert(new_cuboid);
  //     }

  //     to_modify
  //       .into_iter()
  //       .for_each(|c| {
  //         new_cuboid.diff(&c)
  //           .into_iter()
  //           .for_each(|nc| {
  //             cuboids.insert(nc);
  //           });
  //       });
  //   } else {
  //     to_modify
  //       .into_iter()
  //       .for_each(|c| {
  //         cuboids.remove(&c);

  //         c.diff(&new_cuboid)
  //           .into_iter()
  //           .for_each(|nc| {
  //             cuboids.insert(nc);
  //           });
  //       });
  //   }
  // }

  // loop {
  //   let mut collisions = false;

  //   let mut to_remove = vec![];
  //   let mut to_add: Vec<Cuboid> = vec![];

  //   println!("cuboids.len() = {}", cuboids.len());

  //   let mut n = 0;
  //   for c1 in &cuboids {
  //     for c2 in &cuboids {
  //       if c1 != c2 && c1.intersects(c2) && !to_remove.contains(c2) {
  //         to_remove.push(*c2);
  //         to_add.extend(c1.diff(c2).iter());
  //         collisions = true;
  //         n += 1;
  //         println!("n = {}", n);
  //       }
  //     }
  //   }

  //   println!("to_remove: {:?}", to_remove);
  //   println!("to_add: {:?}", to_add);

  //   to_remove
  //     .iter()
  //     .for_each(|r| { cuboids.remove(r); });

  //   to_add
  //     .iter()
  //     .for_each(|n| { cuboids.insert(*n); });

  //   if !collisions {
  //     break;
  //   }
  // }

  // on x=10..12,y=10..12,z=10..12
  // on x=11..13,y=11..13,z=11..13
  add_cuboid(&mut cuboids, Cuboid::new(10, 12, 10, 12, 10, 12));
  add_cuboid(&mut cuboids, Cuboid::new(11, 13, 11, 13, 11, 13));

  let lights = cuboids.iter().map(|c| c.points()).sum::<usize>();
  println!("Part 1 answer: {}", lights);
}

fn add_cuboid(cuboids: &mut HashSet<Cuboid>, new: Cuboid) {
  let collisions = cuboids
    .iter()
    .filter(|&c| new.intersects(c))
    .map(|c| c.clone())
    .collect_vec();

  println!("cuboids: {:?}", cuboids);
  println!("collisions: {:?}", collisions);
  println!("new: {:?}", new);

  if collisions.len() == 0 {
    cuboids.insert(new);
  } else {
    let new_cuboids = collisions
      .into_iter()
      .map(|c| c.diff(&new))
      .flatten()
      .for_each(|nc| add_cuboid(cuboids, nc));
      // .collect_vec();

    println!("new_cuboids: {:?}", new_cuboids);

    // collisions
    //   .into_iter()
    //   .for_each(|c| {
    //     c.diff(&new)
    //       .iter()
    //       .for_each(|nc| add_cuboid(cuboids, *nc));
    //   });
  }
}
