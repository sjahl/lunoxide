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

fn calc_moon_age(current_date: DateTime<Local>) -> f64 {
    let known_new_moon = Utc.with_ymd_and_hms(1900, 1, 1, 0, 0, 0).unwrap();
    let duration: Duration = current_date.with_timezone(&Utc) - known_new_moon;

    // it is highly unlikely that we will overflow an i64 with the number of days since
    // jan 1 1900
    #[allow(clippy::cast_precision_loss)]
    {
        duration.num_days() as f64 % 29.53059
    }
}

fn current_phase(moon_age: f64) -> &'static str {
    println!("{moon_age:?}");
    match moon_age {
        0.0..=24.9 => "Waxing Crescent",
        25.0..=25.9 => "First Quarter",
        26.0..=49.9 => "Waxing Gibbous",
        50.0..=50.9 => "Full Moon",
        51.0..=74.9 => "Waning Gibbous",
        75.0..=75.9 => "Last Quarter",
        76.0..=100.0 => "Waning Crescent",
        _ => "unknown",
    }
}

fn phase_to_emoji(phase: &str) -> &str {
    match phase {
        "Waxing Crescent" => "🌒",
        "First Quarter" => "🌓",
        "Waxing Gibbous" => "🌔",
        "Full" => "🌕",
        "Waning Gibbous" => "🌖",
        "Last Quarter" => "🌗",
        "Waning Crescent" => "🌘",
        "New" => "🌑",
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
        println!(
            "{}",
            phase_to_emoji(current_phase((&moon_age / 29.53059) * 100.0))
        );
    }

    match args.forecast {
        1..=30 => {
            println!(
                "Calculating the moon phase forecast for {} days.... done.",
                args.forecast
            );
        }
        31.. => println!("i only fetch forecasts of up to 30 days."),
        0 => (),
    };
}
