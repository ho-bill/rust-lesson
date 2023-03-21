use serde_json::Value;

pub struct APICaller {
    url: String,
    json_path: String,
    premsg: String,
}

impl APICaller {
    pub fn new(url: String) -> APICaller {
        APICaller {
            url: url,
            json_path: String::new(),
            premsg: String::new(),
        }
    }

    pub fn json_path(&mut self, json_path: String) -> &mut APICaller {
        self.json_path = json_path;
        self
    }

    pub fn premsg(&mut self, premsg: String) -> &mut APICaller {
        self.premsg = premsg;
        self
    }

    pub fn call(&self) {
        match api_getter(&self.url, &self.json_path, &self.premsg) {
            Err(e) => println!("{:?}", e),
            _ => ()
        }
    }

    pub fn value(&self) -> String {
        let value =  String::new();
        match api_value(&self.url, &self.json_path, &self.premsg) {
            Err(_e) => value,
            //just remove ["] from value
            Ok(value) => serde_json::from_str(&value).unwrap()
           
        }
    }

    pub fn getip(&self) {
        match ip_geo() {
            Err(e) => println!("{:?}", e),
            _ => ()
        }
    }
}


#[tokio::main]
async fn api_getter(url: &String, json_path: &String, premsg: &String) -> Result<(), Box<dyn std::error::Error>> {
    let curl_data = reqwest::get(url).await?;
    if curl_data.status().is_success() {
        println!("\nHTTP GET [OK]");
        let json_data: Value = serde_json::from_str(&curl_data.text().await?).expect("Failed to parse json");
        println!("{} {}",premsg , json_data[json_path]);
    } else {
        println!("Server Error: Status: {:?}", curl_data.status());
    }
    Ok(())
}

#[tokio::main]
async fn api_value(url: &String, json_path: &String, premsg: &String) -> Result<String, Box<dyn std::error::Error>> {
    let mut value = String::new();
    let curl_data = reqwest::get(url).await?;
    if curl_data.status().is_success() {
        println!("\nHTTP GET [OK]");
        let json_data: Value = serde_json::from_str(&curl_data.text().await?).expect("Failed to parse json");
        value = format!("{}",json_data[json_path]);
        println!("{} {}",premsg , value);
    } else {
        println!("Server Error: Status: {:?}", curl_data.status());
    }
    Ok(value)
}

#[tokio::main]
pub async fn ip_geo() -> Result<(), Box<dyn std::error::Error>> {
    let ip_data = reqwest::get("https://ipinfo.io/geo").await?;
    if ip_data.status().is_success() {
        println!("HTTP GET [OK]");
        let ip: Value = serde_json::from_str(&ip_data.text().await?).expect("Failed to parse json");
        println!("IP: {}", ip["ip"]);
        println!("Country: {}", ip["city"]);
        println!("Country: {}", ip["country"]);
    } else {
        println!("Server Error: Status: {:?}", ip_data.status());
    }
    Ok(())
}

#[tokio::main]
pub async fn get_datetime() -> Result<(), Box<dyn std::error::Error>> {
    let datetime_data = reqwest::get("https://timeapi.io/api/Time/current/zone?timeZone=Asia/Taipei").await?;
    if datetime_data.status().is_success() {
        println!("HTTP GET [OK]");
        let datatime: Value = serde_json::from_str(&datetime_data.text().await?).expect("Failed to parse json");
        println!("Date: {}", datatime["date"]);
        println!("Day.w: {}", datatime["dayOfWeek"]);
        println!("Time: {}", datatime["time"]);

    } else {
        println!("Server Error: Status: {:?}", datetime_data.status());
    }
    Ok(())
}
