use reqwest::Error;
use serde::Deserialize;
use std::env;
use dotenv::dotenv;


#[derive(Deserialize, Debug)]
struct WeatherResponse {
    main: Main,
    weather: Vec<Weather>,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: u8,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}


#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>>{
   dotenv().ok();

   let args : Vec<String> = env::args().collect();

   if args.len()<3{
    eprintln!("Usage:weather_cli <CITY_NAME> <UNIT (C/F)>");
    return  Ok(());
   }

   let city = &args[1];
   let unit = &args[2].to_uppercase();
   if unit!="C" && unit !="F"{
    eprintln!("Inavlid unit ! Use 'C' for celsisus or 'F' for Fahrenheit");
    return Ok(());
   }

     let api_key = env::var("OPENWEATHER_API_KEY")
        .expect("API key not found in .env file");

      let units = if unit == "C" { "metric" } else { "imperial" };

      let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units={}",
        city, api_key, units
    );
    let response = match reqwest::get(&url).await?.json::<WeatherResponse>().await {
        Ok(data) => data,
        Err(_) => {
            eprintln!("Error fetching data for city: {}", city);
            return Ok(());
        }
    };

        println!("-------------------------");
    println!("Weather in {}:", response.name);
    println!("Temperature: {}°{}", response.main.temp, if unit == "C" { "C" } else { "F" });
    println!("Humidity: {}%", response.main.humidity);
    println!("Condition: {}", response.weather[0].description);
    println!("-------------------------");

    Ok(())



}