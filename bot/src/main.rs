use chrono::prelude::*;
use chrono::Duration;
use clap::{App, Arg};
use gcal;
use std::{collections::HashSet, process, thread};
use wintoast;

fn main() {
    let args = get_args();
    let mut client = gcal::Client::new(&args.client_id, &args.client_secret, &args.refresh_token);
    let mut finished_events: HashSet<String> = HashSet::new();
    loop {
        poll_events(&mut client, &mut finished_events, args.reminder_time);
        println!("sleeping...");
        thread::sleep(std::time::Duration::from_secs(60));
    }
}

struct Args {
    client_id: String,
    client_secret: String,
    refresh_token: String,
    reminder_time: u32,
}

fn get_args() -> Args {
    let matches = App::new("productivity-bot")
        .version("1.0")
        .author("Evan Fleming. <evan.gordon.fleming@gmail.com>")
        .about("Monitors your google calendar for upcoming events")
        .arg(
            Arg::new("client_id")
                .long("client_id")
                .short('i')
                .about("Google client id")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("client_secret")
                .long("client_secret")
                .short('s')
                .about("Google client secret")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("refresh_token")
                .long("refresh_token")
                .short('r')
                .about("Google refresh token")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("reminder_time")
                .long("reminder_time")
                .short('t')
                .about("How far in advance you want to be reminded")
                .takes_value(true),
        )
        .get_matches();
    let client_id = matches.value_of("client_id").unwrap().to_string();
    let client_secret = matches.value_of("client_secret").unwrap().to_string();
    let refresh_token = matches.value_of("refresh_token").unwrap().to_string();
    let reminder_time = matches
        .value_of("reminder_time")
        .unwrap_or("60")
        .parse::<u32>()
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing reminder_time: {}", err);
            process::exit(1);
        });
    Args {
        client_id,
        client_secret,
        refresh_token,
        reminder_time,
    }
}

fn poll_events(
    client: &mut gcal::Client,
    finished_events: &mut HashSet<String>,
    reminder_time: u32,
) {
    let time_min = Utc::now();
    let time_max = Utc::now() + Duration::seconds(reminder_time.into());
    println!("polling for events from {:?} to {:?}", time_min, time_max);
    let events: Vec<gcal::Event> = client.get_events(time_min, time_max).unwrap_or_else(|err| {
        eprintln!("Err fetching events {}", err);
        process::exit(1);
    });
    if events.is_empty() {
        println!("no events found");
        return;
    }
    for event in events {
        if finished_events.contains(&event.id) {
            println!("event already processed");
            continue;
        }
        println!("event found");
        let toast = wintoast::Toast::new(
            "Meeting Reminder",
            event.summary.as_str(),
            wintoast::Duration::Short,
            wintoast::Sound::Default,
        );
        wintoast::show(&toast).unwrap_or_else(|err| {
            eprintln!("Err generating toast {}", err);
            process::exit(1);
        });
        finished_events.insert(event.id);
    }
}
