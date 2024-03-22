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

#[derive(Debug, Eq, PartialEq)]
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


    let mut board: [VecDeque<usize>; 7] = [
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new()
    ];

    let mut counter = 0;

    for i in 0..7{
        let mut col = &mut board[i];
        for j in (0..=i){
            col.push_back(counter);
            counter += 1;
        }
        if col.len() > 0 {
            deck[*col.back().unwrap()].hidden = false;
        }
    }

    let mut drawpile: VecDeque<usize> = (counter..52).collect();
    deck[*drawpile.back().unwrap()].hidden = false;

    let mut game = Game {
        width: termwidth,
        stdout: Box::new(stdout),
        stdin: Box::new(stdin.keys()),

        deck: deck,
        board: board,

        draw: drawpile,
        foundation: [VecDeque::new(), VecDeque::new(), VecDeque::new(), VecDeque::new()],

        cursor: 0,
        cursor_y: 0,
        selected_card: None,
        card_selected_from_pos: 0,
        selected_cards: VecDeque::new(),
    };

    game.run();
}

/// The game state.
struct Game<'a>{
    width: u16,

    stdout: Box<dyn Write>,
    stdin: Box<Keys<StdinLock<'a>>>,

    deck: [Card; 52],
    board: [VecDeque<usize>; 7],
    draw: VecDeque<usize>,
    foundation: [VecDeque<usize>; 4],

    cursor: u8,
    cursor_y: u8,
    selected_card: Option<usize>,
    card_selected_from_pos: u8,
    selected_cards: VecDeque<usize>,
}

impl Game<'_> {
    fn show(&mut self){
        write!(self.stdout, "{}{}{}{}{}", clear::All, style::Reset, cursor::Goto(1, 1), color::Fg(color::Reset), color::Bg(color::Reset));
        match self.draw.back() {
            None => write!(self.stdout, "{}{}|#x#x#x#|{}", color::Bg(color::Yellow), color::Fg(color::Black), style::Reset),
            Some(x) => write!(self.stdout, "{}", self.deck[*x]),
        };

        for (i, col) in self.foundation.iter().enumerate() {
            write!(self.stdout, "{}", cursor::Goto(28 + (i as u16 * 9), 1));
            match col.back() {
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
        // determine WHERE to draw cursor
        match self.cursor {
            card_col @ 0..=6 => write!(self.stdout, "{}", cursor::Goto(1 + (card_col as u16 * 9), 19)),
            foundation_col @ 7..=10 => write!(self.stdout, "{}", cursor::Goto(28 + ((foundation_col as u16 - 7)* 9), 2)),
            _ => write!(self.stdout, "{}", cursor::Goto(1, 2)),
        };
        // determine WHAT to draw as the cursor
        match self.selected_card {
            None => write!(self.stdout, "{}{}|=======|", color::Fg(color::Blue), color::Bg(color::Black)),
            Some(card_idx) => write!(self.stdout, "{}", self.deck[card_idx]),
        };
        write!(self.stdout, "{}{}{}{}", cursor::Goto(15, 1), color::Bg(color::Black), color::Fg(color::White), self.cursor);

        write!(self.stdout, "{}{}{}{}", style::Reset, cursor::Goto(1, 1), color::Fg(color::Reset), color::Bg(color::Reset));
        self.stdout.flush().unwrap();
    }
    fn put_card_back(&mut self){
        write!(self.stdout, "{BEL}");
        match self.card_selected_from_pos {
            c_col @ 0..=6 => self.board[c_col as usize].push_back(self.selected_card.unwrap()),
            f_col @ 7..=10 => self.foundation[f_col as usize].push_back(self.selected_card.unwrap()),
            _ => (),
        }

        self.selected_card = None;
    }
    fn try_move(&mut self){
        let card_idx = self.selected_card.unwrap();
        let card = self.deck[card_idx];
        
        match self.cursor {
            f_col @ 7..=10 => {
                let foundation = self.foundation[f_col as usize];
                match foundation.back() {
                    None => {
                        if 1 == card.value {
                            foundation.push_back(card_idx);
                        } else {
                            self.put_card_back();
                        }
                    },
                    Some(fcard_idx) => {
                        let fcard = self.deck[fcard_idx];
                        if fcard.suite == card.suite && fcard.value = card.value - 1 {
                            foundation.push_back(card_idx);
                        } else {
                            self.put_card_back();
                        }
                    }
                }
            },
            c_col @ 0..=6 => {
                let col = self.board[c_col as usize];
                match col.back() {
                    None => {
                        if 13 == card.value { // can only place kings on open space
                            foundation.push_back(card_idx);
                        } else {
                            self.put_card_back();
                        }
                    },
                    Some(fcard_idx) => {
                        let fcard = self.deck[*fcard_idx];
                        if fcard.suite == card.suite && fcard.value == card.value - 1 {
                            foundation.push_back(card_idx);
                        } else {
                            self.put_card_back();
                        }
                    }
                }
            },
            _ => (),
        }

    }
    fn run(&mut self){
        write!(self.stdout, "{}", cursor::Hide);
        loop {
            self.show();
            let b = self.stdin.next().unwrap().unwrap();
            use termion::event::Key::*;

            let col_height = match (match self.cursor {
                c_col @ 0..=6 => self.board[c_col as usize].len(),
                f_col @ 7..=10 => self.foundation[f_col as usize].len(),
                _ => 0
            }) {
                0 => 1,
                x @ _ => x,
            } as u8;
            match b {
                Char('q') | Char('c') => {
                    write!(self.stdout, "{}{}", style::Reset, cursor::Show);
                    return
                },
                Char('h') | Char('a') | Left  => self.cursor = (self.cursor as i16 - 1).rem_euclid(11) as u8,
                Char('l') | Char('d') | Right => self.cursor = (self.cursor + 1).rem_euclid(11),
                Char('j') | Char('s') | Down  => self.cursor = (self.cursor_y as i16 - 1).rem_euclid(col_height as i16) as u8,
                Char('k') | Char('w') | Up    => self.cursor = (self.cursor_y + 1).rem_euclid(col_height),
                Char(' ') => {
                    match self.selected_card {
                        None => {
                            self.card_selected_from_pos = self.cursor;
                            self.selected_card = match self.cursor {
                                c_col @ 0..=6 => self.board[c_col as usize].pop_back(),
                                f_col @ 7..=10 => self.foundation[f_col as usize].pop_back(),
                                _ => None,
                            };
                        },
                        Some(card_idx) => {
                            self.try_move();
                        },
                    }
                },
                _ => (),
            };
        }
    }
}

