use regex::Regex;

use std::cmp;

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct FabricClaim {
    pub id: i32,
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
}

impl FabricClaim {
    pub fn from_serialized(s: String) -> FabricClaim {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"#(?P<id>\d+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)x(?P<height>\d+)"
            )
            .unwrap();
        }
        let caps = RE.captures(s.as_str()).unwrap();

        FabricClaim {
            id: caps["id"].parse().unwrap(),
            top: caps["top"].parse().unwrap(),
            left: caps["left"].parse().unwrap(),
            width: caps["width"].parse().unwrap(),
            height: caps["height"].parse().unwrap(),
        }
    }

    #[inline]
    pub fn right(&self) -> i32 {
        return self.left + self.width;
    }

    #[inline]
    pub fn bottom(&self) -> i32 {
        return self.top + self.height;
    }

    pub fn intersects(&self, other: &FabricClaim) -> bool {
        self.left < other.right()
            && self.right() > other.left
            && self.top < other.bottom()
            && self.bottom() > other.top
    }

    pub fn intersecting_points(&self, other: &FabricClaim) -> Option<Vec<Point>> {
        if !self.intersects(other) {
            None
        } else {
            let left = cmp::max(self.left, other.left);
            let top = cmp::max(self.top, other.top);
            let right = cmp::min(self.right(), other.right());
            let bottom = cmp::min(self.bottom(), other.bottom());

            let mut out = Vec::new();
            for i in left..right {
                for j in top..bottom {
                    out.push(Point { x: i, y: j })
                }
            }

            Some(out)
        }
    }
}
