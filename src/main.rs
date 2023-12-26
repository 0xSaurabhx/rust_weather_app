    use serde::Deserialize;
    use reqwest;
    
    #[derive(Debug, Deserialize)]
    struct WeatherData {
        main: Main,
        weather: Vec<Weather>,
    }
    
    #[derive(Debug, Deserialize)]
    struct Main {
        temp: f64,
    }
    
    #[derive(Debug, Deserialize)]
    struct Weather {
        description: String,
    }
    
    async fn fetch_weather(api_key: &str, city: &str) -> Result<(), reqwest::Error> {
        let url = format!(
            "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
            city, api_key
        );
        let response = reqwest::get(&url).await?;
        let weather_data: WeatherData = response.json().await?;
        println!("Current weather in {}: ", city);
        println!("Temperature: {}°C", weather_data.main.temp);
        if let Some(weather) = weather_data.weather.first() {
            println!("Condition: {}", weather.description);
        }
        Ok(())
    }
    
    #[tokio::main]
    async fn main() -> Result<(), reqwest::Error> {
        let api_key = "API_KEY"; // Replace with your actual API key
        println!("Enter the city name:");
        let mut city = String::new();
        std::io::stdin().read_line(&mut city).expect("Failed to read line");
        let city = city.trim();
        fetch_weather(api_key, city).await?;
        Ok(())
    }
    