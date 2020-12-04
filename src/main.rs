use std::{thread::sleep, time::Duration, time::{Instant}};
use std::fs::File;
use std::io::{self, BufRead};
use device_query::{DeviceState, Keycode};
use rand::thread_rng;
use rand::seq::SliceRandom;


fn math_exercise(device_state:&mut DeviceState) {
    let a = rand::random::<u32>() % 3 + 1;
    println!("{}", a);

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
        }
    }
}

fn think_of(think_of:&mut usize, questions:&Vec<String>) {

    println!("{}", &questions.get(*think_of % questions.len()).unwrap());
    println!("------");
    loop {
        let t = *think_of+1;
        if t != *think_of {
            *think_of = t;
            break;
        }
    }

    sleep(Duration::from_secs(10));
}

fn main() {
    let mut questions:Vec<String> = Vec::new();
    let file = File::open("./questions.txt").unwrap();
    let reader = io::BufReader::new(file).lines();
    for s in reader {
        questions.push(s.unwrap());
    }

    questions.shuffle(&mut thread_rng());

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
                println!("All done! Game took {:?}", duration);
                t = 0;
            }
        }
        math_exercise(&mut device_state);
        iteration+=1;
    }
}
