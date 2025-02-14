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

/// A simple library for sending push notifications 
/// 
/// to iOS devices which install the [bark] app using the APNS protocol.
/// 
/// [bark]: https://github.com/finb/bark
/// # Example
/// ```rust
/// use bark::{bark::Bark, msg::Msg};
/// 
/// let mut bark: Bark = Bark::new();
/// 
/// let msg = Msg::new("notify".to_string(), "hello world".to_string());
/// 
/// let devices: Vec<String> = vec!["the_device_token_get_from_bark_app"];
/// 
/// let send_reult: Option<Vec<String>> = None;//bark.send(&msg, &devices);
/// 
/// // send result is None if success
/// assert!(send_reult.is_none());
/// 
/// // send result is a vector of failed devices if failed
/// if let Some(failed_devices) = send_reult {
///     // do something
///     println!("send failed: {:?}", failed_devices);
/// };
/// ```
/// # Note
/// 
/// request apns need a token
/// 
/// The token is cached in memory by default and refreshed automatically
///
/// If your app frequently run in oneshot mode, or you want share the token with other app
/// 
/// you can call `bark.token()` to get the token and presist it
/// 
/// and call `bark.born(time_stamp, token)` to new a bark instance with the token
/// 
/// 
/// # Features
/// - [x] send push notifications to iOS devices which install the #bark# app using the APNS protocol.
/// - [x] async send push notificationsto iOS devices which install the #bark# app using the APNS protocol
pub mod bark;
mod apns;
pub mod msg;