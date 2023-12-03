use std::{fs, str::Chars};

#[derive(Clone, Debug)]
enum Snumber {
  Regular { value: usize, },
  Pair { left: Box<Snumber>, right: Box<Snumber>, }
}

impl Snumber {
  fn from(iter: &mut Chars) -> Self {
    let c = iter.next().unwrap();

    match c {
      '[' => {
        let s1 = Self::from(iter);
        iter.next(); // skip ,
        let s2 = Self::from(iter);
        iter.next(); // skip ]

        Snumber::Pair { left: Box::new(s1), right: Box::new(s2), }
      },
      c if ('0'..='9').contains(&c) => Snumber::Regular { value: c.to_string().parse::<usize>().unwrap() },
      _ => panic!()
    }
  }

  fn magnitude(&self) -> usize {
    match self {
      Self::Regular { value } => *value,
      Self::Pair { left, right } => 3 * left.magnitude() + 2 * right.magnitude(),
    }
  }

  fn explode(&mut self, nest_level: usize, left: Option<usize>, right: Option<usize>) {
    todo!();
    
    *self = Self::Regular { value: 0 };
  }

  fn split(&mut self) {
    if let Self::Regular { value } = self {
      if *value > 9 {
        *self = Self::Pair {
          left: Box::new(Self::Regular { value: *value / 2 }),
          right: Box::new(Self::Regular { value: (*value + 1) / 2 }),
        }
      }
    }
  }

  fn reduce(&mut self, root: Option<&mut Self>) {
    let mut root = if let Some(root) = root { root } else { self };

    // match self {
    //   Self::Regular => self.split(),
    //   Self::Pair => self.explode(),
    // }

    todo!()
  }

  fn find_left_regular(&mut self) -> &mut Self {
    self
  }

  fn find_right_regular(&mut self) -> &mut Self {
    self
  }
}

fn main() {
  let input = fs::read_to_string("res/day18.txt")
    .unwrap();

  let mut iter = input.chars();

  let snumber = Snumber::from(&mut iter);
  println!("Snumber: {:?}", snumber);
  println!("Smagnitude: {}", snumber.magnitude());
}
