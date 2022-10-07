use chrono::*;

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

//size must be between 1-5 inclusive

fn main() {
    let now = Local::now();
    let hour = now.hour();
    //size 1 lol
    println!(
        "{:02}:{:02}",
        hour,
        now.minute()
    );
}
