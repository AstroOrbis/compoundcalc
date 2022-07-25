use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let ppd: i32 = args[1].parse::<i32>().unwrap();
    let startval: i32 = args[2].parse::<i32>().unwrap();
    let days: i32 = args[3].parse::<i32>().unwrap();

    let mut valbuf: i32 = startval;
    let mut newval: i32 = startval;

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

fn calc_perc_inc(number: i32, increase: i32) -> i32 {
    number * (100 + increase) / 100
}

fn print_data(day: i32, startval: i32, newval: i32, valbuf: i32) {
    println!("New value on day {}: {} (+{} from starting value, and +{} from yesterday)", day, newval, newval-startval, newval-valbuf);
}