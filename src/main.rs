use lazy_static::lazy_static;
use reqwest::{
    header,
    Client
};
use std::net::IpAddr;

// consts
const TIMEOUT: u64 = 60 * 5;
const CLF_URL: &str = "https://api.cloudflare.com/client/v4/";
const IP_URL: &str = "https://api.ipify.org";

lazy_static! {
    static ref TOKEN: String = get_token();
    static ref ZONE_NAME: String = get_zone_name();
    static ref RECORD_NAME: String = get_record_name();
}

/// read in the token from the environment
fn get_token() -> String {
    dotenv::dotenv().ok();
    format!("Bearer {}", std::env::var("TOKEN").expect("Give an API Token"))
}

/// read in the zone name from the environment
fn get_zone_name() -> String {
    dotenv::dotenv().ok();
    std::env::var("ZONE_NAME").expect("Give a ZONE_NAME")
}

/// read in the record name from the environment
fn get_record_name() -> String {
    dotenv::dotenv().ok();
    std::env::var("RECORD_NAME").expect("Give a RECORD_NAME")
}

/// represents the info we need from a DNS record
struct Record {
    id: String,
    ip_addr: IpAddr,
}

/// gets a zone id from it's name
async fn get_zone_id(name: String) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", header::HeaderValue::from_static(&TOKEN));
    headers.insert("Content-Type", header::HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();
    let res = client.get(format!("{}zones", CLF_URL).as_str())
        .headers(headers)
        .send()
        .await?;

    let json: serde_json::Value = serde_json::from_str(res.text().await?.as_str())?;
    let zones = json["result"].as_array().unwrap();
    for zone in zones {
        if zone["name"].as_str().unwrap() == name {
            return Ok(Some(zone["id"].as_str().unwrap().to_string()));
        }
    }
    Ok(None)
}

/// get a record given the zone id and the record name
async fn get_record(zone_id: String, record_name: String) -> Result<Option<Record>, Box<dyn std::error::Error>> {

    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", header::HeaderValue::from_static(&TOKEN));
    headers.insert("Content-Type", header::HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();
    let res = client.get(format!("{}zones/{}/dns_records", CLF_URL, zone_id).as_str())
        .headers(headers)
        .send()
        .await?;

    let json: serde_json::Value = serde_json::from_str(res.text().await?.as_str())?;
    let records = json["result"].as_array().unwrap();
    for record in records {
        if record["name"].as_str().unwrap() == record_name {
            return Ok(Some(Record {
                id: record["id"].as_str().unwrap().to_string(),
                ip_addr: record["content"].as_str().unwrap().parse().unwrap(),
            }));
        }
    }
    Ok(None)
}

/// get the current ip
async fn get_ip() -> Result<IpAddr, reqwest::Error> {
    let res = reqwest::get(IP_URL)
        .await?
        .text()
        .await?;

    let ip: IpAddr = res.parse().unwrap();
    Ok(ip)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let zone_id = get_zone_id(ZONE_NAME.to_string()).await?.unwrap();
    let mut record = get_record(zone_id.clone(), RECORD_NAME.to_string()).await?.unwrap();

    println!("Starting...");
    println!("Current IP: {}", record.ip_addr);

    loop {
        let client = Client::new();

        // gets the current ip
        let ip = get_ip().await?;

        // compares with the previous ip
        if ip != record.ip_addr {

            // updates the record with the new IP
            let _res = client.put(format!("{}zones/{}/dns_records/{}", CLF_URL, zone_id, record.id).as_str())
                .header("Authorization", TOKEN.as_str())
                .header("Content-Type", "application/json")
                .json(&serde_json::json!({
                    "type": "A",
                    "name": RECORD_NAME.as_str(),
                    "content": ip.to_string(),
                    "ttl": 1,
                    "proxied": true
                }))
                .send()
                .await?;

            println!("Updated from {} -> {}", record.ip_addr, ip);
            // updates internat state... this could break if the ip is externally updated I suppose
            // someday I should fix that
            record.ip_addr = ip;
        }

        // waaaaaaiiiiit for me! I'm commmminnnnnnnn waiiittttt immmm commmingggg wittttthhhhh yyyouuuuuuu!
        tokio::time::sleep(tokio::time::Duration::from_secs(TIMEOUT)).await;
    }
}