use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref CMAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        m.insert('0', r#"  ███  
 █  ██ 
██   ██
██   ██
██   ██
 ██  █ 
  ███  "#);
        m.insert('1', r#"   ██  
  ███  
   ██  
   ██  
   ██  
   ██  
 ██████"#);
        m.insert('2', r#" █████ 
██   ██
    ███
  ████ 
 ████  
███    
███████"#);
        m.insert('3', r#" ██████
    ██ 
   ██  
  ████ 
     ██
██   ██
 █████ "#);
        m.insert('4', r#"   ███ 
  ████ 
 ██ ██ 
██  ██ 
███████
    ██ 
    ██ "#);
        m.insert('5', r#"██████ 
██     
██████ 
     ██
     ██
██   ██
 █████ "#);
        m.insert('6', r#"  ████ 
 ██    
██     
██████ 
██   ██
██   ██
 █████ "#);
        m.insert('7', r#"███████
██   ██
    ██ 
   ██  
  ██   
  ██   
  ██   "#);
        m.insert('8', r#" ████  
██   █ 
███  █ 
 ████  
█  ████
█    ██
 █████ "#);
        m.insert('9', r#" █████ 
██   ██
██   ██
 ██████
     ██
    ██ 
 ████  "#);
        m.insert('?', r#" █████ 
██   ██
   ███ 
  ██   
       
  ██   
  ██   "#);
        m.insert('!', r#"   ██  
   ██  
   ██  
   ██  
       
   ██  
   ██  "#);
        m.insert('#', r#"       
  █ █  
 █████ 
  █ █  
 █████ 
  █ █  
       "#);
        m.insert(':', r#"       
  ██   
  ██   
       
       
  ██   
  ██   "#);
        m.insert('+', r#"       
   █   
   █   
 █████ 
   █   
   █   
       "#);
        m.insert('-', r#"       
       
       
 █████ 
       
       
       "#);
        m.insert('*', r#"       
   █   
 █ █ █ 
  ███  
 █ █ █ 
   █   
       "#);
        m.insert('/', r#"      █
     █ 
    █  
   █   
  █    
 █     
█      "#);
        m.insert('=', r#"       
       
 █████ 
       
 █████ 
       
       "#);
        m.insert(' ', r#"       
       
       
       
       
       
       "#);
        m.insert('A', r#"  ███  
 ██ ██ 
██   ██
██   ██
███████
██   ██
██   ██"#);
        m.insert('B', r#"██████ 
██   ██
██   ██
██████ 
██   ██
██   ██
██████ "#);
        m.insert('C', r#"  ████ 
 ██  ██
██     
██     
██     
 ██  ██
  ████ "#);
        m.insert('D', r#"█████  
██  ██ 
██   ██
██   ██
██   ██
██  ██ 
█████  "#);
        m.insert('E', r#"███████
██     
██     
██████ 
██     
██     
███████"#);
        m.insert('F', r#"███████
██     
██     
██████ 
██     
██     
██     "#);
        m.insert('G', r#"  █████
 ██    
██     
██  ███
██   ██
 ██  ██
  █████"#);
        m.insert('H', r#"██   ██
██   ██
██   ██
███████
██   ██
██   ██
██   ██"#);
        m.insert('I', r#" ██████
   ██  
   ██  
   ██  
   ██  
   ██  
 ██████"#);
        m.insert('J', r#"     ██
     ██
     ██
     ██
██   ██
██   ██
 █████ "#);
        m.insert('K', r#"██   ██
██  ██ 
██ ██  
████   
█████  
██ ███ 
██  ███"#);
        m.insert('L', r#"██     
██     
██     
██     
██     
██     
███████"#);
        m.insert('M', r#"██   ██
███ ███
███████
███████
██ █ ██
██   ██
██   ██"#);
        m.insert('N', r#"██   ██
███  ██
████ ██
███████
██ ████
██  ███
██   ██"#);
        m.insert('O', r#" █████ 
██   ██
██   ██
██   ██
██   ██
██   ██
 █████ "#);
        m.insert('P', r#"██████ 
██   ██
██   ██
██   ██
██████ 
██     
██     "#);
        m.insert('Q', r#" █████ 
██   ██
██   ██
██   ██
██ ████
██  ██ 
 ████ █"#);
        m.insert('R', r#"██████ 
██   ██
██   ██
██  ███
█████  
██ ███ 
██  ███"#);
        m.insert('S', r#" ████  
██  ██ 
██     
 █████ 
     ██
██   ██
 █████ "#);
        m.insert('T', r#" ██████
   ██  
   ██  
   ██  
   ██  
   ██  
   ██  "#);
        m.insert('U', r#"██   ██
██   ██
██   ██
██   ██
██   ██
██   ██
 █████ "#);
        m.insert('V', r#"██   ██
██   ██
██   ██
███ ███
 █████ 
  ███  
   █   "#);
        m.insert('W', r#"██   ██
██   ██
██   ██
██ █ ██
███████
███ ███
██   ██"#);
        m.insert('X', r#"██   ██
███ ███
 █████ 
  ███  
 █████ 
███ ███
██   ██"#);
        m.insert('Y', r#" ██  ██
 ██  ██
 ██  ██
  ████ 
   ██  
   ██  
   ██  "#);
        m.insert('Z', r#"███████
    ███
   ███ 
  ███  
 ███   
███    
███████"#);

        m
    };
}


pub fn concat_blocks() {

}

pub fn to_block() {

}

pub fn map_block() {

}

pub fn to_string() {

}

pub fn to_block_string() {

}