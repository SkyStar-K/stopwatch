use std::{
    io::{self, Write},
    thread,
    time::Duration,
};
use chrono::{NaiveTime};

fn main() {
    let mut watch_hh: u32 = 0;
    let mut watch_mm: u32 = 0;
    let mut watch_ss: u32 = 0;

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let mut stopwatch_hh: u32 = input[0..2].trim().parse().unwrap();
    let mut stopwatch_mm: u32 = input[2..4].trim().parse().unwrap();
    let mut stopwatch_ss: u32 = input[4..6].trim().parse().unwrap();

    let handle = thread::spawn(move|| {

        loop {
            let stopwatch = NaiveTime::from_hms(stopwatch_hh, stopwatch_mm, stopwatch_ss);
            
            print!("{:?} - STOPWATCH\r", stopwatch);
            
            io::stdout().flush().unwrap();

            if stopwatch_hh == 0 && stopwatch_mm == 0 && stopwatch_ss == 0 {
                break;
            } else if stopwatch_mm == 0 && stopwatch_ss == 0 {
                stopwatch_hh -= 1;
                stopwatch_mm = 59;
                stopwatch_ss = 59;
            } else if stopwatch_ss == 0 {
                stopwatch_mm -= 1;
                stopwatch_ss = 59;
            } else {
                stopwatch_ss -= 1;
            }

            thread::sleep(Duration::from_secs(1));
        }
    });

    loop {
        let watch = NaiveTime::from_hms(watch_hh, watch_mm, watch_ss);

        print!("\r{:?} - WATCH ", watch);

        io::stdout().flush().unwrap();

        if watch_mm == 59 && watch_ss == 59 {
            watch_hh += 1;
            watch_mm = 0;
            watch_ss = 0;
        } else if watch_ss == 59 {
            watch_mm += 1;
            watch_ss = 0;
        } else {
            watch_ss += 1;
        }
        
        thread::sleep(Duration::from_secs(1));
    }
    handle.join().unwrap();
}