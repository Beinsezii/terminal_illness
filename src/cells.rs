pub type Row = Vec<u8>;
pub type Grid = Vec<Row>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellOpts {
    pub corners: bool,
    pub life: u8,
    pub grow: [bool; 9],
    pub die: [bool; 9],
}

#[derive(Clone, Debug)]
pub struct Game {
    g1: Grid,
    g2: Grid,
    switch: bool,
    opts: CellOpts,
}

impl Game {
    pub fn new(opts: CellOpts) -> Self {
        Self {
            g1: Grid::new(),
            g2: Grid::new(),
            switch: false,
            opts,
        }
    }

    pub fn advance(&mut self) {
        if !self.switch {
            advance(&self.g1, &mut self.g2, self.opts)
        } else {
            advance(&self.g2, &mut self.g1, self.opts)
        }
        self.switch = !self.switch
    }

    pub fn resize(&mut self, x: usize, y: usize) {
        resize(&mut self.g1, x, y);
        resize(&mut self.g2, x, y);
    }

    pub fn grid<'a>(&'a self) -> &'a Grid {
        if !self.switch {
            &self.g1
        } else {
            &self.g2
        }
    }

    pub fn grid_mut<'a>(&'a mut self) -> &'a mut Grid {
        if !self.switch {
            &mut self.g1
        } else {
            &mut self.g2
        }
    }

    pub fn opts<'a>(&'a self) -> &'a CellOpts {
        &self.opts
    }

    pub fn set_cell(&mut self, x: usize, y: usize, val: u8) {
        if let Some(ptr) = self.grid_mut().get_mut(y).map(|r| r.get_mut(x)).flatten() {
            *ptr = val
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<u8> {
        get_cell(self.grid(), x, y)
    }
}

pub fn get_cell(grid: &Grid, x: usize, y: usize) -> Option<u8> {
    grid.get(y).map(|r| r.get(x)).flatten().cloned()
}

pub fn advance(from: &Grid, to: &mut Grid, opts: CellOpts) {
    // sanity checks. too many?
    assert_eq!(from.len(), to.len());
    assert_eq!(from.get(0).map(|r| r.len()), to.get(0).map(|r| r.len()));
    assert_eq!(from.last().map(|r| r.len()), to.last().map(|r| r.len()));

    for (y, row) in to.iter_mut().enumerate() {
        for (x, cell) in row.iter_mut().enumerate() {
            let (x, y) = (x as isize, y as isize);

            let polars = [
                (x, y - 1),     // up
                (x + 1, y),     // right
                (x, y + 1),     // down
                (x - 1, y),     // left
            ];

            let corners = [
                (x + 1, y - 1), // up right
                (x + 1, y + 1), // right down
                (x - 1, y + 1), // down left
                (x - 1, y - 1), // left up
            ];

            let iter = if opts.corners {
                polars.iter().chain(corners.iter())
            } else {
                polars.iter().chain([].iter())
            };

            let neighbors: usize = iter.filter_map(|(x, y)| {
                if x >= &0 && y >= &0 {
                    get_cell(from, *x as usize, *y as usize)
                } else {
                    None
                }
            })
            .filter(|c| *c != 0)
            .count();

            *cell = if opts.grow[neighbors] {
                (from[y as usize][x as usize] + 1).min(opts.life)
            } else if opts.die[neighbors] {
                from[y as usize][x as usize].saturating_sub(1)
            } else {
                from[y as usize][x as usize]
            }
        }
    }
}

pub fn resize(grid: &mut Grid, x: usize, y: usize) {
    if grid.len() < y {
        grid.extend((0..(y - grid.len())).map(|_| vec![0; x]))
    } else if grid.len() > y {
        *grid = grid[0..y].to_vec()
    }

    assert_eq!(grid.len(), y);

    for row in grid.iter_mut() {
        if row.len() < x {
            row.extend((0..(x - row.len())).map(|_| 0))
        } else if row.len() > x {
            *row = row[0..x].to_vec()
        }
        assert_eq!(row.len(), x);
    }
}
