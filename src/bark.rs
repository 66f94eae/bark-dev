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

use std::time::Duration;

use crate::msg::Msg;


const TOKEN_OFFSET: u64 = 2700;
const TEAM_ID: &str = "5U8LBRXG3A";
const AUTH_KEY_ID: &str = "LH4T9V5U4R";
const TOPIC: &str = "me.fin.bark";

const KEY: &str = r#"-----BEGIN PRIVATE KEY----- 
MIGTAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBHkwdwIBAQQg4vtC3g5L5HgKGJ2+ 
T1eA0tOivREvEAY2g+juRXJkYL2gCgYIKoZIzj0DAQehRANCAASmOs3JkSyoGEWZ 
sUGxFs/4pw1rIlSV2IC19M8u3G5kq36upOwyFWj9Gi3Ejc9d3sC7+SHRqXrEAJow 
8/7tRpV+ 
-----END PRIVATE KEY-----
"#;
pub struct Bark {
    team_id: String,
    auth_key_id: String,
    topic: String,
    key: String,
    token: String
}


impl Bark {
    pub fn new() -> Self {
        Self {
            team_id : TEAM_ID.to_string(),
            auth_key_id : AUTH_KEY_ID.to_string(),
            topic : TOPIC.to_string(),
            key : KEY.to_string(),
            token : ".".to_string(),
        }
    }

    pub fn born(timestamp: u64, token: String) -> Self {
        if timestamp + TOKEN_OFFSET <= Self::ts() {
            println!("warning: token expired, bark will new one");
            return Self::new();
        }
        Self {
            team_id : TEAM_ID.to_string(),
            auth_key_id : AUTH_KEY_ID.to_string(),
            topic: TOPIC.to_string(),
            key: KEY.to_string(),
            token: format!("{}.{}", timestamp, token),
        }
    }

    fn ts() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|_e| Duration::from_secs(0))
            .as_secs()
    }

    /// get apns token
    /// 
    /// return (create_timestamp, token)
    pub fn token(&mut self) -> (u64, String) {
        let token = self.token.split_once(".").unwrap();
        (token.0.parse::<u64>().unwrap_or_else(|_e| 0), token.1.to_string())
    }

    /// force refresh apns token
    /// 
    /// return (create_timestamp, token)
    pub fn force_refresh_token(&mut self) -> (u64, String) {
        self.get_token();
        self.token()
    }
    /// send msg to devices
    /// 
    /// return : None if success, or a vector of failed devices and error messages
    pub fn send<T>(&mut self, msg: &Msg, devices: T) -> Option<Vec<String>> 
    where
        T: IntoIterator<Item = &'static str> + Copy
    {
        crate::apns::send(&msg, self.topic.clone().as_str(), &self.get_token(), devices)
    }

    /// async send to devices
    /// 
    /// return : None if success, or a vector of failed devices and error messages
    pub async fn async_send<T>(&mut self, msg: &Msg, devices: T) -> Option<Vec<String>>
    where
        T: IntoIterator<Item = &'static str> + Copy
    {
        crate::apns::async_send(&msg, self.topic.clone().as_str(), &self.get_token(), devices).await
    }

    fn get_token(&mut self) -> String {
        let time_stamp: u64 = Self::ts(); 

        if let Some((ts, token)) = self.token.split_once(".") {
            // cache the token in memory for TOKEN_OFFSET[default is 2700] seconds
            if ts.parse::<u64>().unwrap_or_else(|_e| 0) + TOKEN_OFFSET >= time_stamp {
                return token.to_string();
            }
        }
        
        let jwt_header: String = Self::clean_str(
            openssl::base64::encode_block(
                format!("{{ \"alg\": \"ES256\", \"kid\": \"{}\" }}", self.auth_key_id)
                .as_bytes()
            )
        );

        let jwt_claims: String = Self::clean_str(
            openssl::base64::encode_block(
                format!("{{ \"iss\": \"{}\", \"iat\": {} }}", 
                        self.team_id, time_stamp
                    )
                .as_bytes()
            )
        );

        let mut singer: openssl::sign::Signer<'_> = openssl::sign::Signer::new(
                                openssl::hash::MessageDigest::sha256(),
                                &openssl::pkey::PKey::from_ec_key(
                                                openssl::ec::EcKey::private_key_from_pem(self.key.as_bytes()).expect("init key data failed")
                                            ).expect("generate private key failed")
                                ).expect("init signer failed");

        let jwt_header: String = format!("{}.{}", jwt_header, jwt_claims);
        singer.update(jwt_header.as_bytes()).expect("fill sign data failed");
        let sign: Vec<u8> = singer.sign_to_vec().expect("sign failed");
        let jwt_signature: String = Self::clean_str(openssl::base64::encode_block(&sign));
        let token: String= format!("{}.{}", jwt_header, jwt_signature);

        self.token = format!("{}.{}", time_stamp, token);
        token
    }
    
    fn clean_str(str: String) -> String {
        str.replace("+", "-")
            .replace("/", "_")
            .replace("=", "")
    }
   

}