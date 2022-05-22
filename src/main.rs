#![allow(unused_imports)]

use rand::{thread_rng, Rng};
use termion::{event::{Key, Event}, input::TermRead, cursor};
use std::{io::{Write, stdout, stdin}, process::Stdio};
use std::thread::sleep;
use std::time::Duration;
use num_traits as num;

#[derive(Default, Clone, Copy)]
struct Coord {
    x: u16,
    y: u16
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Dead,
    Alive
}

struct Board {
    cells: Vec<Cell>,
    width: isize,
    height: isize
}

impl Board {
    fn new(width: usize, height: usize) -> Self {
        Board {
            cells: vec![Cell::Dead; (width*height) as usize],
            width: width as isize,
            height: height as isize
        }
    }

    fn from_board(board: &Board) -> Self {
        Board {
            cells: board.cells.clone(),
            width: board.width,
            height: board.height
        }
    }

    fn at(&self, x: isize, y: isize) -> Cell {
        if !(0..self.width).contains(&x) || !(0..self.height).contains(&y) {
            Cell::Dead
        }
        else {
            *self.cells.get(usize::try_from(y*self.width + x).unwrap()).unwrap()
        }
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.cells[y*self.width as usize + x]
    }

    fn set(&mut self, x: usize, y: usize, value: Cell) {
        *self.get_mut(x, y) = value;
    }
}

fn update(board: &Board, output: &mut Board) {
    if board.width != output.width || board.height != output.height {
        panic!("Input and output board sizes are not the same!");
    }

    for y in 0..board.height {
        for x in 0..board.width {
            let mut neighbours = 0;

            let mut check_neighbours = |px: isize, py: isize| {
                if let Cell::Alive = board.at(px, py) {
                    neighbours += 1;
                }
            };

            for px in (x-1)..=(x+1) {
                for py in (y-1)..=(y+1) {
                    if px == x && py == y { continue };

                    check_neighbours(px, py);
                }
            }

            let mut write = |value: Cell| {
                output.set(x as usize, y as usize, value);
            };

            match board.at(x, y) {
                Cell::Alive => match neighbours {
                    2..=3 => write(Cell::Alive),
                    _ => write(Cell::Dead)
                },
                Cell::Dead => match neighbours {
                    3 => write(Cell::Alive),
                    _ => write(Cell::Dead)
                }
            };
        }
    }
}

fn render(board: &Board, at: Coord) {
    for y in 0..board.height {
        print!("{}", cursor::Goto(at.x, at.y + (y as u16)));

        for x in 0..board.width {
            print!("{}{}", if let Cell::Alive = board.at(x, y) { "x" } else { "`" }, cursor::Right(1));
        }
    }

    stdout().flush().unwrap();
}

fn clear_box(topleft: Coord, bottomright: Coord) {
    print!("{}", termion::clear::All);

    for y in (topleft.y+1)..bottomright.y {
        print!("{}│{}│", cursor::Goto(topleft.x, y), cursor::Goto(bottomright.x, y));
    }

    for x in (topleft.x+1)..bottomright.x {
        print!("{}─{}─", cursor::Goto(x, topleft.y), cursor::Goto(x, bottomright.y));
    }

    println!("{}┌{}┐{}└{}┘", 
        cursor::Goto(topleft.x, topleft.y),
        cursor::Goto(bottomright.x, topleft.y),
        cursor::Goto(topleft.x, bottomright.y),
        cursor::Goto(bottomright.x, bottomright.y)
    );

    // stdout().flush().unwrap();
}

fn main() {
    // o x o o
    // o x x x
    // o o o o
    // o o o o
    
    let mut board = Board::new(40, 40);

    clear_box(Coord { x: 1, y: 1 }, Coord { x: board.width as u16 * 2 + 2, y: board.height as u16 + 2 });

    for cell in &mut board.cells {
        *cell = if thread_rng().gen_bool(0.2) { Cell::Alive } else { Cell::Dead };
    }

    let mut buffer = Board::from_board(&board);

    let mut board_ptr = &mut board;
    let mut buffer_ptr = &mut buffer;

    let mut _ticks: usize = 0;
    loop {
        render(board_ptr, Coord { x: 2, y: 2 });

        update(board_ptr, buffer_ptr);

        std::mem::swap(&mut board_ptr, &mut buffer_ptr);

        sleep(Duration::from_millis(200));
        _ticks += 200;
    }
}
