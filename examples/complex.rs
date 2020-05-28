use terminal_fonts::{concat_blocks, map_block, to_block, to_string};

fn red(v: &str) -> String {
    format!("{}{}{}", "\u{001b}[31m", v, "\u{001b}[0m")
}

fn yellow(v: &str) -> String {
    format!("{}{}{}", "\u{001b}[33m", v, "\u{001b}[0m")
}

fn blue(v: &str) -> String {
    format!("{}{}{}", "\u{001b}[34m", v, "\u{001b}[0m")
}

fn main() {
    let hour_block = map_block(&to_block("05"), red);
    let minute_block = map_block(&to_block("30"), yellow);
    let second_block = map_block(&to_block("12"), blue);
    let sep_block = to_block(":");

    let result = to_string(&concat_blocks(&vec![
        &hour_block,
        &sep_block,
        &minute_block,
        &sep_block,
        &second_block,
    ]));
    println!("{}", result)
}
