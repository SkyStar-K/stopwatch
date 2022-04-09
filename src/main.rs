use std::io;
use std::thread;
use std::time::Duration;
use chrono::NaiveTime;

fn main() {
    let mut watch_hh: u32 = 0;
    let mut watch_mm: u32 = 0;
    let mut watch_ss: u32 = 0;

    let mut stopwatch_hh = String::new();
    let mut stopwatch_mm = String::new();
    let mut stopwatch_ss = String::new();
    
    std::io::stdin().read_line(&mut stopwatch_hh).unwrap();
    let mut stopwatch_hh: u32 = stopwatch_hh.trim().parse().unwrap();
    std::io::stdin().read_line(&mut stopwatch_mm).unwrap();
    let mut stopwatch_mm: u32 = stopwatch_mm.trim().parse().unwrap();
    std::io::stdin().read_line(&mut stopwatch_ss).unwrap();
    let mut stopwatch_ss: u32 = stopwatch_ss.trim().parse().unwrap();

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

            thread::sleep(Duration::from_secs(1));
        }
    });
    println!("{}", stopwatch_ss);
    loop {
        let watch = NaiveTime::from_hms(watch_hh, watch_mm, watch_ss);
        
        println!("{:?} - WATCH", watch);
        
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