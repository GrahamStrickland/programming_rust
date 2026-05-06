use std::collections::HashMap;
use std::collections::hash_map::Values;
use std::fmt;
use std::fs::File;
use std::io::{self, Error, Write, stdin};

use rand::RngExt;

#[derive(Clone, Debug)]
struct Room {
    name: String,
    hiding_spots: HashMap<String, String>,
}

impl Room {
    fn new(name: String, hiding_spots: HashMap<String, String>) -> Room {
        Room { name, hiding_spots }
    }

    fn hiding_spots(&self) -> Values<'_, String, String> {
        self.hiding_spots.values()
    }
}

struct Player {
    name: String,
    room: Room,
}

impl Player {
    fn new(name: String, room: Room) -> Player {
        Player { name, room }
    }

    fn location(&self) -> Room {
        self.room.clone()
    }
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "room {}, hiding_spots: {:?}",
            self.name, self.hiding_spots
        )
    }
}

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

    let input_lines = vec![
        String::from(
            "
    // Read some data, one line at a time.
",
        ),
        String::from(
            "
    for line in input_lines {
",
        ),
        String::from(
            "
        let trimmed = trim_comments_and_whitespace(line);
",
        ),
        String::from(
            "
        if trimmed.is_empty() {
",
        ),
        String::from(
            "
            // Jump back to the top of the loop and
",
        ),
        String::from(
            "
            // move on to the next line of input.
",
        ),
        String::from(
            "
            continue;
",
        ),
        String::from(
            "
        }
",
        ),
        String::from(
            "
    }
",
        ),
    ];

    // Read some data, one line at a time.
    for line in input_lines {
        let trimmed = trim_comments_and_whitespace(line);
        if trimmed.is_empty() {
            // Jump back to the top of the loop and
            // move on to the next line of input.
            continue;
        }
        println!("{}", trimmed);
    }

    let keys = "keys";
    let apartment = vec![
        Room::new(
            String::from("bedroom"),
            HashMap::from([(String::from("wardrobe"), String::from("clothes"))]),
        ),
        Room::new(
            String::from("hallway"),
            HashMap::from([(String::from("drawer"), String::from("keys"))]),
        ),
    ];

    'search: for room in apartment {
        for spot in room.hiding_spots() {
            if spot.contains(keys) {
                println!("Your keys are {} in the {}.", spot, room);
                break 'search;
            }
        }
    }

    // Find the square root of the first perfect square
    // in the series.
    let sqrt = 'outer: loop {
        let n = next_number();
        for i in 1.. {
            let square = i * i;
            if square == n {
                // Found a square root.
                break 'outer i;
            }
            if square > n {
                // `n` isn't a perfect square, try the next
                break;
            }
        }
    };

    println!("{}", sqrt);

    f();

    let filename = "hello.txt";
    let _ = write_hello(filename);

    let x = gcd(1302, 462); // function call

    println!("{}", x);

    let player = Player::new(
        String::from("John Doe"),
        Room::new(
            String::from("bedroom"),
            HashMap::from([(String::from("wardrobe"), String::from("clothes"))]),
        ),
    );

    let room = player.location(); // method call

    println!("Player {} is located in {}", player.name, room);

    let mut numbers: Vec<i32> = Vec::new(); // type-associated function call

    // return Vec<i32>::with_capacity(1000);   // error: something about chained comparisons

    // let ramp = (0 .. n).collect<Vec<i32>>();    // same error

    // return Vec::<i32>::with_capacity(1000); // ok, using ::<

    let n = 10;

    let ramp = (0..n).collect::<Vec<i32>>(); // ok, using ::<

    // return Vec::with_capacity(10); // ok, if the fn return type is Vec<i32>

    let ramp: Vec<i32> = (0..n).collect(); // ok, the variable's type is given
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

fn trim_comments_and_whitespace(s: String) -> String {
    let mut trimmed = String::from("");
    let mut maybe_comment = false;

    for c in s.chars() {
        if c.is_whitespace() {
            continue;
        } else if c == '/' {
            if maybe_comment {
                return trimmed;
            }
            maybe_comment = true;
            continue;
        } else {
            trimmed.push(c);
        }

        maybe_comment = false;
    }

    trimmed
}

fn next_number() -> i32 {
    rand::rng().random_range(1..=1000000)
}

fn f() {
    // return type omitted: defaults to ()
    return; // return value omitted: defaults to ()
}

fn write_hello(filename: &str) -> Result<File, Error> {
    let mut output = File::create(filename)?;

    // let output = match File::create(filename) {
    //     Ok(f) => f,
    //     Err(err) => return Err(err)
    // };

    match output.write_all(b"hello!\n") {
        Ok(_) => Ok(output),
        Err(err) => Err(err),
    }
}

// fn wait_for_process(process: &mut Process) -> i32 {
//     while true {
//         if process.wait() {
//             return process.exit_code();
//         }
//     }
// }   // error: mismatched types: expected i32, found ()

// fn serve_forever(socket: ServerSocket, handler: ServerHandler) -> ! {
//     socket.listen();
//     loop {
//         let s = socket.accept();
//         handler.handle(s);
//     }
// }

fn gcd(x: i32, y: i32) -> i32 {
    let mut a = x;
    let mut b = y;

    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}
