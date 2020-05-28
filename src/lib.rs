//! Big fonts for terminal display. Each character is a block formed by many dots.
//!
//! # Example
//! ```rust
//! use terminal_fonts::{to_block_string};
//! 
//! fn main() {
//!     println!("{}", to_block_string("05:30:12 AM"))
//! }
//! ```
//! ```text
//!    ███   ██████           ██████   ███              ██    █████            ███   ██   ██
//!   █  ██  ██        ██        ██   █  ██    ██      ███   ██   ██          ██ ██  ███ ███
//!  ██   ██ ██████    ██       ██   ██   ██   ██       ██       ███         ██   ██ ███████
//!  ██   ██      ██           ████  ██   ██            ██     ████          ██   ██ ███████
//!  ██   ██      ██              ██ ██   ██            ██    ████           ███████ ██ █ ██
//!   ██  █  ██   ██   ██    ██   ██  ██  █    ██       ██   ███             ██   ██ ██   ██
//!    ███    █████    ██     █████    ███     ██     ██████ ███████         ██   ██ ██   ██
//! ```
//!

use lazy_static::lazy_static;
use std::collections::HashMap;

const C: &str = "█";
const E: &str = " ";

lazy_static! {
    static ref EMPTY_BLOCK: Block = vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![]];
    static ref BLKS: HashMap<char, Vec<Vec<&'static str>>> = {
        let mut m = HashMap::new();
        m.insert(
            '0',
            vec![
                vec![E, E, C, C, C, E, E],
                vec![E, C, E, E, C, C, E],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![E, C, C, E, E, C, E],
                vec![E, E, C, C, C, E, E],
            ],
        );
        m.insert(
            '1',
            vec![
                vec![E, E, E, C, C, E, E],
                vec![E, E, C, C, C, E, E],
                vec![E, E, E, C, C, E, E],
                vec![E, E, E, C, C, E, E],
                vec![E, E, E, C, C, E, E],
                vec![E, E, E, C, C, E, E],
                vec![E, C, C, C, C, C, C],
            ],
        );
        m.insert(
            '2',
            vec![
                vec![E, C, C, C, C, C, E],
                vec![C, C, E, E, E, C, C],
                vec![E, E, E, E, C, C, C],
                vec![E, E, C, C, C, C, E],
                vec![E, C, C, C, C, E, E],
                vec![C, C, C, E, E, E, E],
                vec![C, C, C, C, C, C, C],
            ],
        );
        m.insert(
            '3',
            vec![
                vec![E, C, C, C, C, C, C],
                vec![E, E, E, E, C, C, E],
                vec![E, E, E, C, C, E, E],
                vec![E, E, C, C, C, C, E],
                vec![E, E, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![E, C, C, C, C, C, E],
            ],
        );
        m.insert(
            '4',
            vec![
                vec![E, E, E, C, C, C, E],
                vec![E, E, C, C, C, C, E],
                vec![E, C, C, E, C, C, E],
                vec![C, C, E, E, C, C, E],
                vec![C, C, C, C, C, C, C],
                vec![E, E, E, E, C, C, E],
                vec![E, E, E, E, C, C, E],
            ],
        );
        m.insert(
            '5',
            vec![
                vec![C, C, C, C, C, C, E],
                vec![C, C, E, E, E, E, E],
                vec![C, C, C, C, C, C, E],
                vec![E, E, E, E, E, C, C],
                vec![E, E, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![E, C, C, C, C, C, E],
            ],
        );
        m.insert(
            '6',
            vec![
                vec![E, E, C, C, C, C, E],
                vec![E, C, C, E, E, E, E],
                vec![C, C, E, E, E, E, E],
                vec![C, C, C, C, C, C, E],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![E, C, C, C, C, C, E],
            ],
        );
        m.insert(
            '7',
            vec![
                vec![C, C, C, C, C, C, C],
                vec![C, C, E, E, E, C, C],
                vec![E, E, E, E, C, C, E],
                vec![E, E, E, C, C, E, E],
                vec![E, E, C, C, E, E, E],
                vec![E, E, C, C, E, E, E],
                vec![E, E, C, C, E, E, E],
            ],
        );
        m.insert(
            '8',
            vec![
                vec![E, C, C, C, C, E, E],
                vec![C, C, E, E, E, C, E],
                vec![C, C, C, E, E, C, E],
                vec![E, C, C, C, C, E, E],
                vec![C, E, E, C, C, C, C],
                vec![C, E, E, E, E, C, C],
                vec![E, C, C, C, C, C, E],
            ],
        );
        m.insert(
            '9',
            vec![
                vec![E, C, C, C, C, C, E],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![E, C, C, C, C, C, C],
                vec![E, E, E, E, E, C, C],
                vec![E, E, E, E, C, C, E],
                vec![E, C, C, C, C, E, E],
            ],
        );
        m.insert(
            '?',
            vec![
                vec![E, C, C, C, C, C, E],
                vec![C, C, E, E, E, C, C],
                vec![E, E, E, C, C, C, E],
                vec![E, E, C, C, E, E, E],
                vec![E, E, E, E, E, E, E],
                vec![E, E, C, C, E, E, E],
                vec![E, E, C, C, E, E, E],
            ],
        );
        m.insert(
            '!',
            vec![
                vec![E, E, E, C, C, E, E],
                vec![E, E, E, C, C, E, E],
                vec![E, E, E, C, C, E, E],
                vec![E, E, E, C, C, E, E],
                vec![E, E, E, E, E, E, E],
                vec![E, E, E, C, C, E, E],
                vec![E, E, E, C, C, E, E],
            ],
        );
        m.insert(
            '#',
            vec![
                vec![E, E, E, E, E, E, E],
                vec![E, E, C, E, C, E, E],
                vec![E, C, C, C, C, C, E],
                vec![E, E, C, E, C, E, E],
                vec![E, C, C, C, C, C, E],
                vec![E, E, C, E, C, E, E],
                vec![E, E, E, E, E, E, E],
            ],
        );
        m.insert(
            ':',
            vec![
                vec![E, E, E, E, E, E, E],
                vec![E, E, C, C, E, E, E],
                vec![E, E, C, C, E, E, E],
                vec![E, E, E, E, E, E, E],
                vec![E, E, E, E, E, E, E],
                vec![E, E, C, C, E, E, E],
                vec![E, E, C, C, E, E, E],
            ],
        );
        m.insert(
            '+',
            vec![
                vec![E, E, E, E, E, E, E],
                vec![E, E, E, C, E, E, E],
                vec![E, E, E, C, E, E, E],
                vec![E, C, C, C, C, C, E],
                vec![E, E, E, C, E, E, E],
                vec![E, E, E, C, E, E, E],
                vec![E, E, E, E, E, E, E],
            ],
        );
        m.insert(
            '-',
            vec![
                vec![E, E, E, E, E, E, E],
                vec![E, E, E, E, E, E, E],
                vec![E, E, E, E, E, E, E],
                vec![E, C, C, C, C, C, E],
                vec![E, E, E, E, E, E, E],
                vec![E, E, E, E, E, E, E],
                vec![E, E, E, E, E, E, E],
            ],
        );
        m.insert(
            '*',
            vec![
                vec![E, E, E, E, E, E, E],
                vec![E, E, E, C, E, E, E],
                vec![E, C, E, C, E, C, E],
                vec![E, E, C, C, C, E, E],
                vec![E, C, E, C, E, C, E],
                vec![E, E, E, C, E, E, E],
                vec![E, E, E, E, E, E, E],
            ],
        );
        m.insert(
            '/',
            vec![
                vec![E, E, E, E, E, E, C],
                vec![E, E, E, E, E, C, E],
                vec![E, E, E, E, C, E, E],
                vec![E, E, E, C, E, E, E],
                vec![E, E, C, E, E, E, E],
                vec![E, C, E, E, E, E, E],
                vec![C, E, E, E, E, E, E],
            ],
        );
        m.insert(
            '=',
            vec![
                vec![E, E, E, E, E, E, E],
                vec![E, E, E, E, E, E, E],
                vec![E, C, C, C, C, C, E],
                vec![E, E, E, E, E, E, E],
                vec![E, C, C, C, C, C, E],
                vec![E, E, E, E, E, E, E],
                vec![E, E, E, E, E, E, E],
            ],
        );
        m.insert(
            ' ',
            vec![
                vec![E, E, E, E, E, E, E],
                vec![E, E, E, E, E, E, E],
                vec![E, E, E, E, E, E, E],
                vec![E, E, E, E, E, E, E],
                vec![E, E, E, E, E, E, E],
                vec![E, E, E, E, E, E, E],
                vec![E, E, E, E, E, E, E],
            ],
        );
        m.insert(
            'A',
            vec![
                vec![E, E, C, C, C, E, E],
                vec![E, C, C, E, C, C, E],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, C, C, C, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
            ],
        );
        m.insert(
            'B',
            vec![
                vec![C, C, C, C, C, C, E],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, C, C, C, C, E],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, C, C, C, C, E],
            ],
        );
        m.insert(
            'C',
            vec![
                vec![E, E, C, C, C, C, E],
                vec![E, C, C, E, E, C, C],
                vec![C, C, E, E, E, E, E],
                vec![C, C, E, E, E, E, E],
                vec![C, C, E, E, E, E, E],
                vec![E, C, C, E, E, C, C],
                vec![E, E, C, C, C, C, E],
            ],
        );
        m.insert(
            'D',
            vec![
                vec![C, C, C, C, C, E, E],
                vec![C, C, E, E, C, C, E],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, C, C, E],
                vec![C, C, C, C, C, E, E],
            ],
        );
        m.insert(
            'E',
            vec![
                vec![C, C, C, C, C, C, C],
                vec![C, C, E, E, E, E, E],
                vec![C, C, E, E, E, E, E],
                vec![C, C, C, C, C, C, E],
                vec![C, C, E, E, E, E, E],
                vec![C, C, E, E, E, E, E],
                vec![C, C, C, C, C, C, C],
            ],
        );
        m.insert(
            'F',
            vec![
                vec![C, C, C, C, C, C, C],
                vec![C, C, E, E, E, E, E],
                vec![C, C, E, E, E, E, E],
                vec![C, C, C, C, C, C, E],
                vec![C, C, E, E, E, E, E],
                vec![C, C, E, E, E, E, E],
                vec![C, C, E, E, E, E, E],
            ],
        );
        m.insert(
            'G',
            vec![
                vec![E, E, C, C, C, C, C],
                vec![E, C, C, E, E, E, E],
                vec![C, C, E, E, E, E, E],
                vec![C, C, E, E, C, C, C],
                vec![C, C, E, E, E, C, C],
                vec![E, C, C, E, E, C, C],
                vec![E, E, C, C, C, C, C],
            ],
        );
        m.insert(
            'H',
            vec![
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, C, C, C, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
            ],
        );
        m.insert(
            'I',
            vec![
                vec![E, C, C, C, C, C, C],
                vec![E, E, E, C, C, E, E],
                vec![E, E, E, C, C, E, E],
                vec![E, E, E, C, C, E, E],
                vec![E, E, E, C, C, E, E],
                vec![E, E, E, C, C, E, E],
                vec![E, C, C, C, C, C, C],
            ],
        );
        m.insert(
            'J',
            vec![
                vec![E, E, E, E, E, C, C],
                vec![E, E, E, E, E, C, C],
                vec![E, E, E, E, E, C, C],
                vec![E, E, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![E, C, C, C, C, C, E],
            ],
        );
        m.insert(
            'K',
            vec![
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, C, C, E],
                vec![C, C, E, C, C, E, E],
                vec![C, C, C, C, E, E, E],
                vec![C, C, C, C, C, E, E],
                vec![C, C, E, C, C, C, E],
                vec![C, C, E, E, C, C, C],
            ],
        );
        m.insert(
            'L',
            vec![
                vec![C, C, E, E, E, E, E],
                vec![C, C, E, E, E, E, E],
                vec![C, C, E, E, E, E, E],
                vec![C, C, E, E, E, E, E],
                vec![C, C, E, E, E, E, E],
                vec![C, C, E, E, E, E, E],
                vec![C, C, C, C, C, C, C],
            ],
        );
        m.insert(
            'M',
            vec![
                vec![C, C, E, E, E, C, C],
                vec![C, C, C, E, C, C, C],
                vec![C, C, C, C, C, C, C],
                vec![C, C, C, C, C, C, C],
                vec![C, C, E, C, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
            ],
        );
        m.insert(
            'N',
            vec![
                vec![C, C, E, E, E, C, C],
                vec![C, C, C, E, E, C, C],
                vec![C, C, C, C, E, C, C],
                vec![C, C, C, C, C, C, C],
                vec![C, C, E, C, C, C, C],
                vec![C, C, E, E, C, C, C],
                vec![C, C, E, E, E, C, C],
            ],
        );
        m.insert(
            'O',
            vec![
                vec![E, C, C, C, C, C, E],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![E, C, C, C, C, C, E],
            ],
        );
        m.insert(
            'P',
            vec![
                vec![C, C, C, C, C, C, E],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, C, C, C, C, E],
                vec![C, C, E, E, E, E, E],
                vec![C, C, E, E, E, E, E],
            ],
        );
        m.insert(
            'Q',
            vec![
                vec![E, C, C, C, C, C, E],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, C, C, C, C],
                vec![C, C, E, E, C, C, E],
                vec![E, C, C, C, C, E, C],
            ],
        );
        m.insert(
            'R',
            vec![
                vec![C, C, C, C, C, C, E],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, C, C, C],
                vec![C, C, C, C, C, E, E],
                vec![C, C, E, C, C, C, E],
                vec![C, C, E, E, C, C, C],
            ],
        );
        m.insert(
            'S',
            vec![
                vec![E, C, C, C, C, E, E],
                vec![C, C, E, E, C, C, E],
                vec![C, C, E, E, E, E, E],
                vec![E, C, C, C, C, C, E],
                vec![E, E, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![E, C, C, C, C, C, E],
            ],
        );
        m.insert(
            'T',
            vec![
                vec![E, C, C, C, C, C, C],
                vec![E, E, E, C, C, E, E],
                vec![E, E, E, C, C, E, E],
                vec![E, E, E, C, C, E, E],
                vec![E, E, E, C, C, E, E],
                vec![E, E, E, C, C, E, E],
                vec![E, E, E, C, C, E, E],
            ],
        );
        m.insert(
            'U',
            vec![
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![E, C, C, C, C, C, E],
            ],
        );
        m.insert(
            'V',
            vec![
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, C, E, C, C, C],
                vec![E, C, C, C, C, C, E],
                vec![E, E, C, C, C, E, E],
                vec![E, E, E, C, E, E, E],
            ],
        );
        m.insert(
            'W',
            vec![
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, E, E, C, C],
                vec![C, C, E, C, E, C, C],
                vec![C, C, C, C, C, C, C],
                vec![C, C, C, E, C, C, C],
                vec![C, C, E, E, E, C, C],
            ],
        );
        m.insert(
            'X',
            vec![
                vec![C, C, E, E, E, C, C],
                vec![C, C, C, E, C, C, C],
                vec![E, C, C, C, C, C, E],
                vec![E, E, C, C, C, E, E],
                vec![E, C, C, C, C, C, E],
                vec![C, C, C, E, C, C, C],
                vec![C, C, E, E, E, C, C],
            ],
        );
        m.insert(
            'Y',
            vec![
                vec![E, C, C, E, E, C, C],
                vec![E, C, C, E, E, C, C],
                vec![E, C, C, E, E, C, C],
                vec![E, E, C, C, C, C, E],
                vec![E, E, E, C, C, E, E],
                vec![E, E, E, C, C, E, E],
                vec![E, E, E, C, C, E, E],
            ],
        );
        m.insert(
            'Z',
            vec![
                vec![C, C, C, C, C, C, C],
                vec![E, E, E, E, C, C, C],
                vec![E, E, E, C, C, C, E],
                vec![E, E, C, C, C, E, E],
                vec![E, C, C, C, E, E, E],
                vec![C, C, C, E, E, E, E],
                vec![C, C, C, C, C, C, C],
            ],
        );
        m
    };
}

type Block = Vec<Vec<String>>;

pub fn concat_block(block1: &Block, block2: &Block) -> Block {
    block1
        .iter()
        .zip(block2.iter())
        .map(|(line1, line2)| {
            line1
                .iter()
                .cloned()
                .chain(
                    vec![E.to_owned()]
                        .iter()
                        .cloned()
                        .chain(line2.iter().cloned()),
                )
                .collect()
        })
        .collect()
}

pub fn concat_blocks(blocks: &[&Block]) -> Block {
    if blocks.len() == 0 {
        return EMPTY_BLOCK.to_vec();
    }
    blocks
        .iter()
        .skip(1)
        .fold(blocks[0].to_vec(), |acc, c| concat_block(&acc, c))
}

pub fn to_block(text: &str) -> Block {
    text.chars().fold(EMPTY_BLOCK.to_vec(), |acc, c| {
        concat_block(&acc, &get_char_block(&c))
    })
}

pub fn map_block<F>(block: &Block, mapper: F) -> Block
where
    F: Fn(&str) -> String,
{
    block
        .iter()
        .map(|line| {
            line.iter()
                .map(|cell| mapper(cell.as_str()))
                .collect::<Vec<String>>()
        })
        .collect()
}

pub fn to_string(block: &Block) -> String {
    block
        .iter()
        .map(|line| line.iter().map(|v| v.to_owned()).collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn to_block_string(text: &str) -> String {
    to_string(&to_block(text))
}

fn get_char_block(c: &char) -> Block {
    BLKS.get(c)
        .unwrap_or(&BLKS[&'?'])
        .iter()
        .map(|line| {
            line.iter()
                .map(|line| line.to_string())
                .collect::<Vec<String>>()
        })
        .collect()
}
