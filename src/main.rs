use std::thread::sleep;
use std::time::Duration;
use chrono::NaiveTime;

fn main() {
    let mut watch_hh: u32 = 0;
    let mut watch_mm: u32 = 0;
    let mut watch_ss: u32 = 0;

    loop {
        let time = NaiveTime::from_hms(watch_hh, watch_mm, watch_ss);

        println!("{:?}", time);

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
        
        sleep(Duration::from_secs(1));
    }
}