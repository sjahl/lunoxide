use chrono::{prelude::*, Duration, Utc};
use clap::Parser;

// Plan
// 3. request the current moon phase and return it

#[derive(Parser, Debug)]
#[command(name = "lunoxide")]
#[command(about = "Moon phases in your terminal", long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    phase: bool,

    #[arg(short, long, default_value_t = 0)]
    forecast: u8,
}

#[allow(clippy::cast_precision_loss)]
fn calc_moon_age(current_date: DateTime<Local>) -> f64 {
    let known_new_moon = Utc.with_ymd_and_hms(1900, 1, 1, 0, 0, 0).unwrap();
    let duration: Duration = current_date.with_timezone(&Utc) - known_new_moon;

    // it is highly unlikely that we will overflow an i64 with the number of days since
    // jan 1 1900
    duration.num_days() as f64 % 29.53059
}

#[allow(clippy::cast_possible_truncation)]
fn current_phase(moon_age: f64) -> &'static str {
    let fraction = ((moon_age / 29.53059) * 100.0).round() as i64;
    match fraction {
        0..=1 | 99..=100 => "New Moon",
        2..=23 => "Waxing Crescent",
        24..=26 => "First Quarter",
        27..=48 => "Waxing Gibbous",
        49..=51 => "Full Moon",
        52..=73 => "Waning Gibbous",
        74..=76 => "Last Quarter",
        77..=98 => "Waning Crescent",
        _ => "unknown",
    }
}

fn phase_to_emoji(phase: &str) -> &str {
    match phase {
        "Waxing Crescent" => "🌒",
        "First Quarter" => "🌓",
        "Waxing Gibbous" => "🌔",
        "Full Moon" => "🌕",
        "Waning Gibbous" => "🌖",
        "Last Quarter" => "🌗",
        "Waning Crescent" => "🌘",
        "New Moon" => "🌑",
        _ => "unknown phase",
    }
}

fn main() {
    let args = Args::parse();

    let current_date: DateTime<Local> = Local::now();
    // let forecast_end_date: DateTime<Local> =
    // current_date + Duration::days(i64::from(args.forecast));

    let moon_age = calc_moon_age(current_date);

    if args.phase {
        println!("{}", phase_to_emoji(current_phase(moon_age)));
    }

    match args.forecast {
        1..=30 => {
            println!(
                "Calculating the moon phase forecast for {} days.... done.",
                args.forecast
            );
            for day in 0..args.forecast {
                let moon_age = calc_moon_age(current_date + Duration::days(i64::from(day)));
                println!("{}", phase_to_emoji(current_phase(moon_age)));
            }
        }
        31.. => println!("i only fetch forecasts of up to 30 days."),
        0 => (),
    };
}
