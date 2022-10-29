#![warn(
clippy::pedantic,
clippy::nursery,
clippy::unwrap_used,
clippy::expect_used,
clippy::correctness,
clippy::style,
clippy::perf,
)]

use chrono::{Local, Timelike};
use clap::Parser;
use std::process::exit;
use std::{thread::sleep, time};

//TODO
//generate fonts on the fly
//center in window (get the size of the window)
//color?
//seconds display toggle?
//12 hour time toggle
//documentation
//flashing separator
//toggle for flashing separator
//check if terminal size changed and adapt

//currently, size must be between 1-5 inclusive

#[derive(Parser)]
struct Cli {
    //get the command line arguments
    //clock size
    #[arg(short, long, default_value_t = 1)]
    size: u8,
}

//here's our graphics
// [string, 0-9:, height]
const CHAR_SIZE_1: [[&str; 11]; 1] = [["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", ":"]];
const CHAR_SIZE_2: [[&str; 11]; 2] = [
    [
        "  ", " |", " ]", " ]", "|+", "[ ", "| ", "-+", "[]", "[]", " .",
    ],
    [
        "[]", " |", "[ ", " ]", " |", " ]", "[]", " |", "[]", " |", " .",
    ],
];
const CHAR_SIZE_3: [[&str; 11]; 3] = [
    [
        " _ ", "   ", " _ ", " _ ", "   ", " _ ", " _ ", " _ ", " _ ", " _ ", "   ",
    ],
    [
        "| |", "  |", " _|", " _|", "|_|", "|_ ", "|_ ", "  |", "|_|", "|_|", " _ ",
    ],
    [
        "|_|", "  |", "|_ ", " _|", "  |", " _|", "|_|", "  |", "|_|", " _|", " _ ",
    ],
];
const CHAR_SIZE_4: [[&str; 11]; 4] = [
    [
        " __ ", "    ", " __ ", " __ ", "    ", " __ ", " __ ", " __ ", " __ ", " __ ", "    ",
    ],
    [
        "|  |", "   |", "   |", "   |", "|  |", "|   ", "|   ", "   |", "|  |", "|  |", "  | ",
    ],
    [
        "|  |", "   |", "|--|", " --|", "|--|", "|--|", "|--|", "   |", "|--|", "|--|", "    ",
    ],
    [
        "|__|", "   |", "|__ ", " __|", "   |", " __|", "|__|", "   |", "|__|", " __|", "  | ",
    ],
];
const CHAR_SIZE_5: [[&str; 11]; 5] = [
    [
        " ___ ", "     ", " ___ ", "___ ", "     ", " ___ ", " ___ ", " ___ ", " ___ ", " ___ ",
        "     ",
    ],
    [
        "|   |", "    |", "    |", "   |", "|   |", "|    ", "|    ", "    |", "|   |", "|   |",
        "     ",
    ],
    [
        "|   |", "    |", " ___|", "___|", "|___|", "|___ ", "|___ ", "    |", "|___|", "|___|",
        "  |  ",
    ],
    [
        "|   |", "    |", "|    ", "   |", "    |", "    |", "|   |", "    |", "|   |", "    |",
        "     ",
    ],
    [
        "|___|", "    |", "|___ ", "___|", "    |", " ___|", "|___|", "    |", "|___|", " ___|",
        "  |  ",
    ],
];

//what id like to do is to copy the character set into an array once to avoid using a match
//every loop, but i couldnt figure it out :(
fn get_string_from_charset(row: usize, array_index: u16, size: i32) -> &'static str {
    if size == 1 {
        return CHAR_SIZE_1[array_index as usize][row];
    }
    if size == 2 {
        return CHAR_SIZE_2[array_index as usize][row];
    }
    if size == 3 {
        return CHAR_SIZE_3[array_index as usize][row];
    }
    if size == 4 {
        return CHAR_SIZE_4[array_index as usize][row];
    }
    if size == 5 {
        CHAR_SIZE_5[array_index as usize][row]
    } else {
        panic!("This should be impossible to reach!");
    }
}

fn realign(x_alignment: u16,y_alignment: u16,vert: u16) {
    print!(
        "{goto}",
        goto = termion::cursor::Goto(
            x_alignment,
            y_alignment + vert)
    );
}

fn get_terminal_size() -> (u16, u16) {
    if let Ok(output) = termion::terminal_size(){
        //value is safe, return
        output
    }else {
        //value isn't safe! lets get outta here!
        panic!("Couldn't get terminal size!")
    }
}

fn main() {
    match ctrlc::set_handler(move || {
        //user wants to quit!
        //fix cursor
        println!("{}", termion::cursor::Show);
        //now exit.
        exit(0);
    }) {
        Ok(_ok) => {}
        Err(error) => {panic!("Unable to set ctrl + c handler! : {}", error)}
    }

    let args = Cli::parse();
    if ((args.size) > 5) | ((args.size) < 1) {
        //ensure we have the font size requested
        println!("Clock size can only be between 1 and 5!");
        exit(1)
    }
    assert!(args.size <= 5);
    assert!(args.size >= 1);
    //blank out the terminal
    println!("{}", termion::clear::All);
    //initialize variables for screen alignment
    //set size of chars
    let size: u16 = args.size.into();
    let total_len: u16 = &size * 5; //output is 5 digits including the colon
    let x_shift = 0;    //set the draw location's top left
    let y_shift = &size;    //shift values for alignment
    //get screen size
    let screen_size: (u16, u16) = get_terminal_size();
    //bit shift to half value,- half of the len of horizontal text size. for x align
    let x_alignment = (((screen_size.0) >> 1) - ((total_len) >> 1)) + x_shift;
    //bit shift to half value,- half of the len of vertical text size. for y align
    let y_alignment = (((screen_size.1) >> 1) - ((total_len) >> 1)) + y_shift;



    //is drawin' time
    //now we loop
    loop {
        //reset row_iter
        let mut row_iter: u16 = 0;
        //get the time
        let now = Local::now();
        //set the numbers
        let hour_tens: usize = (now.hour() / 10) as usize;      //#_:__
        let hour_ones: usize = (now.hour() % 10) as usize;      //_#:__
        let minute_tens: usize = (now.minute() / 10) as usize;  //__:#_
        let minute_ones: usize = (now.minute() % 10) as usize;  //__:_#
        //draw the rows
        loop {
            //first we must realign
            realign(x_alignment,y_alignment,row_iter + size);
            //then print the line
            //hours
            //tens
            print!(
                "{}",
                get_string_from_charset(hour_tens, row_iter, i32::from(size))
            );
            //ones
            print!(
                "{}",
                get_string_from_charset(hour_ones, row_iter, i32::from(size))
            );
            //colon
            print!("{}", get_string_from_charset(10, row_iter, i32::from(size)));
            //minutes
            //tens
            print!(
                "{}",
                get_string_from_charset(minute_tens, row_iter, i32::from(size))
            );
            //ones
            print!(
                "{}",
                get_string_from_charset(minute_ones, row_iter, i32::from(size))
            );
            row_iter += 1;
            //check if we've drawn all the rows
            if row_iter >= size {
                //done drawing time
                break;
            }
        }
        //print a line so the text will actually appear
        println!();
        //now we sleep until its time to update again
        sleep(time::Duration::from_millis(1000));
    }
}
