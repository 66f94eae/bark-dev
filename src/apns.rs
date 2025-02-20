// MIT License
//
// Copyright (c) 2025 66f94eae
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.


use crate::msg::Msg;
use std::{collections::{HashMap, HashSet}, io::Error};

const APNS_HOST: &str = "api.push.apple.com";

/// send msg to devices
/// 
/// return: None if success, or a vector of failed devices
pub fn send<T>(msg: &Msg, topic: &str, token: &str, devices: T) -> Option<Vec<String>> 
where 
    T: IntoIterator<Item = String>
{
    let rt: Result<tokio::runtime::Runtime, Error> = tokio::runtime::Runtime::new();
    match rt {
        Ok(rt) => {
            rt.block_on(
                async_send(msg, topic, token, devices)
            )
        },
        Err(e)=> {
            eprintln!("send failed: {}", e);
            Some(devices.into_iter().map(|device| device.to_string()).collect())
        }
    }
}

/// async send to devices
/// 
/// return: None if success, or a vector of failed devices
pub async fn async_send<T>(msg: &Msg, topic: &str, token: &str, devices: T) -> Option<Vec<String>> 
where 
    T: IntoIterator<Item = String>
{
    let devices: Vec<String> = devices.into_iter().collect::<Vec<_>>(); 
    match do_send(msg, topic, token, devices.clone().into_iter()).await {
        Ok(results) => {
            if results.is_empty() {
                return None;
            }
            Some(results.keys().map(|device| device.to_string()).collect::<Vec<String>>())
        },
        Err(e) => {
            eprintln!("all failed: {}", e.to_string());
            Some(devices)
        }
    }
        
}

/// do send to real device
async fn do_send<T>(msg: &Msg, topic: &str, token: &str, devices: T) -> Result<HashMap<String, String>, Error>
where 
    T: Iterator<Item = String>
{
    let client: reqwest::Client = reqwest::ClientBuilder::new().http2_prior_knowledge().build().unwrap();
    let mut results: HashMap<String, String> = HashMap::new();
    let body: String = msg.serialize();
    let devices: HashSet<String> = devices.collect::<HashSet<String>>();
    for device  in devices {
        let resp = 
                client
                    .post(format!("https://{host}/3/device/{device}", host = APNS_HOST, device = device))
                    .bearer_auth(token)
                    .header("apns-push-type", "alert")
                    .header("apns-topic", topic)
                    .body(body.clone())
                    .send().await;
        match resp {
            Ok(resp) => {
                if ! resp.status().is_success() {
                    let sc = resp.status().as_u16().to_string();
                    if let Some(len) = resp.content_length() {
                        if len > 2 {
                            match resp.text().await {
                                Ok(text) => {
                                    results.insert(device.to_string(), sc + text.as_str());
                                },
                                Err(e) => {
                                    eprint!("{}", e.to_string());
                                    results.insert(device.to_string(), sc + e.to_string().as_str());
                                }
                            }
                        }
                    }
                }
            },
            Err(e) => {
                eprintln!("send to {} failed: {}", device, e.to_string());
                results.insert(device.to_string(), e.to_string());
            }
        }
    }
    Ok(results)
}