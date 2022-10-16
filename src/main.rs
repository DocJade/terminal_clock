use chrono::*;
use clap::Parser;
use std::process::exit;
use std::{thread::sleep, time};
//use termion;

//basic

//get the time
//create numbers
//refresh display

//later:
//center in window (get the size of the window)
//color?
//seconds display toggle?
//12 hour time toggle
//documentation
//flashing separator
//toggle for flashing separator
//check if terminal size changed and adapt

//size must be between 1-5 inclusive

//maybe there is a way i can store the graphics in a table or array of some sort?

#[derive(Parser)]
struct Cli {
    //get the command line arguments
    /// clock size
    #[arg(short, long, default_value_t = 1)]
    size: u8,
}

fn main() {
    ctrlc::set_handler(move || {
        //user wants to quit
        //fix cursor
        println!("{}", termion::cursor::Show);
        //now exit
        exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let args = Cli::parse();
    if ((args.size) > 5) | ((args.size) < 1) {
        //size check
        println!("Clock size can only be between 1 and 5!");
        exit(1)
    }
    println!("{}", termion::clear::All);

    //here's our graphics
    // [string, 0-9:, height]
    const CHAR_SIZE_1: [[&str; 11]; 1] = [
        ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", ":"]
    ];
    const CHAR_SIZE_2: [[&str; 11]; 2] = [
        ["  ", " |", " ]", " ]", "|+", "[ ", "| ", "-+", "[]", "[]", " .", ],
        ["[]", " |", "[ ", " ]", " |", " ]", "[]", " |", "[]", " |", " .", ],
    ];
    const CHAR_SIZE_3: [[&str; 11]; 3] = [
        [" _ ", "   ", " _ ", " _ ", "   ", " _ ", " _ ", " _ ", " _ ", " _ ", "   ", ],
        ["| |", "  |", " _|", " _|", "|_|", "|_ ", "|_ ", "  |", "|_|", "|_|", " _ ", ],
        ["|_|", "  |", "|_ ", " _|", "  |", " _|", "|_|", "  |", "|_|", " _|", " _ ", ],
    ];
    const CHAR_SIZE_4: [[&str; 11]; 4] = [
        [" __ ", "    ", " __ ", " __ ", "    ", " __ ", " __ ", " __ ", " __ ", " __ ", "    ", ],
        ["|  |", "   |", "   |", "   |", "|  |", "|   ", "|   ", "   |", "|  |", "|  |", "  | ", ],
        ["|  |", "   |", "|--|", " --|", "|--|", "|--|", "|--|", "   |", "|--|", "|--|", "    ", ],
        ["|__|", "   |", "|__ ", " __|", "   |", " __|", "|__|", "   |", "|__|", " __|", "  | ", ],
    ];
    const CHAR_SIZE_5: [[&str; 11]; 5] = [
        [" ___ ", "     ", " ___ ", "___ ", "     ", " ___ ", " ___ ", " ___ ", " ___ ", " ___ ", "     ", ],
        ["|   |", "    |", "    |", "   |", "|   |", "|    ", "|    ", "    |", "|   |", "|   |", "     ", ],
        ["|   |", "    |", " ___|", "___|", "|___|", "|___ ", "|___ ", "    |", "|___|", "|___|", "  |  ", ],
        ["|   |", "    |", "|    ", "   |", "    |", "    |", "|   |", "    |", "|   |", "    |", "     ", ],
        ["|___|", "    |", "|___ ", "___|", "    |", " ___|", "|___|", "    |", "|___|", " ___|", "  |  ", ],
    ];

    //initialize variables for math stuffs
    //set size of chars
    let size: u16 = args.size.into();
    let total_len: u16 = &size * 5; //output is 5 digits including the colon
                                    //set the draw location's top left
                                    //shift values for alignment
    let x_shift = 0;
    let y_shift = &size;
    //get screen size
    let screen_size: (u16, u16) = termion::terminal_size().expect("couldnt get terminal size");
    //bit shift to half value,- half of the len of horizontal text size. for x align
    let x_alignment = (((screen_size.0) >> 1) - ((total_len) >> 1)) + x_shift;
    //bit shift to half value,- half of the len of vertical text size. for y align
    let y_alignment = (((screen_size.1) >> 1) - ((total_len) >> 1)) + y_shift;

    let realign = |vert: usize| {
        print! {"{goto}", goto = termion::cursor::Goto(x_alignment, y_alignment + vert as u16)}
    };

    //load the correct size into our charset
    /*
    let char_set =
    match &size {
        1 => {char_set[0].copy_from_slice(CHAR_SIZE_1)}
        2 => {CHAR_SIZE_2}
        3 => {CHAR_SIZE_3}
        x => {unreachable!("we shouldn't be able to match against anything other than 1-5")}
    };
    */

    //what id like to do is to copy the character set into an array once to avoid using a match
    //every loop, but i couldnt figure it out :(
    fn get_string_from_charset(row: usize, array_index: usize, size: i32) -> &'static str {
        if size == 1 {
            return CHAR_SIZE_1[array_index][row];
        }
        if size == 2 {
            return CHAR_SIZE_2[array_index][row];
        }
        if size == 3 {
            return CHAR_SIZE_3[array_index][row];
        }
        if size == 4 {
            return CHAR_SIZE_4[array_index][row];
        }
        if size == 5 {
            CHAR_SIZE_5[array_index][row]
        } else {
            "you should never see this"
        }
    }

    //now we loop
    //is drawin' time
    loop {
        //reset row_iter
        let mut row_iter: usize = 0;
        //get the time and throw it into a string
        let now = Local::now();
        let time_str: String = format!("{h:0>2}{m:0>2}", h = now.hour(), m = now.minute());
        //if the time is 21:12 the string will read 2112
        //now here's some goofy code
        //set the numbers
        //time number 0
        let tn0: usize = time_str.chars().next().unwrap().to_digit(10).unwrap() as usize;
        let tn1: usize = time_str.chars().nth(1).unwrap().to_digit(10).unwrap() as usize;
        let tn2: usize = time_str.chars().nth(2).unwrap().to_digit(10).unwrap() as usize;
        let tn3: usize = time_str.chars().nth(3).unwrap().to_digit(10).unwrap() as usize;
        //draw the rows
        loop {
            //first we must realign
            realign(row_iter + size as usize);
            //then print the line
            //hours
            print!("{}", get_string_from_charset(tn0, row_iter, size as i32));
            print!("{}", get_string_from_charset(tn1, row_iter, size as i32));
            //colon
            print!("{}", get_string_from_charset(10, row_iter, size as i32));
            //minutes
            print!("{}", get_string_from_charset(tn2, row_iter, size as i32));
            print!("{}", get_string_from_charset(tn3, row_iter, size as i32));
            //check if we've drawn all the rows, else increment loop count.
            row_iter += 1;
            if row_iter >= size as usize {
                break;
            }
        }
        //print a line so the text will actually appear
        println!();
        //now we sleep
        sleep(time::Duration::from_millis(1000));
    }
    //if you somehow got here something is VERY wrong
}
