use std::io::{self, Write, stdin};

fn main() {
    for i in 0..20 {
        println!("{}", i);
    }

    let range = std::ops::Range { start: 0, end: 20 };
    for i in range {
        println!("{}", i);
    }

    let mut strings: Vec<String> = error_messages();
    // for s in strings {  // each String is moved into s here...
    //     println!("{}", s);
    // }   // ...and dropped here
    // println!("{} error(s)", strings.len()); // error: use of moved value

    for rs in &strings {
        println!("String {:?} is at address {:p}.", *rs, rs);
    }

    for rs in &mut strings {
        // the type of rs is &mut String
        rs.push('\n'); // add a newline to each string
    }

    // Each call to `next_line` returns either `Some(line)`, where
    // `line` is a line of input, or `None`, if we've reached the end of
    // the input. Return the first line that starts with "answer: ".
    // Otherwise, return "answer: nothing".
    let answer = loop {
        if let Some(line) = next_line() {
            if line.starts_with("answer: ") {
                break line;
            }
        } else {
            break String::from("answer: nothing");
        }
    };

    println!("{}", answer);
}

fn error_messages() -> Vec<String> {
    vec![
        String::from("error: invalid variable"),
        String::from("error: invalid input"),
    ]
}

fn next_line() -> Option<String> {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut line = String::new();
    match stdin().read_line(&mut line) {
        Ok(_) => Some(line),
        Err(_) => None,
    }
}
