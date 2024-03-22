extern crate termion;

use rand::seq::SliceRandom;
use rand::thread_rng;

use std::collections::VecDeque;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor, style, color};
use termion::input::Keys;

use std::fmt;
use std::io::{self, Read, Write, StdinLock};

const ESC: char = '\x1b';
const BEL: char = '\x07';

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
    deck.shuffle(&mut thread_rng());

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut stdout = stdout.into_raw_mode().unwrap();
    let termsize = termion::terminal_size().ok();
    let termwidth = termsize.map(|(w,_)| w - 2).unwrap();
    let termheight = termsize.map(|(_,h)| h - 2).unwrap();


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

    for i in 0..7{
        let mut col = &mut board[i];
        for j in (0..=i){
            col.push(counter);
            counter += 1;
        }
        if col.len() > 0 {
            deck[col[col.len() - 1]].hidden = false;
        }
    }

    let mut drawpile: VecDeque<usize> = (counter..52).collect();
    deck[*drawpile.get(0).unwrap()].hidden = false;

    let mut game = Game {
        width: termwidth,
        stdout: Box::new(stdout),
        stdin: Box::new(stdin.keys()),

        deck: deck,
        board: board,

        draw: drawpile,
        foundation: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],

        cursor: 0,
        selected_card: None,
    };

    game.run();
}

/// The game state.
struct Game<'a>{
    width: u16,

    stdout: Box<dyn Write>,
    stdin: Box<Keys<StdinLock<'a>>>,

    deck: [Card; 52],
    board: [Vec<usize>; 7],
    draw: VecDeque<usize>,
    foundation: [Vec<usize>; 4],

    cursor: u8,
    selected_card: Option(usize),
}

impl Game<'_> {
    fn show(&mut self){
        write!(self.stdout, "{}{}{}{}{}", clear::All, style::Reset, cursor::Goto(1, 1), color::Fg(color::Reset), color::Bg(color::Reset));
        match self.draw.get(0) {
            None => write!(self.stdout, "{}{}|#x#x#x#|{}", color::Bg(color::Yellow), color::Fg(color::Black), style::Reset),
            Some(x) => write!(self.stdout, "{}", self.deck[*x]),
        };

        for (i, col) in self.foundation.iter().enumerate() {
            write!(self.stdout, "{}", cursor::Goto(28 + (i as u16 * 9), 1));
            match col.last() {
                None => write!(self.stdout, "{}{}|#x#x#x#|{}", color::Bg(color::Yellow), color::Fg(color::LightYellow), style::Reset),
                Some(x) => write!(self.stdout, "{}", self.deck[*x]),
            };
        }

        //write!(self.stdout, "{1}{0}", self.deck[*card as usize]);
        for (i, col) in self.board.iter().enumerate() {
            for (j, card) in col.iter().enumerate() {
                write!(self.stdout, "{1}{0}", self.deck[*card as usize], cursor::Goto(1 + (i as u16 * 9), 3+ j as u16));
            }
        }
        match self.cursor {
            card_col @ 0..=6 => write!(self.stdout, "{}", cursor::Goto(1 + (card_col as u16 * 9), 19)),
            foundation_col @ 7..=10 => write!(self.stdout, "{}", cursor::Goto(28 + ((foundation_col as u16 - 7)* 9), 2)),
            _ => write!(self.stdout, "{}", cursor::Goto(1, 2)),
        };
        write!(self.stdout, "{}{}|=======|", color::Fg(color::Blue), color::Bg(color::Black));
        write!(self.stdout, "{}{}{}{}", cursor::Goto(15, 1), color::Bg(color::Black), color::Fg(color::White), self.cursor);

        if self.selected_cursor <= 10 {
            write!(self.stdout, "{}{}{}{}", cursor::Goto(19, 1), color::Bg(color::Black), color::Fg(color::White), self.selected_cursor);
            match self.selected_cursor {
                card_col @ 0..=6 => write!(self.stdout, "{}", cursor::Goto(1 + (card_col as u16 * 9), 19)),
                foundation_col @ 7..=10 => write!(self.stdout, "{}", cursor::Goto(28 + ((foundation_col as u16 - 7)* 9), 2)),
                _ => Ok(()),
            };
            write!(self.stdout, "{}{}", color::Fg(color::Blue), color::Bg(color::White));
            if self.cursor == self.selected_cursor {
                write!(self.stdout, "|#=#=#=#|");
            } else {
                write!(self.stdout, "|=======|");
            }
        }

        write!(self.stdout, "{}{}{}{}", style::Reset, cursor::Goto(1, 1), color::Fg(color::Reset), color::Bg(color::Reset));
        self.stdout.flush().unwrap();
    }
    fn try_move(&mut self){
        if self.cursor == self.selected_cursor {
            write!(self.stdout, "{BEL}");
            write!(self.stdout, "{BEL}");
            return
        }
        let card_to_move = match self.selected_cursor {
            c_col @ 0..=7 => self.board[c_col].get(),
        };
        write!(self.stdout, "{BEL}");
    }
    fn run(&mut self){
        write!(self.stdout, "{}", cursor::Hide);
        loop {
            self.show();
            let b = self.stdin.next().unwrap().unwrap();
            use termion::event::Key::*;
            match b {
                Char('q') | Char('c') => {
                    write!(self.stdout, "{}{}", style::Reset, cursor::Show);
                    return
                },
                Char('h') | Char('a') | Left  => self.cursor = (self.cursor as i16 - 1).rem_euclid(11) as u8,
                Char('j') | Char('s') | Down  => self.cursor = match self.cursor {
                    0..=2 => 7,
                    card_col @ 3..=6 => card_col,
                    foundation_col @ 7..=10 => foundation_col - 4,
                    _ => 0,
                },
                Char('k') | Char('w') | Up    => self.cursor = match self.cursor {
                    0..=2 => 7,
                    card_col @ 3..=6 => card_col + 4,
                    foundation_col @ 7..=10 => foundation_col,
                    _ => 0,
                },
                Char('l') | Char('d') | Right => self.cursor = (self.cursor + 1).rem_euclid(11),
                Char(' ') => {
                    if self.selected_cursor > 10 {
                        self.selected_cursor = self.cursor;
                    } else {
                        self.try_move();
                        self.selected_cursor = 11;
                    }
                },
                _ => (),
            };
        }
    }
}

