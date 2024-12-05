use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

pub struct Grid<V> {
    map: HashMap<Coord, V>,
}

impl<T> Grid<T> {
    pub fn at(&self, coord: Coord) -> Option<&T> {
        self.map.get(&coord)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Coord, &T)> {
        self.map.iter()
    }
}

impl<V> FromIterator<(Coord, V)> for Grid<V> {
    fn from_iter<T: IntoIterator<Item = (Coord, V)>>(iter: T) -> Self {
        Grid {
            map: HashMap::from_iter(iter),
        }
    }
}
