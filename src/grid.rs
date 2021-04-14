use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Grid<T, const N: usize>(pub [[T; N]; N]);

pub type Coord = (usize, usize);

impl<T, const N: usize> Grid<T, N>
where
    T: Default + Copy,
{
    pub fn new() -> Self {
        Self([[T::default(); N]; N])
    }
}

impl<T, const N: usize> Grid<T, N> {
    pub fn neighbors((x, y): Coord) -> impl Iterator<Item = Coord> {
        let one_axis = |x| {
            if x > 0 { Some(x - 1) } else { None }
                .into_iter()
                .chain(if x + 1 < N { Some(x + 1) } else { None }.into_iter())
        };
        one_axis(x)
            .map(move |x| (x, y))
            .chain(one_axis(y).map(move |y| (x, y)))
    }
}

impl<T, const N: usize> Index<Coord> for Grid<T, N> {
    type Output = T;
    fn index(&self, (x, y): Coord) -> &Self::Output {
        &self.0[x][y]
    }
}

impl<T, const N: usize> IndexMut<Coord> for Grid<T, N> {
    fn index_mut(&mut self, (x, y): Coord) -> &mut Self::Output {
        &mut self.0[x][y]
    }
}
