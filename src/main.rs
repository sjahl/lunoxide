use chrono::{prelude::*, Duration, Utc};
use clap::Parser;
use std::fmt;

#[derive(Parser, Debug)]
#[command(name = "lunoxide")]
#[command(about = "Moon phases in your terminal", long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    phase: bool,

    #[arg(short, long, default_value_t = 0)]
    forecast: u8,
}

enum MoonPhase {
    NewMoon,
    WaxingCrescent,
    FirstQuarter,
    WaxingGibbous,
    FullMoon,
    WaningGibbous,
    LastQuarter,
    WaningCrescent,
    Unknown,
}

impl From<&str> for MoonPhase {
    fn from(value: &str) -> Self {
        match value {
            "Waxing Crescent" => Self::WaxingCrescent,
            "First Quarter" => Self::FirstQuarter,
            "Waxing Gibbous" => Self::WaxingGibbous,
            "Full Moon" => Self::FullMoon,
            "Waning Gibbous" => Self::WaningGibbous,
            "Last Quarter" => Self::LastQuarter,
            "Waning Crescent" => Self::WaningCrescent,
            "New Moon" => Self::NewMoon,
            _ => Self::Unknown,
        }
    }
}

impl fmt::Display for MoonPhase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MoonPhase::WaxingCrescent => write!(f, "🌒, Waxing Crescent"),
            MoonPhase::FirstQuarter => write!(f, "🌓, First Quarter"),
            MoonPhase::WaxingGibbous => write!(f, "🌔, Waxing Gibbous"),
            MoonPhase::FullMoon => write!(f, "🌕, Full Moon"),
            MoonPhase::WaningGibbous => write!(f, "🌖, Waning Gibbous"),
            MoonPhase::LastQuarter => write!(f, "🌗, Last Quarter"),
            MoonPhase::WaningCrescent => write!(f, "🌘, Waning Crescent"),
            MoonPhase::NewMoon => write!(f, "🌑, New Moon"),
            MoonPhase::Unknown => write!(f, "MoonPhase::Unknown"),
        }
    }
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
fn current_phase(moon_age: f64) -> MoonPhase {
    let fraction = ((moon_age / 29.53059) * 100.0).round() as i64;
    match fraction {
        0..=1 | 99..=100 => MoonPhase::NewMoon,
        2..=23 => MoonPhase::WaxingCrescent,
        24..=26 => MoonPhase::FirstQuarter,
        27..=48 => MoonPhase::WaxingGibbous,
        49..=51 => MoonPhase::FullMoon,
        52..=73 => MoonPhase::WaningGibbous,
        74..=76 => MoonPhase::LastQuarter,
        77..=98 => MoonPhase::WaningCrescent,
        _ => MoonPhase::Unknown,
    }
}

fn main() {
    let args = Args::parse();

    let current_date: DateTime<Local> = Local::now();

    if args.phase {
        let moon_age = calc_moon_age(current_date);
        println!("The current moon phase is: {}", &current_phase(moon_age));
    }

    match args.forecast {
        1..=30 => {
            println!(
                "Calculating the moon phase forecast for {} days...",
                args.forecast
            );
            for day in 0..args.forecast {
                let moon_age = calc_moon_age(current_date + Duration::days(i64::from(day)));
                println!("{}", &current_phase(moon_age));
            }
        }
        31.. => println!("i only fetch forecasts of up to 30 days."),
        0 => (),
    };
}
