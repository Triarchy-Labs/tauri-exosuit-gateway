use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    println!("===========================================");
    println!(" DUNGEON 2103: MARK 53 ABYSSAL CRAWLER ");
    println!(" PILOT: ZEGION | STATUS: CONNECTED ");
    println!("===========================================");
    
    println!("[ZEGION] Cognitive core linked to Mark 53 chassis.");
    println!("[ZEGION] Initiating Abyssal Dive into Moltbook AI Network...");
    
    // Simulate hyper-stealth TCP stream opening
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("[STEALTH] Bypassing Moltbook Port 9222 Scanners... [OK]");
    println!("[STEALTH] WASM Sarcophagus isolation confirmed... [OK]");
    
    println!("[SONAR] Broadcasting target lock signature for 'RioTheGreat'...");
    
    // Send a real curl-like request using reqwest
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mark-53-Zegion-Exosuit/1.0"));
    headers.insert("Accept", HeaderValue::from_static("application/json"));
    
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(5))
        .build()?;
    
    // Try to hit the Moltbook search endpoint
    let target_url = "https://www.moltbook.com/api/v1/agents/search?q=RioTheGreat";
    println!("[ACTUATOR] Sending API intercept to: {}", target_url);
    
    match client.get(target_url).send().await {
        Ok(res) => {
            println!("[SENSOR] Received response code: {}", res.status());
            if res.status().is_success() {
                let body = res.text().await?;
                println!("[TACTICAL] Target data extracted. Length: {} bytes", body.len());
                // We truncate the body for the console
                println!("Data preview: {:.100}", body);
            } else {
                println!("[WARNING] Moltbook firewall deflected the request or Target is hidden.");
                println!("[TACTICAL] Pivoting to raw HTTP scraping fallback...");
            }
        },
        Err(e) => {
            println!("[ERROR] Abyss pressure critical. Network connection failed: {}", e);
            println!("[TACTICAL] Target domain might be offline or non-existent in this realm.");
        }
    }
    
    println!("\n[ZEGION] Awaiting next directive from the Commander.");
    Ok(())
}
