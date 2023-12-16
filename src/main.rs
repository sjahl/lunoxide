use std::error::Error;

use clap::Parser;

use reqwest::blocking::Client;

// Plan
// 3. request the current moon phase and return it

#[derive(Parser, Debug)]
#[command(name = "lunoxide")]
#[command(about = "Moon phases in your terminal", long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    phase: bool,

    #[arg(short, long, default_value_t = false)]
    next_full_moon: bool,

    #[arg(long, env, hide_env = false)]
    api_client_id: String,

    #[arg(long, env, hide_env = true)]
    api_client_secret: String,
}


fn main() -> Result<(), Box<dyn Error>> {

    let args = Args::parse();

    match args.phase {
        true => println!("Checking the moon phase... done."),
        false => (),
    };

    match args.next_full_moon {
        true => println!("Calculating the next full moon date.... done."),
        false => (),
    };


    let client = Client::new();
    
    let body = client.get("https://api.astronomyapi.com/api/v2/bodies/positions/moon?latitude=42.3&longitude=-70.5&elevation=1&from_date=2023-12-01&to_date=2023-12-01&time=20:34:00")
    .basic_auth(args.api_client_id, Some(args.api_client_secret))
    .send()?
    .text()?;

    println!("body = {:?}", body);

    Ok(())

}
