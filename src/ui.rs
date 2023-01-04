use std::io::{self, Write};

pub struct Option {
    pub name: String,
    pub value: i32
}

pub fn console_clear() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn header_start_no_clear() {
    println!("{}\n", "---".to_string().repeat(20));
}

pub fn header_start() {
    console_clear();
    println!("{}\n", "---".to_string().repeat(20));
}

pub fn header_end() {
    println!("\n{}", "---".to_string().repeat(20));
}

pub fn prompt(ask: &str) -> String {
    print!("{}", ask);
    io::stdout().flush().unwrap();

    let mut value = "".to_string();
    io::stdin().read_line(&mut value).unwrap();

    value
}

pub fn hprompt(ask: &str) -> String {
    header_start();
    println!("{}", ask);
    header_end();

    let res = prompt("");
	console_clear();

    res
}

pub fn promptln(ask: &str) -> String {
    prompt(&format!("{}\n", ask))
}

pub fn prompt_options(ask: &str, options: &[Option]) -> i32 {
    header_start();
    println!("{}", ask);

    for option in options {
        println!("{}) {}", option.value, option.name);
    }

    header_end();

    let res = prompt("").trim().parse().unwrap();
    console_clear();

    res
}

pub fn prompt_options_no_clear(ask: &str, options: &[Option]) -> i32 {
    println!("{}", ask);

    for option in options {
        println!("{}) {}", option.value, option.name);
    }

    header_end();

    let res = prompt("").trim().parse().unwrap();

    res
}