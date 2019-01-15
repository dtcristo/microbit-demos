#![no_main]
#![no_std]

use cortex_m_rt::entry;
use heapless::consts::U32;
use heapless::spsc::Queue;
use microbit::led::Display;
use nrf51::{Peripherals, GPIOTE};
use nrf51_hal::delay::Delay;
use nrf51_hal::prelude::*;
use panic_halt;

const GRID_SIZE: (u8, u8) = (5, 5);
const TICK_RATE_MS: u32 = 500;

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

    fn update(&mut self, gpiote: &GPIOTE) {
        let a_pressed = gpiote.events_in[0].read().bits() != 0;
        let b_pressed = gpiote.events_in[1].read().bits() != 0;
        gpiote.events_in[0].write(|w| unsafe { w.bits(0) });
        gpiote.events_in[1].write(|w| unsafe { w.bits(0) });
        if let Some(turn) = match (a_pressed, b_pressed) {
            (false, false) => None,
            (true, false) => Some(Turn::Left),
            (false, true) => Some(Turn::Right),
            (true, true) => None,
        } {
            self.snake.turn(turn);
        };
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
        leds.display(delay, board, TICK_RATE_MS);
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

    fn turn(&mut self, turn: Turn) {
        self.direction = self.direction.with_turn(turn);
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

impl Direction {
    fn with_turn(self, turn: Turn) -> Self {
        match self {
            Direction::North => match turn {
                Turn::Left => Direction::West,
                Turn::Right => Direction::East,
            },
            Direction::South => match turn {
                Turn::Left => Direction::East,
                Turn::Right => Direction::West,
            },
            Direction::East => match turn {
                Turn::Left => Direction::North,
                Turn::Right => Direction::South,
            },
            Direction::West => match turn {
                Turn::Left => Direction::South,
                Turn::Right => Direction::North,
            },
        }
    }
}

#[derive(Copy, Clone)]
enum Turn {
    Left,
    Right,
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
    fn new(x: i8, y: i8) -> Self {
        let x_max = GRID_SIZE.0 as i8;
        let y_max = GRID_SIZE.1 as i8;
        Cell {
            x: ((x % x_max + x_max) % x_max) as u8,
            y: ((y % y_max + y_max) % y_max) as u8,
        }
    }

    fn with_direction(self, direction: Direction) -> Self {
        match direction {
            Direction::North => Cell::new(self.x as i8, (self.y - 1) as i8),
            Direction::South => Cell::new(self.x as i8, (self.y + 1) as i8),
            Direction::East => Cell::new((self.x + 1) as i8, self.y as i8),
            Direction::West => Cell::new((self.x - 1) as i8, self.y as i8),
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

        // Configure button A press events
        let _ = gpio.pin17.into_floating_input();
        p.GPIOTE.config[0]
            .write(|w| unsafe { w.mode().event().psel().bits(17).polarity().hi_to_lo() });
        p.GPIOTE.events_in[0].write(|w| unsafe { w.bits(0) });

        // Configure button B press events
        let _ = gpio.pin26.into_floating_input();
        p.GPIOTE.config[1]
            .write(|w| unsafe { w.mode().event().psel().bits(26).polarity().hi_to_lo() });
        p.GPIOTE.events_in[1].write(|w| unsafe { w.bits(0) });

        let mut game = Game::new();

        loop {
            game.draw(&mut leds, &mut delay);
            game.update(&p.GPIOTE);
        }
    }
    panic!();
}
