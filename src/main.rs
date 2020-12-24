use std::thread::sleep;
use chrono::{Duration, Utc};

fn fmt_duration(duration: Duration) -> String {
    let hours = duration.num_hours() % 24;
    let minutes = duration.num_minutes() % 60;
    let seconds = duration.num_seconds() % 60;
    return format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
}

fn prompt(duration: Duration) -> String {
    return format!("[{}]", fmt_duration(duration));
}

fn main() {
    let start_time = Utc::now();

    let mut rl = rustyline::Editor::<()>::new();

    loop {
        let elapsed_time = Utc::now() - start_time;
        eprint!("\r[{}] Working on project...\x1b[K", fmt_duration(elapsed_time));

        let line_result = rl.readline(prompt(elapsed_time).as_str());
        match line_result {
            Ok(line) => println!("Thanks for input: {}", line),
            _ => return
        }

        sleep(std::time::Duration::from_secs(1));
    }
}
