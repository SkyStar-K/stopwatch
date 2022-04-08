use std::io;
use std::thread;
use std::time::Duration;
use chrono::NaiveTime;

fn main() {
    let mut watch_hh: u32 = 0;
    let mut watch_mm: u32 = 0;
    let mut watch_ss: u32 = 0;

    let mut stopwatch_hh = 0;
    let mut stopwatch_mm = 0;
    let mut stopwatch_ss = 0;

    let handle = thread::spawn(move|| {
        loop {
            let stopwatch = NaiveTime::from_hms(stopwatch_hh, stopwatch_mm, stopwatch_ss);

            println!("{:?} - STOPWATCH", stopwatch);

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

            thread::sleep(Duration::from_millis(100));
        }
    });

    loop {
        let watch = NaiveTime::from_hms(watch_hh, watch_mm, watch_ss);
        
        println!("{:?} - WATCH", watch);
        
        if watch_mm == 0 && watch_ss == 59 {
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