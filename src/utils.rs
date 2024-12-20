pub fn input(day: u16, example: bool) -> anyhow::Result<String> {
    let mut fname = format!("inputs/{day:02}");
    if example {
        fname.push('e');
    }
    fname.push_str(".txt");
    Ok(std::fs::read_to_string(&fname)?)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Hash, PartialOrd, Ord)]
pub struct Vec2 {
    pub x: isize,
    pub y: isize,
}

impl Vec2 {
    pub fn neighbors(&self) -> [Vec2; 4] {
        [self.up(), self.right(), self.down(), self.left()]
    }

    pub fn manhattan_distance(&self, other: &Vec2) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn right(&self) -> Vec2 {
        Vec2 {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn left(&self) -> Vec2 {
        Vec2 {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn up(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn down(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y + 1,
        }
    }
}

impl From<(isize, isize)> for Vec2 {
    fn from(value: (isize, isize)) -> Self {
        Vec2 {
            x: value.0,
            y: value.1,
        }
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Dir {
    North,
    South,
    East,
    West,
}
