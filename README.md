# Bark library for Rust

## Overview

OverWrite the code [直接调用apns接口](https://bark.day.app/#/apns?id=%e7%9b%b4%e6%8e%a5%e8%b0%83%e7%94%a8apns%e6%8e%a5%e5%8f%a3)

This library is for someone developed in Rust for sending push notifications to iOS devices which has instaledthe [Bark](https://github.com/imnotjames/bark) app.

The message is directly sent to APNS server, so it is very fast and safe.

And the message can be sent in encrypted mode, that means APNS can not see the content of the message.


## Key Features

- **Multi-Device Support**: Send notifications to multiple iOS devices simultaneously.
- **Customizable Notifications**: Set custom titles and messages for your notifications.
- **Secure Communication**: Utilizes Apple Push Notification service (APNs) with JWT authentication.

## Technical Details

- **Language**: Rust
- **Dependencies**:
  - `openssl`: For cryptographic operations and JWT token generation
  - `reqwest`: For making HTTP requests to the APNs servers
  - `tokio`: For asynchronous I/O operations

## Example Usage
first add dependencies
```
cargo add bark-dev
```

### send a simple message
```rust
let mut bark = bark::Bark::new();
let msg = bark::Message::new("title", "body");
let resp = bark.send(msg);
let devices = vec!["device_token_get_from_bark_app"];
let send_result = bark.send(&msg, &devices);

if let Some(failed_device) = send_result {
    println!("Failed to send to device: {}", failed_device);
}

```

### send a encrypted message
```rust
let mut bark = bark::Bark::new();
let mut msg = bark::Message::new("title", "body");

msg.set_enc_type("aes128");
msg.set_mode("cbc");
msg.set_key("the_key_must_the_same_as_bark_app");
// if you not set iv it will generate a random iv and send it to the server
msg.set_iv("the_iv_must_the_same_as_bark_app");

let resp = bark.send(msg);
let devices = vec!["device_token_get_from_bark_app"];
let send_result = bark.send(&msg, &devices);

if let Some(failed_device) = send_result {
    println!("Failed to send to device: {}", failed_device);
}

```

### async send a message
```rust
let mut bark = bark::Bark::new();
let msg = bark::Message::new("title", "body");
let devices = vec!["device_token_get_from_bark_app"];

let send_result = bark.async_send(&msg, &devices).await?;

if let Some(failed_device) = send_result {
    println!("Failed to send to device: {}", failed_device);
}
```


## known issue
- not all param support in encrypt mode [detail in code](https://github.com/Finb/Bark/blob/master/NotificationServiceExtension/Processor/CiphertextProcessor.swift#L13)