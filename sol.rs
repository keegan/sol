extern crate termion;

use rand::seq::SliceRandom;
use rand::thread_rng;

use std::thread::sleep;
use std::time;

use std::process::exit;

use std::collections::VecDeque;

use termion::input::Keys;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, style};

use std::fmt;
use std::io::{self, StdinLock, Write};

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

impl Card {
    fn op_suite(&self, other: &Card) -> bool {
        match self.suite {
            Suite::Heart | Suite::Diamond => match other.suite {
                Suite::Heart | Suite::Diamond => false,
                Suite::Club | Suite::Spade => true,
            },
            Suite::Club | Suite::Spade => match other.suite {
                Suite::Heart | Suite::Diamond => true,
                Suite::Club | Suite::Spade => false,
            },
        }
    }
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
        Card {
            hidden: true,
            value: 1,
            suite: Suite::Heart,
        },
        Card {
            hidden: true,
            value: 2,
            suite: Suite::Heart,
        },
        Card {
            hidden: true,
            value: 3,
            suite: Suite::Heart,
        },
        Card {
            hidden: true,
            value: 4,
            suite: Suite::Heart,
        },
        Card {
            hidden: true,
            value: 5,
            suite: Suite::Heart,
        },
        Card {
            hidden: true,
            value: 6,
            suite: Suite::Heart,
        },
        Card {
            hidden: true,
            value: 7,
            suite: Suite::Heart,
        },
        Card {
            hidden: true,
            value: 8,
            suite: Suite::Heart,
        },
        Card {
            hidden: true,
            value: 9,
            suite: Suite::Heart,
        },
        Card {
            hidden: true,
            value: 10,
            suite: Suite::Heart,
        },
        Card {
            hidden: true,
            value: 11,
            suite: Suite::Heart,
        },
        Card {
            hidden: true,
            value: 12,
            suite: Suite::Heart,
        },
        Card {
            hidden: true,
            value: 13,
            suite: Suite::Heart,
        },
        Card {
            hidden: true,
            value: 1,
            suite: Suite::Diamond,
        },
        Card {
            hidden: true,
            value: 2,
            suite: Suite::Diamond,
        },
        Card {
            hidden: true,
            value: 3,
            suite: Suite::Diamond,
        },
        Card {
            hidden: true,
            value: 4,
            suite: Suite::Diamond,
        },
        Card {
            hidden: true,
            value: 5,
            suite: Suite::Diamond,
        },
        Card {
            hidden: true,
            value: 6,
            suite: Suite::Diamond,
        },
        Card {
            hidden: true,
            value: 7,
            suite: Suite::Diamond,
        },
        Card {
            hidden: true,
            value: 8,
            suite: Suite::Diamond,
        },
        Card {
            hidden: true,
            value: 9,
            suite: Suite::Diamond,
        },
        Card {
            hidden: true,
            value: 10,
            suite: Suite::Diamond,
        },
        Card {
            hidden: true,
            value: 11,
            suite: Suite::Diamond,
        },
        Card {
            hidden: true,
            value: 12,
            suite: Suite::Diamond,
        },
        Card {
            hidden: true,
            value: 13,
            suite: Suite::Diamond,
        },
        Card {
            hidden: true,
            value: 1,
            suite: Suite::Club,
        },
        Card {
            hidden: true,
            value: 2,
            suite: Suite::Club,
        },
        Card {
            hidden: true,
            value: 3,
            suite: Suite::Club,
        },
        Card {
            hidden: true,
            value: 4,
            suite: Suite::Club,
        },
        Card {
            hidden: true,
            value: 5,
            suite: Suite::Club,
        },
        Card {
            hidden: true,
            value: 6,
            suite: Suite::Club,
        },
        Card {
            hidden: true,
            value: 7,
            suite: Suite::Club,
        },
        Card {
            hidden: true,
            value: 8,
            suite: Suite::Club,
        },
        Card {
            hidden: true,
            value: 9,
            suite: Suite::Club,
        },
        Card {
            hidden: true,
            value: 10,
            suite: Suite::Club,
        },
        Card {
            hidden: true,
            value: 11,
            suite: Suite::Club,
        },
        Card {
            hidden: true,
            value: 12,
            suite: Suite::Club,
        },
        Card {
            hidden: true,
            value: 13,
            suite: Suite::Club,
        },
        Card {
            hidden: true,
            value: 1,
            suite: Suite::Spade,
        },
        Card {
            hidden: true,
            value: 2,
            suite: Suite::Spade,
        },
        Card {
            hidden: true,
            value: 3,
            suite: Suite::Spade,
        },
        Card {
            hidden: true,
            value: 4,
            suite: Suite::Spade,
        },
        Card {
            hidden: true,
            value: 5,
            suite: Suite::Spade,
        },
        Card {
            hidden: true,
            value: 6,
            suite: Suite::Spade,
        },
        Card {
            hidden: true,
            value: 7,
            suite: Suite::Spade,
        },
        Card {
            hidden: true,
            value: 8,
            suite: Suite::Spade,
        },
        Card {
            hidden: true,
            value: 9,
            suite: Suite::Spade,
        },
        Card {
            hidden: true,
            value: 10,
            suite: Suite::Spade,
        },
        Card {
            hidden: true,
            value: 11,
            suite: Suite::Spade,
        },
        Card {
            hidden: true,
            value: 12,
            suite: Suite::Spade,
        },
        Card {
            hidden: true,
            value: 13,
            suite: Suite::Spade,
        },
    ];
    deck.shuffle(&mut thread_rng());

    let stdout = io::stdout();
    let stdout = stdout.lock();

    let stdin = io::stdin();
    let stdin = stdin.lock();

    let stdout = stdout.into_raw_mode().unwrap();
    let termsize = termion::terminal_size().ok();
    let termwidth = termsize.map(|(w, _)| w - 2).unwrap();

    let mut board: [VecDeque<usize>; 7] = [
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
    ];

    let mut counter = 0;

    for i in 0..7 {
        let col = &mut board[i];
        for j in 0..=i {
            col.push_back(counter);
            counter += 1;
        }
    }

    let drawpile: VecDeque<usize> = (counter..52).collect();
    for card in drawpile.iter() {
        deck.get_mut(*card).unwrap().hidden = false;
    }

    let mut game = Game {
        width: termwidth,
        stdout: Box::new(stdout),
        stdin: Box::new(stdin.keys()),

        deck: deck,
        board: board,

        draw: drawpile,
        foundation: [
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
        ],

        cursor: 0,
        cursor_y: 0,
        selected_card: None,
        card_selected_from_pos: 80,
        selected_cards: VecDeque::new(),
    };

    game.run();
}

/// The game state.
struct Game<'a> {
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
    fn show(&mut self) {
        write!(
            self.stdout,
            "{}{}{}{}{}",
            clear::All,
            style::Reset,
            cursor::Goto(1, 1),
            color::Fg(color::Reset),
            color::Bg(color::Reset)
        )
        .unwrap();
        match self.draw.back() {
            None => write!(
                self.stdout,
                "{}{}|#x#x#x#|{}",
                color::Bg(color::Yellow),
                color::Fg(color::Black),
                style::Reset
            ),
            Some(x) => write!(self.stdout, "{}", self.deck[*x]),
        }
        .unwrap();

        for (i, col) in self.foundation.iter().enumerate() {
            write!(self.stdout, "{}", cursor::Goto(28 + (i as u16 * 9), 1)).unwrap();
            match col.back() {
                None => write!(
                    self.stdout,
                    "{}{}|#x#x#x#|{}",
                    color::Bg(color::Yellow),
                    color::Fg(color::LightYellow),
                    style::Reset
                ),
                Some(x) => write!(self.stdout, "{}", self.deck[*x]),
            }
            .unwrap();
        }

        //write!(self.stdout, "{1}{0}", self.deck[*card as usize]);
        for (i, col) in self.board.iter().enumerate() {
            for (j, card) in col.iter().enumerate() {
                write!(
                    self.stdout,
                    "{1}{0}",
                    self.deck[*card as usize],
                    cursor::Goto(1 + (i as u16 * 9), 3 + j as u16)
                )
                .unwrap();
            }
        }
        // determine WHERE to draw cursor
        if self.cursor_y > 0 {
            write!(
                self.stdout,
                "{}{}",
                color::Fg(color::Blue),
                color::Bg(color::LightBlue)
            )
            .unwrap();
            match self.cursor {
                card_col @ 0..=6 => {
                    let y: u16 = 2 + self.cursor_y as u16;
                    let x: u16 = 1 + (card_col as u16 * 9);
                    write!(
                        self.stdout,
                        "{}|{}|",
                        cursor::Goto(x, y),
                        cursor::Goto(x + 8, y)
                    )
                }
                foundation_col @ 7..=10 => write!(
                    self.stdout,
                    "{}|{}|",
                    cursor::Goto(28 + ((foundation_col as u16 - 7) * 9), 1),
                    cursor::Goto(37 + ((foundation_col as u16 - 7) * 9), 1)
                ),
                _ => write!(self.stdout, "{}", cursor::Goto(1, 2)),
            }
            .unwrap();
        }

        match self.cursor {
            card_col @ 0..=6 => write!(
                self.stdout,
                "{}",
                cursor::Goto(1 + (card_col as u16 * 9), 20)
            ),
            foundation_col @ 7..=10 => write!(
                self.stdout,
                "{}",
                cursor::Goto(28 + ((foundation_col as u16 - 7) * 9), 2)
            ),
            _ => Ok(()),
        }
        .unwrap();
        // determine WHAT to draw as the cursor
        match self.selected_card {
            None => {
                if self.cursor_y == 0 {
                    write!(
                        self.stdout,
                        "{}{}|=======|",
                        color::Fg(color::Blue),
                        color::Bg(color::Black)
                    )
                } else {
                    Ok(())
                }
            }
            Some(card_idx) => write!(self.stdout, "{}", self.deck[card_idx]),
        }
        .unwrap();
        write!(
            self.stdout,
            "{}{}{}{}/{}",
            cursor::Goto(15, 1),
            color::Bg(color::Black),
            color::Fg(color::White),
            self.cursor,
            self.cursor_y
        )
        .unwrap();

        write!(
            self.stdout,
            "{}{}{}{}",
            style::Reset,
            cursor::Goto(1, 1),
            color::Fg(color::Reset),
            color::Bg(color::Reset)
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }
    fn run(&mut self) {
        write!(self.stdout, "{}", cursor::Hide).unwrap();
        loop {
            if self.draw.len() == 0 {
                let mut still_playing = false;
                for card in self.deck.iter() {
                    still_playing |= card.hidden; // if any cards hidden, we keep playing
                }
                if !still_playing {
                    return self.win();
                }
            }
            for i in 0..7 {
                if i != self.card_selected_from_pos {
                    let col: &mut VecDeque<usize> = &mut self.board[i as usize];
                    if col.len() > 0 {
                        self.deck[*col.back().unwrap()].hidden = false;
                    }
                }
            }
            self.show();
            let b = self.stdin.next().unwrap().unwrap();
            use termion::event::Key::*;

            let col_height = match self.cursor {
                c_col @ 0..=6 => self.board[c_col as usize].len() + 1,
                _f_col @ 7..=10 => 1,
                _ => 1,
            };
            match b {
                Char('q') | Char('c') => {
                    write!(self.stdout, "{}{}", style::Reset, cursor::Show).unwrap();
                    return;
                }
                Char('f') => {
                    // you cant rotate drawpile if holding a card
                    if self.card_selected_from_pos > 11 {
                        self.draw.rotate_right(1);
                    }
                }
                Char('e') => {
                    match self.selected_card {
                        Some(_) => (),
                        None => {
                            self.selected_card = self.draw.pop_back();
                            self.selected_cards.push_back(self.selected_card.unwrap());
                            self.card_selected_from_pos = 11;
                        }
                    };
                }
                Char('h') | Char('a') | Left => {
                    self.cursor = (self.cursor as i16 - 1).rem_euclid(11) as u8
                }
                Char('l') | Char('d') | Right => self.cursor = (self.cursor + 1).rem_euclid(11),
                Char('j') | Char('s') | Down => {
                    self.cursor_y = (self.cursor_y + 1).rem_euclid(col_height as u8)
                }
                Char('k') | Char('w') | Up => {
                    self.cursor_y = (self.cursor_y as i16 - 1).rem_euclid(col_height as i16) as u8
                }
                Char(' ') => {
                    match self.selected_card {
                        None => {
                            self.card_selected_from_pos = self.cursor;
                            match self.cursor {
                                c_col @ 0..=6 => {
                                    let col = &mut self.board[c_col as usize];
                                    let height: u8 = col.len() as u8;
                                    self.cursor_y = if self.cursor_y > height {
                                        height
                                    } else {
                                        self.cursor_y
                                    };
                                    if 0 == self.cursor_y {
                                        self.selected_card = col.pop_back();
                                        match self.selected_card {
                                            None => (),
                                            Some(c) => self.selected_cards.push_back(c),
                                        }
                                    } else {
                                        let idx = self.cursor_y as usize - 1;
                                        col.make_contiguous();
                                        let mut iterator = col.as_slices().0[idx..].iter();
                                        let mut prev_card: Option<&usize> = None;
                                        let is_valid_stack: bool = loop {
                                            let card = iterator.next();
                                            match prev_card {
                                                None => {
                                                    match card {
                                                        None => break false, // empty list? shouldnt be possible
                                                        Some(ci) => {
                                                            let c = &self.deck[*ci];
                                                            if c.hidden {
                                                                break false;
                                                            }
                                                            prev_card = card;
                                                        }
                                                    }
                                                }
                                                Some(pci) => {
                                                    match card {
                                                        None => break true, // weve gone through the whole stack and nothing looked bad
                                                        Some(ci) => {
                                                            let pc = &self.deck[*pci];
                                                            let c = &self.deck[*ci];
                                                            if c.hidden {
                                                                break false;
                                                            }
                                                            if !pc.op_suite(c) {
                                                                break false;
                                                            }
                                                            if pc.value != c.value + 1 {
                                                                break false;
                                                            }
                                                            prev_card = card;
                                                        }
                                                    }
                                                }
                                            }
                                        };
                                        if is_valid_stack {
                                            self.selected_cards = col.split_off(idx);
                                            self.selected_card =
                                                self.selected_cards.front().copied();
                                        }
                                    }
                                }
                                f_col @ 7..=10 => {
                                    self.selected_card =
                                        self.foundation[(f_col - 7) as usize].pop_back();
                                }
                                _ => self.selected_card = None,
                            };
                        }
                        Some(card_idx) => {
                            let card = &self.deck[card_idx];

                            let can_place = match self.cursor {
                                c_col @ 0..=6 => {
                                    let col = &self.board[c_col as usize];
                                    match col.back() {
                                        None => {
                                            if card.value == 13 {
                                                true
                                            } else {
                                                false
                                            }
                                        }
                                        Some(back_card_idx) => {
                                            let back_card: &Card = &self.deck[*back_card_idx];
                                            card.op_suite(back_card)
                                                && (card.value == back_card.value - 1)
                                        }
                                    }
                                }
                                f_col @ 7..=10 => {
                                    if self.selected_cards.len() > 1 {
                                        false
                                    } else {
                                        let fcol: &VecDeque<usize> =
                                            &self.foundation[(f_col - 7) as usize];
                                        match fcol.back() {
                                            None => {
                                                if card.value == 1 {
                                                    true
                                                } else {
                                                    false
                                                }
                                            }
                                            Some(back_card_idx) => {
                                                let back_card: &Card = &self.deck[*back_card_idx];
                                                (card.suite == back_card.suite)
                                                    && (card.value == back_card.value + 1)
                                            }
                                        }
                                    }
                                }
                                _ => false,
                            };
                            if can_place {
                                match self.cursor {
                                    c_col @ 0..=6 => {
                                        self.board[c_col as usize]
                                            .extend(self.selected_cards.iter());
                                    }
                                    f_col @ 7..=10 => {
                                        self.foundation[(f_col - 7) as usize]
                                            .push_back(self.selected_card.unwrap());
                                    }
                                    _ => (),
                                }
                            } else {
                                write!(self.stdout, "{BEL}").unwrap();
                                // put the card(s) back
                                match self.card_selected_from_pos {
                                    c_col @ 0..=6 => {
                                        self.board[c_col as usize]
                                            .extend(self.selected_cards.iter());
                                    }
                                    f_col @ 7..=10 => {
                                        self.foundation[(f_col - 7) as usize]
                                            .push_back(self.selected_card.unwrap());
                                    }
                                    11 => {
                                        self.draw.push_back(self.selected_card.unwrap());
                                    }
                                    _ => (),
                                }
                            }
                            self.selected_card = None;
                            self.selected_cards.clear();
                            self.card_selected_from_pos = 80;
                        }
                    }
                }
                _ => (),
            };
        }
    }
    fn win(&mut self) {
        let mut ring: VecDeque<usize> = VecDeque::new();
        self.foundation[0] = (0..13).collect();
        self.foundation[1] = (13..26).collect();
        self.foundation[2] = (26..39).collect();
        self.foundation[3] = (39..52).collect();
        let mut ctr: usize = 0;
        let mut ctr1: usize = 0;
        loop {
            if self.foundation[0].len() > 1 {
                ring.push_front(self.foundation[0].pop_back().unwrap());
            } else if self.foundation[1].len() > 1 {
                ring.push_front(self.foundation[1].pop_back().unwrap());
            } else if self.foundation[2].len() > 1 {
                ring.push_front(self.foundation[2].pop_back().unwrap());
            } else if self.foundation[3].len() > 1 {
                ring.push_front(self.foundation[3].pop_back().unwrap());
            } else {
                ctr1 += 1;
                ring.rotate_left(1);
            }
            if ctr1 > 20 {
                break;
            }
            ctr = ctr + 1;
            self.show();
            for (i, cidx) in ring.iter().enumerate() {
                let card = &mut self.deck[*cidx];
                if ((i + ctr) % 2) == 0 {
                    card.hidden = false;
                } else {
                    card.hidden = true;
                }
                let x: u16;
                let y: u16;
                if i < 25 {
                    y = 3 + i as u16;
                    x = 50 - ((3.0 * (169.0 - (17 - y as i16).pow(2) as f64).sqrt()) as u16);
                } else {
                    y = 3 + 52 - i as u16;
                    x = 50 + ((3.0 * (169.0 - (17 - y as i16).pow(2) as f64).sqrt()) as u16);
                }
                write!(self.stdout, "{}{}", cursor::Goto(x, y), card).unwrap();
            }
            self.stdout.flush().unwrap();
            sleep(time::Duration::from_millis(50));
        }
        ctr = 1;
        loop {
            if ctr > 800 {
                break;
            }
            ctr += 1;
            let xscale: f64 =
                2.0 * ((ctr as f64 / 23.0).cos().abs() + (ctr as f64 / 130.0).sin().abs() * 0.5);
            self.show();
            for (i, cidx) in ring.iter().enumerate() {
                let card = &mut self.deck[*cidx];
                if ((i + ctr) % 2) == 0 {
                    card.hidden = false;
                } else {
                    card.hidden = true;
                }
                let x: u16;
                let y: u16;
                if i < 25 {
                    y = 3 + i as u16;
                    x = 50 - ((xscale * (169.0 - (17 - y as i16).pow(2) as f64).sqrt()) as u16);
                } else {
                    y = 3 + 52 - i as u16;
                    x = 50 + ((xscale * (169.0 - (17 - y as i16).pow(2) as f64).sqrt()) as u16);
                }
                write!(self.stdout, "{}{}", cursor::Goto(x, y), card).unwrap();
            }
            self.stdout.flush().unwrap();
            sleep(time::Duration::from_millis(5));
        }
        write!(
            self.stdout,
            "{}{}{}",
            cursor::Goto(10, 40),
            color::Fg(color::Green),
            color::Bg(color::Black)
        )
        .unwrap();
        write!(self.stdout, "You Win!!!").unwrap();
        write!(self.stdout, "{}", cursor::Goto(1, 51)).unwrap();
        write!(
            self.stdout,
            "{}{}{}{}",
            style::Reset,
            cursor::Show,
            color::Fg(color::Reset),
            color::Bg(color::Reset)
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }
}
