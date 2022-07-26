use clap::{App, Arg};

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

    let ppd: f32 = matches
        .value_of("percentage")
        .unwrap()
        .parse::<f32>()
        .unwrap();
    let startval: f32 = matches
        .value_of("startval")
        .unwrap()
        .parse::<f32>()
        .unwrap();
    let days: i32 = matches.value_of("days").unwrap().parse::<i32>().unwrap();

    let mut valbuf: f32 = startval;
    let mut newval: f32 = startval;

    clearscreen::clear().expect("Failed to clear screen");

    println!("Percentage return per day: {}%", ppd);
    println!("Starting value: {}", startval);
    println!("Days: {}", days);
    println!("\n");

    for day in 1..=days {
        newval = calc_perc_inc(newval, ppd);
        print_data(day, startval, newval, valbuf);
        valbuf = newval;
    }
}

fn calc_perc_inc(number: f32, increase: f32) -> f32 {
    number * (100.0 + increase) / 100.0
}

fn print_data(day: i32, startval: f32, newval: f32, valbuf: f32) {
    println!(
        "New value on day {}: {} (+{} from starting value, and +{} from yesterday)",
        day,
        newval,
        newval - startval,
        newval - valbuf
    );
}