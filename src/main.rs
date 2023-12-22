use chrono::{prelude::*, Duration};
use clap::Parser;
use reqwest::blocking::Client;
use serde_json::Value;
use std::error::Error;

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

    #[arg(long, env, hide_env = false)]
    api_client_id: String,

    #[arg(long, env, hide_env = true)]
    api_client_secret: String,
}

struct BasicAuth {
    user: String,
    password: String,
}

fn make_api_request(
    from_date: &String,
    to_date: &String,
    time: &String,
    creds: BasicAuth,
) -> Result<Value, reqwest::Error> {
    let client = Client::new();

    let url = format!("https://api.astronomyapi.com/api/v2/bodies/positions/moon?latitude=42.3&longitude=-70.5&elevation=1&from_date={from_date}&to_date={to_date}&time={time}");

    let resp: serde_json::Value = client
        .get(url)
        .basic_auth(creds.user, Some(creds.password))
        .send()?
        .json()?;

    Ok(resp)
}

fn phase_to_emoji(phase: &str) -> &str {
    match phase {
        "Waxing Gibbous" => "🌔",
        "Waning Gibbous" => "🌖",
        "Waxing Crescent" => "🌒",
        "Waning Crescent" => "🌘",
        _ => "unknown phase",
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let creds = BasicAuth {
        user: args.api_client_id,
        password: args.api_client_secret,
    };

    let current_date: DateTime<Local> = Local::now();
    let forecast_end_date: DateTime<Local> =
        current_date + Duration::days(i64::from(args.forecast));
    let date_formatted = format!("{}", current_date.format("%Y-%m-%d"));
    let f_end_date_formatted = format!("{}", forecast_end_date.format("%Y-%m-%d"));
    let time_formatted = format!("{}", current_date.format("%H:%M:%S"));

    let api_response = make_api_request(
        &date_formatted,
        &f_end_date_formatted,
        &time_formatted,
        creds,
    )?;

    let current_phase =
        &api_response["data"]["table"]["rows"][0]["cells"][0]["extraInfo"]["phase"]["string"];
    let current_phase = current_phase.as_str().unwrap_or_default();

    if args.phase {
        println!("{}", phase_to_emoji(current_phase));
    }

    // I think the data is returned in a format like this

    /*
            | 2023-12-21T20:34:00.000-05:00 | 2023-12-21T20:34:00.000-05:00 |
       moon || date | extraInfo | position ||| date | extraInfo | position ||
    */

    // for cell in first row of response data,
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

    Ok(())
}
