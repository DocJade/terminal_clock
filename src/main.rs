use chrono::*;
use clap::Parser;
use std::process::exit;
use std::{thread::sleep, time};
use termion;

//basic

//get the time
//create numbers
//refresh display

//later
//center in window (get the size of the window)
//color?
//seconds display toggle?
//12 hour time toggle
//documentation
//flashing separator
//toggle for flashing separator

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
    let args = Cli::parse();
    if ((&args.size as &u8) > &5) | ((&args.size as &u8) < &1) {
        println!("Clock size can only be between 1 and 5!");
        exit(1)
    }
    println!("text size is {}",args.size);
    let t_size = termion::terminal_size().expect("couldnt get terminal size");
    println!("term size is x:{}, y:{}",t_size.0,t_size.1);
    sleep(time::Duration::from_secs(5));
    println!("{}",termion::clear::All);

    //enter the loop depending on size
    match &args.size {
        1 => size_1(),
        _ => {} //catch all no op.
    }

    exit(1) //if you somehow got here something is wrong
}
//this feels dumb but i dont know how else to do it yet

fn size_1(){
    //add switch here later //let out_length = if  { }
    let out_length = 5; //temp bc output should always be 5 for now:"00:00"
    let out_height = 1;
    //align the text to the middle of the window
    let screen = termion::terminal_size().expect("couldnt get terminal size");
    //bit shift to half value,- half of the len of horizontal text size. for x align
    let x_alignment = ((screen.0)>>1)-((out_length)>>1);
    //bit shift to half value,- half of the len of vertical text size. for y align
    let y_alignment = (screen.1)>>1-((out_height)>>1);

    loop {
        //get the time
        let now = Local::now();
        //print time
        println!("{clear}{goto}{hour:02}:{minute:02}",
                 clear = termion::clear::All,
                 goto  = termion::cursor::Goto(x_alignment, y_alignment), //todo fix this making the window scroll
                 hour = now.hour(),
                 minute = now.minute());
        //todo add second support with switch
        //and finally sleep for one second.
        sleep(time::Duration::from_millis(1000)); //todo make this change based on seconds flag
        //this is the simplest version. just a print command
    }
}