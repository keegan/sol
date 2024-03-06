extern crate termion;

//use rand::Rng;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor, style, color};

use std::fmt;
use std::io::{self, Read, Write};

const ESC: char = '\x1b';

#[derive(Debug)]
enum Suite {
    Heart,
    Club,
    Diamond,
    Spade,
}

#[derive(Debug)]
struct Card {
    value: u8,
    suite: Suite,
    hidden: bool,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.hidden {
            write!(f, "{ESC}[92;42m| . . . |{ESC}[0m")
        } else {
            let value: char = match self.value {
                1 => 'A',
                val @ 2..=9 => (val + 48) as char,
                10 => 'X',
                11 => 'J',
                12 => 'Q',
                13 => 'K',
                _ => '_',
            };
            match self.suite {
                Suite::Heart => write!(f, "{ESC}[31;47m| {value}  <3 |{ESC}[0m"),
                Suite::Diamond => write!(f, "{ESC}[31;47m| {value}  <> |{ESC}[0m"),
                Suite::Club => write!(f, "{ESC}[30;47m| {value}  o8 |{ESC}[0m"),
                Suite::Spade => write!(f, "{ESC}[30;47m| {value}  3> |{ESC}[0m"),
            }
        }
    }
}

fn main() {
    let mut deck: [Card; 52] = [
        Card { hidden: true, value: 1, suite: Suite::Heart, },
        Card { hidden: true, value: 2, suite: Suite::Heart, },
        Card { hidden: true, value: 3, suite: Suite::Heart, },
        Card { hidden: true, value: 4, suite: Suite::Heart, },
        Card { hidden: true, value: 5, suite: Suite::Heart, },
        Card { hidden: true, value: 6, suite: Suite::Heart, },
        Card { hidden: true, value: 7, suite: Suite::Heart, },
        Card { hidden: true, value: 8, suite: Suite::Heart, },
        Card { hidden: true, value: 9, suite: Suite::Heart, },
        Card { hidden: true, value: 10, suite: Suite::Heart, },
        Card { hidden: true, value: 11, suite: Suite::Heart, },
        Card { hidden: true, value: 12, suite: Suite::Heart, },
        Card { hidden: true, value: 13, suite: Suite::Heart, },

        Card { hidden: true, value: 1, suite: Suite::Diamond, },
        Card { hidden: true, value: 2, suite: Suite::Diamond, },
        Card { hidden: true, value: 3, suite: Suite::Diamond, },
        Card { hidden: true, value: 4, suite: Suite::Diamond, },
        Card { hidden: true, value: 5, suite: Suite::Diamond, },
        Card { hidden: true, value: 6, suite: Suite::Diamond, },
        Card { hidden: true, value: 7, suite: Suite::Diamond, },
        Card { hidden: true, value: 8, suite: Suite::Diamond, },
        Card { hidden: true, value: 9, suite: Suite::Diamond, },
        Card { hidden: true, value: 10, suite: Suite::Diamond, },
        Card { hidden: true, value: 11, suite: Suite::Diamond, },
        Card { hidden: true, value: 12, suite: Suite::Diamond, },
        Card { hidden: true, value: 13, suite: Suite::Diamond, },

        Card { hidden: true, value: 1, suite: Suite::Club, },
        Card { hidden: true, value: 2, suite: Suite::Club, },
        Card { hidden: true, value: 3, suite: Suite::Club, },
        Card { hidden: true, value: 4, suite: Suite::Club, },
        Card { hidden: true, value: 5, suite: Suite::Club, },
        Card { hidden: true, value: 6, suite: Suite::Club, },
        Card { hidden: true, value: 7, suite: Suite::Club, },
        Card { hidden: true, value: 8, suite: Suite::Club, },
        Card { hidden: true, value: 9, suite: Suite::Club, },
        Card { hidden: true, value: 10, suite: Suite::Club, },
        Card { hidden: true, value: 11, suite: Suite::Club, },
        Card { hidden: true, value: 12, suite: Suite::Club, },
        Card { hidden: true, value: 13, suite: Suite::Club, },

        Card { hidden: true, value: 1, suite: Suite::Spade, },
        Card { hidden: true, value: 2, suite: Suite::Spade, },
        Card { hidden: true, value: 3, suite: Suite::Spade, },
        Card { hidden: true, value: 4, suite: Suite::Spade, },
        Card { hidden: true, value: 5, suite: Suite::Spade, },
        Card { hidden: true, value: 6, suite: Suite::Spade, },
        Card { hidden: true, value: 7, suite: Suite::Spade, },
        Card { hidden: true, value: 8, suite: Suite::Spade, },
        Card { hidden: true, value: 9, suite: Suite::Spade, },
        Card { hidden: true, value: 10, suite: Suite::Spade, },
        Card { hidden: true, value: 11, suite: Suite::Spade, },
        Card { hidden: true, value: 12, suite: Suite::Spade, },
        Card { hidden: true, value: 13, suite: Suite::Spade, },
    ];

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut stdout = stdout.into_raw_mode().unwrap();
    let termsize = termion::terminal_size().ok();
    let termwidth = termsize.map(|(w,_)| w - 2).unwrap();
    let termheight = termsize.map(|(_,h)| h - 2).unwrap();


    for card in deck.iter() {
        print!("{card}");
    }

    let mut board: [Vec<usize>; 7] = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new()
    ];

    let mut counter = 0;

    for i in (0..7){
        let mut col = &mut board[i];
        for j in (0..=i){
            col.push(counter);
            counter += 1;
        }
        if col.len() > 0 {
            deck[col[col.len() - 1]].hidden = false;
        }
    }


    let mut game = Game {
        width: termwidth,
        stdout: Box::new(stdout),
        stdin: Box::new(stdin),

        deck: deck,
        board: board,
    };

    game.show();
}

/// The game state.
struct Game{
    width: u16,

    stdout: Box<dyn Write>,
    stdin: Box<dyn Read>,

    deck: [Card; 52],
    board: [Vec<usize>; 7],
}

impl Game {
    fn show(&mut self){
        write!(self.stdout, "{}{}{}", clear::All, style::Reset, cursor::Goto(1, 1));
        for (i, col) in self.board.iter().enumerate() {
            for (j, card) in col.iter().enumerate() {
                write!(self.stdout, "{1}{0}", self.deck[*card as usize], cursor::Goto(1 + (i as u16 * 9), 2 + j as u16));
            }
        }
        //write!(self.stdout, "{}.{:?}", cursor::Goto(1, 60), self.board);
    }
}

