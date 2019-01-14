#![no_main]
#![no_std]

use cortex_m_rt::entry;
use heapless::consts::U32;
use heapless::spsc::Queue;
use microbit::led::Display;
use nrf51::Peripherals;
use nrf51_hal::delay::Delay;
use nrf51_hal::prelude::*;
use panic_halt;

const GRID_SIZE: (u8, u8) = (5, 5);

struct Game {
    snake: Snake,
    // food: Food,
}

impl Game {
    fn new() -> Self {
        Game {
            snake: Snake::new(),
            // food: Food::new(),
        }
    }

    fn update(&mut self) {
        self.snake.slither();
    }

    fn draw(&self, leds: &mut Display, delay: &mut Delay) {
        let mut board = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ];
        board[self.snake.head.y as usize][self.snake.head.x as usize] = 1;
        for cell in self.snake.tail.iter() {
            board[cell.y as usize][cell.x as usize] = 1;
        }
        leds.display(delay, board, 200);
    }
}

struct Snake {
    head: Cell,
    tail: Queue<Cell, U32>,
    direction: Direction,
}

impl Snake {
    fn new() -> Self {
        let mut tail: Queue<Cell, U32> = Queue::new();
        let _ = tail.enqueue(Cell::new(0, 2));
        let _ = tail.enqueue(Cell::new(1, 2));
        Snake {
            head: Cell::new(2, 2),
            tail,
            direction: Direction::East,
        }
    }

    fn slither(&mut self) {
        let _ = self.tail.enqueue(self.head);
        self.head = self.head.with_direction(self.direction);
        let _ = self.tail.dequeue();
    }
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

// struct Food {
//     cell: Cell,
// }

// impl Food {
//     fn new() -> Self {
//         Food { cell: (0, 0) }
//     }
// }

#[derive(Copy, Clone)]
struct Cell {
    x: u8,
    y: u8,
}

impl Cell {
    fn new(x: u8, y: u8) -> Self {
        Cell {
            x: x % GRID_SIZE.0,
            y: y % GRID_SIZE.1,
        }
    }

    fn with_direction(self, direction: Direction) -> Self {
        match direction {
            Direction::North => Cell::new(self.x, self.y - 1),
            Direction::South => Cell::new(self.x, self.y + 1),
            Direction::East => Cell::new(self.x + 1, self.y),
            Direction::West => Cell::new(self.x - 1, self.y),
        }
    }
}

#[entry]
fn main() -> ! {
    if let Some(p) = Peripherals::take() {
        let mut delay = Delay::new(p.TIMER0);
        let gpio = p.GPIO.split();
        let col1 = gpio.pin4.into_push_pull_output();
        let col2 = gpio.pin5.into_push_pull_output();
        let col3 = gpio.pin6.into_push_pull_output();
        let col4 = gpio.pin7.into_push_pull_output();
        let col5 = gpio.pin8.into_push_pull_output();
        let col6 = gpio.pin9.into_push_pull_output();
        let col7 = gpio.pin10.into_push_pull_output();
        let col8 = gpio.pin11.into_push_pull_output();
        let col9 = gpio.pin12.into_push_pull_output();
        let row1 = gpio.pin13.into_push_pull_output();
        let row2 = gpio.pin14.into_push_pull_output();
        let row3 = gpio.pin15.into_push_pull_output();
        let mut leds = Display::new(
            col1, col2, col3, col4, col5, col6, col7, col8, col9, row1, row2, row3,
        );

        let mut game = Game::new();

        loop {
            game.draw(&mut leds, &mut delay);
            game.update();
        }
    }
    panic!();
}
