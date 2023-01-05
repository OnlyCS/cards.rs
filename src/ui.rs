use std::io::{stdin, stdout, Write};

pub struct Option {
    pub name: &'static str,
    pub value: i32,
}

pub(super) fn promptfn() -> String {
    let mut value = String::new();
    stdout().flush().unwrap();
    stdin().read_line(&mut value).unwrap();

    value
}

macro_rules! console_clear {
    () => {
        print!("\x1B[2J\x1B[1;1H");
    };
}

macro_rules! header_start {
    () => {
        println!("{}\n", "---".to_string().repeat(20));
    };

    ($clear:expr) => {
        if !$clear {
            console_clear!();
            header_start!();
        } else {
            header_start!();
        }
    };
}

macro_rules! header_end {
    () => {
        println!("\n{}", "---".to_string().repeat(20));
    };
}

macro_rules! prompt {
	($($args:tt)*) => {{
		let formatted = std::fmt::format(format_args!($($args)*));

		print!("{}", formatted);
		let value = crate::ui::promptfn();

		value
	}};
}

macro_rules! prompt_options {
    ($ask:expr, $options:expr) => {{
        header_start!();
        println!("{}", $ask);
        for option in $options {
            println!("{}) {}", option.value, option.name);
        }
        header_end!();

        let res: i32 = prompt!("").trim().parse().unwrap();
        assert!($options.iter().any(|x| x.value == res), "E_INVALID_OPTION");

        res
    }};
}

macro_rules! prompt_headers {
	($($args:tt)*) => {{
		let formatted = std::fmt::format(format_args!($($args)*));

		header_start!();
		println!("{}", formatted);
		header_end!();

		let res = prompt!("");

		res
	}};
}

pub(crate) use console_clear;
pub(crate) use header_end;
pub(crate) use header_start;
pub(crate) use prompt;
pub(crate) use prompt_headers;
pub(crate) use prompt_options;
