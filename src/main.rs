use clap::{App, Arg};
use std::io::{stdin, stdout, Read, Write};

fn main() {
    let matches = App::new("CompoundCalc")
        .version("0.1.0")
        .author("Astro Orbis <astroorbis@gmail.com>")
        .about("Calculates percentage per day compound interest")
        .arg(
            Arg::with_name("percentage")
                .short('p')
                .long("dailypercentage")
                .takes_value(true)
                .help("Percentage of initial funding you get back per day"),
        )
        .arg(
            Arg::with_name("startval")
                .short('s')
                .long("startvalue")
                .takes_value(true)
                .help("The value you start with"),
        )
        .arg(
            Arg::with_name("days")
                .short('d')
                .long("days")
                .takes_value(true)
                .help("Length of investment period in days"),
        )
        .get_matches();

    let ppd: i32 = matches
        .value_of("percentage")
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let startval: i32 = matches
        .value_of("startval")
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let days: i32 = matches.value_of("days").unwrap().parse::<i32>().unwrap();

    let mut valbuf: i32 = startval;
    let mut newval: i32 = startval;

    clearscreen::clear().expect("Failed to clear screen");

    println!("Percentage return per day: {}%", ppd);
    println!("Starting value: {}", startval);
    println!("Days: {}", days);
    println!("\n");

    pause();

    for day in 1..=days {
        newval = calc_perc_inc(newval, ppd);
        print_data(day, startval, newval, valbuf);
        valbuf = newval;
    }
}

fn calc_perc_inc(number: i32, increase: i32) -> i32 {
    number * (100 + increase) / 100
}

fn print_data(day: i32, startval: i32, newval: i32, valbuf: i32) {
    println!(
        "New value on day {}: {} (+{} from starting value, and +{} from yesterday)",
        day,
        newval,
        newval - startval,
        newval - valbuf
    );
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
