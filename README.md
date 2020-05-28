# Terminal Fonts

Big fonts for terminal display. Each character is a block formed by many dots.

The font data came from [Arcade](https://www.dafont.com/arcade-ya.font), font copy rights belongs to the original author.

## Usage

### Generate block string directly

```rs
use terminal_fonts::{to_block_string};

fn main() {
    println!("{}", to_block_string("05:30:12 AM"))
}
```

```
   ███   ██████           ██████   ███              ██    █████            ███   ██   ██
  █  ██  ██        ██        ██   █  ██    ██      ███   ██   ██          ██ ██  ███ ███
 ██   ██ ██████    ██       ██   ██   ██   ██       ██       ███         ██   ██ ███████
 ██   ██      ██           ████  ██   ██            ██     ████          ██   ██ ███████
 ██   ██      ██              ██ ██   ██            ██    ████           ███████ ██ █ ██
  ██  █  ██   ██   ██    ██   ██  ██  █    ██       ██   ███             ██   ██ ██   ██
   ███    █████    ██     █████    ███     ██     ██████ ███████         ██   ██ ██   ██
```

### Generate blocks and manipulate them

```rs
use terminal_fonts::{map_block, to_block, concat_blocks, to_string};

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
```

<img src="https://user-images.githubusercontent.com/4012553/83168645-5ee2ee80-a144-11ea-8fb6-4406858dd224.png" alt="">
