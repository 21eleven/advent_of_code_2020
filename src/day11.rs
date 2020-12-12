use std::fmt;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Tile {
    Empty,
    Unoccupied,
    Occupied,
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match *self {
            Tile::Occupied => write!(f, "#"),
            Tile::Unoccupied => write!(f, "L"),
            Tile::Empty => write!(f, "."),
        }
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Vec<Tile>> {
    // let input = "L.LL.LL.LL
// LLLLLLL.LL
// L.L.L..L..
// LLLL.LL.LL
// L.LL.LL.LL
// L.LLLLL.LL
// ..L.L.....
// LLLLLLLLLL
// L.LLLLLL.L
// L.LLLLL.LL";
    input.split("\n").map(|ln| {
        ln.chars().map(|c| {
            match c {
                'L' => Tile::Unoccupied,
                '#' => Tile::Occupied,
                _ => Tile::Empty
            }
        }).collect()
    }).collect()
}

fn all_unoccupied(i: usize, j: usize, mat: &Vec<Vec<Tile>>) -> bool {

    let mut unoccupied = true;
    let directions = [
        (-1,0), 
        (-1,1), 
        (0,1), 
        (1,1), 
        (1,0), 
        (1,-1),
        (0,-1),
        (-1,-1)
    ];
    for (di, dj) in directions.iter() {
        unoccupied &= !locally_occupied((i as i32)+di, (j as i32)+dj, mat);
    }
    unoccupied
}
fn n_locally_occupied(i: usize, j: usize, mat: &Vec<Vec<Tile>>) -> usize {

    let mut n = 0;
    let directions = [
        (-1,0), 
        (-1,1), 
        (0,1), 
        (1,1), 
        (1,0), 
        (1,-1),
        (0,-1),
        (-1,-1)
    ];
    for (di, dj) in directions.iter() {
        if locally_occupied((i as i32)+di, (j as i32)+dj, mat) {
            n += 1;
        }
    }
    n
}

fn locally_occupied(i: i32, j: i32, mat: &Vec<Vec<Tile>>) -> bool {
    let h = mat.len() as i32;
    let w = mat[0].len() as i32;
    if i < 0 || i >= h {
        return false
    }
    if j < 0 || j >= w {
        return false
    }
    match mat[i as usize][j as usize] {
        Tile::Occupied => true,
        _ => false
    }
}

fn count_occupied(seats: Vec<Vec<Tile>>) -> usize {
    seats.iter().map(|row|{
        row.iter().filter(|tile| **tile == Tile::Occupied).count()
    }).sum::<usize>()
}

#[aoc(day11, part1)]
pub fn p1(input: &[Vec<Tile>]) -> usize {
    let mut seats = vec![];
    for row in input.iter() {
        seats.push(row.clone());
    }
    let h = seats.len();
    let w = seats[0].len();
    loop {
        let mut next = vec![];
        for i in 0..h {
            let mut row = vec![];
            for j in 0..w {
                match seats[i][j] {
                    Tile::Unoccupied => {
                        if all_unoccupied(i, j, &seats) {
                            row.push(Tile::Occupied);
                        } else {
                            row.push(Tile::Unoccupied);
                        }
                    },
                    Tile::Occupied => {
                        if n_locally_occupied(i, j, &seats) >= 4 {
                           row.push(Tile::Unoccupied);
                        } else {
                            row.push(Tile::Occupied);
                        }
                    },
                    Tile::Empty => row.push(Tile::Empty),
                }
            }
            next.push(row)
        }
        if seats == next {
            break;
        } else {
            seats = next;
        }
    }
    count_occupied(seats)
}

fn all_diagonally_unoccupied(i: usize, j: usize, mat: &Vec<Vec<Tile>>) -> bool {

    let mut unoccupied = true;
    let directions = [
        (-1,0), 
        (-1,1), 
        (0,1), 
        (1,1), 
        (1,0), 
        (1,-1),
        (0,-1),
        (-1,-1)
    ];
    for (di, dj) in directions.iter() {
        unoccupied &= !diagonally_occupied(i as i32, j as i32, *di as i32, *dj as i32,  mat);
    }
    unoccupied
}
fn n_diagonally_occupied(i: usize, j: usize, mat: &Vec<Vec<Tile>>) -> usize {

    let mut n = 0;
    let directions = [
        (-1,0), 
        (-1,1), 
        (0,1), 
        (1,1), 
        (1,0), 
        (1,-1),
        (0,-1),
        (-1,-1)
    ];
    for (di, dj) in directions.iter() {
        if diagonally_occupied(i as i32, j as i32, *di as i32, *dj as i32,  mat) {
            n += 1;
        }
    }
    n
}

fn diagonally_occupied(i: i32, j: i32, di: i32, dj: i32, mat: &Vec<Vec<Tile>>) -> bool {
    let i = i + di;
    let j = j + dj;
    let h = mat.len() as i32;
    let w = mat[0].len() as i32;
    if i < 0 || i >= h {
        return false
    }
    if j < 0 || j >= w {
        return false
    }
    match mat[i as usize][j as usize] {
        Tile::Occupied => true,
        Tile::Unoccupied => false,
        Tile::Empty => diagonally_occupied(i,j,di,dj,mat)
    }
}

#[aoc(day11, part2)]
pub fn p2(input: &[Vec<Tile>]) -> usize {
    let mut seats = vec![];
    for row in input.iter() {
        seats.push(row.clone());
    }
    let h = seats.len();
    let w = seats[0].len();
    loop {
        let mut next = vec![];
        for i in 0..h {
            let mut row = vec![];
            for j in 0..w {
                match seats[i][j] {
                    Tile::Unoccupied => {
                        if all_diagonally_unoccupied(i, j, &seats) {
                            row.push(Tile::Occupied);
                        } else {
                            row.push(Tile::Unoccupied);
                        }
                    },
                    Tile::Occupied => {
                        if n_diagonally_occupied(i, j, &seats) >= 5 {
                           row.push(Tile::Unoccupied);
                        } else {
                            row.push(Tile::Occupied);
                        }
                    },
                    Tile::Empty => row.push(Tile::Empty),
                }
            }
            next.push(row)
        }
        if seats == next {
            break;
        } else {
            seats = next;
        }
    }
    count_occupied(seats)
}
