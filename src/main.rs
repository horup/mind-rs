use std::{thread::sleep, time::Duration, time::{Instant}};
use std::fs::File;
use std::io::{self, BufRead};
use device_query::{DeviceState, Keycode};
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::io::stdout;
use crossterm::{ExecutableCommand, cursor::MoveTo, Result, style::Print, terminal::Clear};


fn math_exercise(device_state:&mut DeviceState) {
    let a = rand::random::<u32>() % 3 + 1;
    print_top(&format!("{}\n", a)).unwrap();

    let mut success = false;
    loop {
        let keys = device_state.query_keymap();
        match a {
            1 => if keys.iter().filter(|x| **x == Keycode::Key1).count() > 0 {success = true;},
            2 => if keys.iter().filter(|x| **x == Keycode::Key2).count() > 0 {success = true;},
            _ => if keys.iter().filter(|x| **x == Keycode::Key3).count() > 0 {success = true;},
        }

        if success && keys.iter().count() == 0 {
            break;
        } else {
            sleep(Duration::from_millis(1));
        }
    }
}

fn think_of(think_of:&mut usize, questions:&Vec<String>) {

    let q = &questions.get(*think_of % questions.len()).unwrap();
    print_top(&format!("{}\n", q)).unwrap();
    loop {
        let t = *think_of+1;
        if t != *think_of {
            *think_of = t;
            break;
        }
    }

    sleep(Duration::from_secs(10));
}

fn load_questions() -> Vec<String> {
    let mut questions:Vec<String> = Vec::new();
    let file = File::open("./questions.txt").unwrap();
    let reader = io::BufReader::new(file).lines();
    for s in reader {
        questions.push(s.unwrap());
    }

    questions.shuffle(&mut thread_rng());
    questions
}

fn print_top(s:&str) -> Result<()> {
    let mut out = stdout();
    out.execute(Clear(crossterm::terminal::ClearType::All))?;
    sleep(Duration::from_millis(100));
    out.execute(MoveTo(0,0))?;
    out.execute(Print(s))?;
    out.execute(MoveTo(0,1))?;
    Ok(())
}

fn main() -> Result<()> {
    let questions = load_questions();
    let mut now = Instant::now();
    let mut device_state = DeviceState::new();
    let mut iteration = 1;
    let num = 20;
    let mut t = 0;
    loop {
        if iteration % num == 0 {
            think_of(&mut t, &questions);
            if t >= questions.len() {
                let duration = Instant::now() - now;
                now = Instant::now();
                stdout().execute(Print("Hello World"))?;
                print_top(&format!("All done! Game took {:?}. Starting a new game in 10 seconds...", duration))?;
                sleep(Duration::from_secs(10));
                t = 0;
            }
        }
        sleep(Duration::from_millis(100));
        math_exercise(&mut device_state);
        iteration+=1;
    }
}
