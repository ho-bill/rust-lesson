use online::check;
use std::process;
use netapi::netapi::apicaller;

fn main() {
    println!("Checking IP Address from https://ipinfo.io/geo ");
    if !check(Some(5)).is_ok() {
        eprintln!("Network error, exit..");
        process::exit(4);
    } 
    

    apicaller::APICaller
    ::new(String::new()).getip();

    apicaller::APICaller
        ::new("https://ipinfo.io/geo".to_string())
        .json_path("ip".to_string())
        .premsg("IPv4 Addr: ".to_string())
        .call();
    
    apicaller::APICaller
        ::new("https://ipinfo.io/geo".to_string())
        .json_path("country".to_string())
        .premsg("Country : ".to_string())
        .call();
    
    let timezone = apicaller::APICaller
        ::new("https://ipinfo.io/geo".to_string())
        .json_path("timezone".to_string())
        .premsg("Time zone : ".to_string())
        .value();
    let mut timezone_url = "https://timeapi.io/api/Time/current/zone?timeZone=".to_string();  
    timezone_url  = format!("{}{}", timezone_url, timezone);  
    //println!("Timezone: {}", timezone_url);   
    apicaller::APICaller
        ::new(timezone_url.clone())
        .json_path("date".to_string())
        .premsg("Date : ".to_string())
        .call();
    
    apicaller::APICaller
        ::new(timezone_url.clone())
        .json_path("time".to_string())
        .premsg("Time : ".to_string())
        .call();


}