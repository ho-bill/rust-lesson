# Serde JSON & Reqwest test with free API

* Get your location & IP from free API ipinfo.io
```
curl https://ipinfo.io/geo
{
  "ip": "47.251.23.118",
  "city": "Sunnyvale",
  "region": "California",
  "country": "US",
  "loc": "37.3688,-122.0363",
  "org": "AS45102 Alibaba (US) Technology Co., Ltd.",
  "postal": "94088",
  "timezone": "America/Los_Angeles",
  "readme": "https://ipinfo.io/missingauth"
```
* And get local time from free API timeapi.io : 

```
 curl https://timeapi.io/api/Time/current/zone?timeZone=America/Los_Angeles 
{
  "year": 2023,
  "month": 3,
  "day": 20,
  "hour": 19,
  "minute": 11,
  "seconds": 50,
  "milliSeconds": 672,
  "dateTime": "2023-03-20T19:11:50.6727633",
  "date": "03/20/2023",
  "time": "19:11",
  "timeZone": "America/Los_Angeles",
  "dayOfWeek": "Monday",
  "dstActive": true
}
```
* Run this app in different country will get local time.
* In ubuntu, you have to install libssl-dev for building app.

```
cargo run

Checking IP Address from https://ipinfo.io/geo 
HTTP GET [OK]
IP: "47.251.23.18"
Country: "Sunnyvale"
Country: "US"

HTTP GET [OK]
IPv4 Addr:  "47.251.23.18"

HTTP GET [OK]
Country :  "US"

HTTP GET [OK]
Time zone :  "America/Los_Angeles"

HTTP GET [OK]
Date :  "03/20/2023"

HTTP GET [OK]
Time :  "19:26"
```
