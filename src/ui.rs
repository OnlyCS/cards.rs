use std::io::{self, Write};

pub fn console_clear() {
    print!("\x1B[2J\x1B[1;1H");
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