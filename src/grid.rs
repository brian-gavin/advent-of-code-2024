use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Coord {
    row: isize,
    col: isize,
}

impl<T: Into<isize>> From<(T, T)> for Coord {
    fn from((row, col): (T, T)) -> Self {
        Self {
            row: row.into(),
            col: col.into(),
        }
    }
}

impl Coord {
    pub fn origin() -> Coord {
        Coord { row: 0, col: 0 }
    }

    pub fn north(self, n: isize) -> Coord {
        Coord {
            row: self.row - n,
            ..self
        }
    }

    pub fn east(self, n: isize) -> Coord {
        Coord {
            col: self.col + n,
            ..self
        }
    }

    pub fn south(self, n: isize) -> Coord {
        Coord {
            row: self.row + n,
            ..self
        }
    }

    pub fn west(self, n: isize) -> Coord {
        Coord {
            col: self.col - n,
            ..self
        }
    }

    pub fn northeast(self, n: isize) -> Coord {
        self.north(n).east(n)
    }

    pub fn southeast(self, n: isize) -> Coord {
        self.south(n).east(n)
    }

    pub fn southwest(self, n: isize) -> Coord {
        self.south(n).west(n)
    }

    pub fn northwest(self, n: isize) -> Coord {
        self.north(n).west(n)
    }
}

#[derive(Debug, Clone)]
pub struct Grid<V> {
    map: HashMap<Coord, V>,
}

impl<T> Grid<T> {
    pub fn at(&self, coord: Coord) -> Option<&T> {
        self.map.get(&coord)
    }

    pub fn at_mut(&mut self, coord: Coord) -> Option<&mut T> {
        self.map.get_mut(&coord)
    }

    pub fn insert(&mut self, coord: Coord, v: T) -> Option<T> {
        self.map.insert(coord, v)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Coord, &T)> {
        self.map.iter()
    }

    pub fn swap(&mut self, a: Coord, b: Coord) {
        assert_ne!(a, b);
        let ap = self.map.get_mut(&a).unwrap() as *mut _;
        let bp = self.map.get_mut(&b).unwrap() as *mut _;
        // SAFETY: should be fine
        unsafe {
            std::ptr::swap(ap, bp);
        };
    }
}

impl<V> FromIterator<(Coord, V)> for Grid<V> {
    fn from_iter<T: IntoIterator<Item = (Coord, V)>>(iter: T) -> Self {
        Grid {
            map: HashMap::from_iter(iter),
        }
    }
}
