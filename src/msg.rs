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


use openssl::symm::{Cipher, Crypter, Mode};

/// Push Notification Message structure.
///
/// This struct represents a push notification message that can be sent to devices.
/// It contains various fields to customize the notification's behavior and appearance.
///
/// # Example
/// ```rust
/// use bark::msg::Msg;
///
/// // new a simple message with title and body
/// let msg = Msg::new("title", "body");
///
/// // new a message with title = Notification and body
/// let mut msg = Msg::with_body("body");
///
/// // set some fields
/// msg.set_level("active");
/// msg.set_badge(1);
/// // and so on
/// ```
pub struct Msg {
    /// Push Title
    title: String,

    /// Push Content
    body: String,

    /// Push Interruption Level
    /// active: Default value, the system will immediately display the notification on the screen.
    /// timeSensitive: Time-sensitive notification, can be displayed while in focus mode.
    /// passive: Only adds the notification to the notification list, will not display on the screen.
    level: Option<String>,

    /// Push Badge, can be any number
    badge: Option<u64>,

    /// Pass 0 to disable; Automatically copy push content below iOS 14.5; above iOS 14.5, you need to manually long-press the push or pull down the push
    auto_copy: Option<u8>,

    /// When copying the push, specify the content to copy; if this parameter is not provided, the entire push content will be copied
    copy: Option<String>,

    /// You can set different ringtones for the push
    sound: Option<String>,

    /// Set a custom icon for the push; the set icon will replace the default Bark icon
    icon: Option<String>,

    /// Group messages; pushes will be displayed in groups in the notification center
    group: Option<String>,

    /// Pass 1 to save the push; passing anything else will not save the push; if not passed, it will be decided according to the app's internal settings
    is_archive: Option<u8>,

    /// The URL to jump to when clicking the push, supports URL Scheme and Universal Link
    url: Option<String>,

    /// iv, 12 Bytes
    iv: Option<String>,
    /// encrypt type
    enc_type: Option<EncryptType>,
    /// encrypt mode
    mode: Option<EncryptMode>,
    /// encrypt key, 24 Bytes
    key: Option<String>,
    /// cipher
    cipher: Option<Cipher>,
}

#[derive(Clone, Copy)]
pub enum EncryptMode {
    CBC,
    ECB,
    GCM,
}

impl EncryptMode {
    pub fn from_str(str: &str) -> Option<Self> {
        if str.is_empty() {
            return None;
        }
        match str.to_lowercase().as_str() {
            "cbc" => Some(EncryptMode::CBC),
            "ecb" => Some(EncryptMode::ECB),
            "gcm" => Some(EncryptMode::GCM),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub enum EncryptType {
    AES128,
    AES192,
    AES256,
}

impl EncryptType {
    pub fn from_str(str: &str) -> Option<Self> {
        if str.is_empty() {
            return None;
        }
        match str.to_lowercase().as_str() {
            "aes128" => Some(EncryptType::AES128),
            "aes192" => Some(EncryptType::AES192),
            "aes256" => Some(EncryptType::AES256),
            _ => None,
        }
    }
}

impl Msg {
    /// Creates a new `Msg` instance with a title and body.
    ///
    /// # Arguments
    /// - `title`: The title of the notification.
    /// - `body`: The content/body of the notification.
    ///
    /// # Returns
    /// A new `Msg` instance.
    pub fn new(title: &str, body: &str) -> Self {
        Msg {
            ..Self::default(Some(title.to_string()), body.to_string())
        }
    }

    /// Creates a new `Msg` instance with only a body.
    ///
    /// # Arguments
    /// - `body`: The content/body of the notification.
    ///
    /// # Returns
    /// A new `Msg` instance with the title set to "Notification".
    pub fn with_body(body: &str) -> Self {
        Msg {
            ..Self::default(None, body.to_string())
        }
    }

    /// Creates a default `Msg` instance.
    ///
    /// # Arguments
    /// - `title`: An optional title for the notification.
    /// - `body`: The content/body of the notification.
    ///
    /// # Returns
    /// A new `Msg` instance with default values.
    fn default(title: Option<String>, body: String) -> Self {
        Msg {
            title: title.unwrap_or("Notification".to_string()),
            body,
            level: None,
            badge: None,
            auto_copy: None,
            copy: None,
            sound: Some("chime.caf".to_string()),
            icon: Some("https://github.com/66f94eae/bark-dev/raw/main/bot.jpg".to_string()),
            group: None,
            is_archive: None,
            url: None,
            iv: None,
            enc_type: None,
            mode: None,
            key: None,
            cipher: None,
        }
    }

    /// Sets the interruption level of the notification.
    ///
    /// # Arguments
    /// - `level`: The interruption level (`active`, `timeSensitive`, or `passive`).
    ///
    /// # Returns
    /// A mutable reference to `self` for method chaining.
    pub fn set_level(&mut self, level: &str) -> &mut Self {
        match level.to_lowercase().as_str() {
            "active" => self.level = Some("active".to_string()),
            "timesensitive" => self.level = Some("timeSensitive".to_string()),
            "passive" => self.level = Some("passive".to_string()),
            _ => self.level = None,
        }
        self
    }

    /// Sets the badge number.
    ///
    /// # Arguments
    /// - `badge`: The badge number to display on the app icon.
    ///
    /// # Returns
    /// A mutable reference to `self` for method chaining.
    pub fn set_badge(&mut self, badge: u64) -> &mut Self {
        if badge > 0 {
            self.badge = Some(badge);
        } else {
            self.badge = None;
        }
        self
    }

    /// Sets whether to automatically copy the notification content.
    ///
    /// # Arguments
    /// - `auto_copy`: 0 to disable, other values to enable.
    ///
    /// # Returns
    /// A mutable reference to `self` for method chaining.
    pub fn set_auto_copy(&mut self, auto_copy: u8) -> &mut Self {
        match auto_copy {
            0 => self.auto_copy = Some(0),
            _ => self.auto_copy = None,
        }
        self
    }

    /// Sets specific content to copy when the notification is copied.
    ///
    /// # Arguments
    /// - `copy`: The content to copy.
    ///
    /// # Returns
    /// A mutable reference to `self` for method chaining.
    pub fn set_copy(&mut self, copy: &str) -> &mut Self {
        if copy.trim().is_empty() {
            self.copy = None;
        } else {
            self.copy = Some(copy.to_string());
        }
        self
    }

    /// Sets the sound file to play with the notification.
    ///
    /// # Arguments
    /// - `sound`: The sound file name.
    ///
    /// # Returns
    /// A mutable reference to `self` for method chaining.
    pub fn set_sound(&mut self, sound: &str) -> &mut Self {
        self.sound = Some(sound.to_string());
        self
    }

    /// Sets a custom icon URL for the notification.
    ///
    /// # Arguments
    /// - `icon`: The icon URL.
    ///
    /// # Returns
    /// A mutable reference to `self` for method chaining.
    pub fn set_icon(&mut self, icon: &str) -> &mut Self {
        if icon.trim().is_empty() {
            self.icon = None;
        } else {
            self.icon = Some(icon.to_string());
        }
        self
    }

    /// Sets the group identifier for notifications.
    ///
    /// # Arguments
    /// - `group`: The group identifier.
    ///
    /// # Returns
    /// A mutable reference to `self` for method chaining.
    pub fn set_group(&mut self, group: &str) -> &mut Self {
        self.group = Some(group.to_string());
        self
    }

    /// Sets whether to archive the notification.
    ///
    /// # Arguments
    /// - `is_archive`: 1 to save, other values to not save.
    ///
    /// # Returns
    /// A mutable reference to `self` for method chaining.
    pub fn set_is_archive(&mut self, is_archive: u8) -> &mut Self {
        match is_archive {
            1 => self.is_archive = Some(1),
            _ => self.is_archive = None,
        }
        self
    }

    /// Sets the URL to open when the notification is clicked.
    ///
    /// # Arguments
    /// - `url`: The URL.
    ///
    /// # Returns
    /// A mutable reference to `self` for method chaining.
    pub fn set_url(&mut self, url: &str) -> &mut Self {
        if url.trim().is_empty() {
            self.url = None;
        } else {
            self.url = Some(url.to_string());
        }
        self
    }

    /// Sets the initialization vector for encryption.
    ///
    /// # Arguments
    /// - `iv`: The initialization vector.
    ///
    /// # Returns
    /// A mutable reference to `self` for method chaining.
    pub fn set_iv(&mut self, iv: &str) -> &mut Self {
        if iv.trim().is_empty() {
            self.iv = None;
        } else if iv.len() != 12 {
            panic!("Invalid IV length. IV must be 12 bytes long.");
        } else {
            self.iv = Some(iv.to_string());
        }
        self
    }

    /// Generates a random initialization vector.
    ///
    /// # Returns
    /// A mutable reference to `self` for method chaining.
    pub fn gen_iv(&mut self) -> &mut Self {
        let mut iv: [u8; 16] = [0u8; 16];
        openssl::rand::rand_bytes(&mut iv).unwrap();
        self.set_iv(iv.iter().map(|b| format!("{:02x}", b)).collect::<String>().split_off(16).as_str())
    }

    fn set_cipher(&mut self) -> &mut Self {
        if self.enc_type.is_none() || self.mode.is_none() {
            return self;
        }
        let enc_type = self.enc_type.unwrap();
        let mode = self.mode.unwrap();

        let cipher: Cipher = match enc_type {
            EncryptType::AES128 => {
                match mode {
                    EncryptMode::CBC => {
                        Cipher::aes_128_cbc()
                    },
                    EncryptMode::ECB => {
                        Cipher::aes_128_ecb()
                    },
                    EncryptMode::GCM => {
                        Cipher::aes_128_gcm()
                    },
                }
            },
            EncryptType::AES192 => {
                match mode {
                    EncryptMode::CBC => {
                        Cipher::aes_192_cbc()
                    },
                    EncryptMode::ECB => {
                        Cipher::aes_192_ecb()
                    },
                    EncryptMode::GCM => {
                        Cipher::aes_192_gcm()
                    },
                }
            }, 
            EncryptType::AES256 => {
                match mode {
                    EncryptMode::CBC => {
                        Cipher::aes_256_cbc()
                    },
                    EncryptMode::ECB => {
                        Cipher::aes_256_ecb()
                    },
                    EncryptMode::GCM => {
                        Cipher::aes_256_gcm()
                    },
                }
            },
        };
        self.cipher = Some(cipher);
        self
    }

    /// Sets the encryption type and updates the cipher.
    ///
    /// # Arguments
    /// - `enc_type`: The encryption type [`EncryptType`].
    ///
    /// # Panics
    /// Panics if the encryption type already set.
    ///
    /// # Returns
    /// A mutable reference to `self` for method chaining.
    pub fn set_enc_type(&mut self, enc_type: EncryptType) -> &mut Self {
        if self.enc_type.is_some() {
            panic!("Encrypt type can only be set once");
        }
        self.enc_type = Some(enc_type);
        self.set_cipher();
        self
    }

    /// Sets the encryption mode and updates the cipher.
    ///
    /// # Arguments
    /// - `mode`: The encryption mode [`EncryptMode`].
    ///
    /// # Panics
    /// Panics if the encryption mode already set.
    ///
    /// # Returns
    /// A mutable reference to `self` for method chaining.
    pub fn set_mode(&mut self, mode: EncryptMode) -> &mut Self {
        if self.mode.is_some() {
            panic!("Encrypt mode can only be set once");
        }
        self.mode = Some(mode);
        match mode {
            EncryptMode::ECB | EncryptMode::GCM => {
                if self.iv.is_none() {
                    self.gen_iv();
                }
            },
            _ => {},
        }
        self.set_cipher();
        self
    }

    /// Sets the encryption key.
    ///
    /// # Arguments
    /// - `key`: The encryption key.
    ///
    /// # Returns
    /// A mutable reference to `self` for method chaining.
    pub fn set_key(&mut self, key: &str) -> &mut Self {
        if key.len() != 24 {
            panic!("Invalid key length. Key must be 24 characters long.");
        }
        self.key = Some(key.to_string());
        self
    }

    fn json(&self, encry_body: Option<String>) -> String {
        let mut body: String = format!("{{\"aps\":{{\"mutable-content\":1,\"category\":\"myNotificationCategory\",\"interruption-level\":\"{level}\",", level = self.level.as_ref().unwrap_or(&"active".to_string()));

        if let Some(badge) = self.badge {
            body += &format!("\"badge\":{badge},", badge = badge);
        }

        if let Some(sound) = &self.sound {
            body += &format!("\"sound\":\"{sound}\",", sound = sound);
        }

        if let Some(group) = &self.group {
            body += &format!("\"thread-id\":\"{group}\",", group = group);
        }

        let alert: String = format!(
            "\"alert\":{{\"title\":\"{title}\",\"body\":\"{body}\"}}}}",
            title = self.title,
            body = if encry_body.is_some() {
                "NoContent"
            } else {
                self.body.as_str()
            }
        );

        body = body + &alert;

        if let Some(icon) = &self.icon {
            body += &format!(",\"icon\":\"{icon}\"", icon = icon);
        }

        if let Some(auto_copy) = self.auto_copy {
            body += &format!(",\"autoCopy\":{auto_copy}", auto_copy = auto_copy);
        }

        if let Some(is_archive) = self.is_archive {
            body += &format!(",\"isArchive\":{is_archive}", is_archive = is_archive);
        }

        if let Some(copy) = &self.copy {
            body += &format!(",\"copy\":\"{copy}\"", copy = copy);
        }

        if let Some(url) = &self.url {
            body += &format!(",\"url\":\"{url}\"", url = url);
        }

        if let Some(iv) = &self.iv {
            body += &format!(",\"iv\":\"{iv}\"", iv = iv);
        }

        if let Some(encry_body) = encry_body {
            body += &format!(",\"ciphertext\":\"{encry_body}\"", encry_body = encry_body);
        }

        body + "}"
    }

    fn to_json(&self) -> String {
        // let body: String = format!("{{\"aps\":{{\"interruption-level\":\"critical\",\"mutable-content\":1,\"alert\":{{\"title\":\"{title}\",\"body\":\"{body}\"}},\"category\":\"myNotificationCategory\",\"sound\":\"chime.caf\"}},\"icon\":\"{icon}\"}}",
        // title = self.title, body = self.body, icon= self.icon
        //     );
        self.json(None)
    }

    /// Encrypts the message using the specified encryption type, mode, and key.
    /// 
    /// # Returns
    /// A `Result` containing the encrypted message as a `String` or an error if the encryption fails.
    fn encrypt(&self) -> Result<String, Box<dyn std::error::Error>> {
        if self.enc_type.is_none() || self.mode.is_none() || self.key.is_none() {
            panic!("Encrypt type, mode, and key must be set");
        }

        let key: String = self.key.as_ref().unwrap().clone();

        let original: String = format!("{{\"body\":\"{}\"}}", self.body);
        let original: &[u8] = original.as_bytes();

        let cipher: Cipher = self.cipher.unwrap();

        let mut crypter: Crypter = Crypter::new(
            cipher,
            Mode::Encrypt,
            key.as_bytes(),
            Some(self.iv.as_ref().unwrap().as_bytes()),
        )
        .unwrap();
        crypter.pad(true); // Enable PKCS7 padding
        let mut buffer: Vec<u8> = vec![0; original.len() + cipher.block_size()];
        let count: usize = crypter.update(&original, &mut buffer).unwrap();
        let rest: usize = crypter.finalize(&mut buffer[count..]).unwrap();
        buffer.truncate(count + rest);
        Ok(self.json(Some(openssl::base64::encode_block(&buffer))))
    }

    /// Serializes the message into a JSON string, encrypting the message if necessary.
    /// 
    /// # Returns
    /// A `String` containing the serialized message.
    pub fn serialize(&self) -> String {
        if self.cipher.is_some() {
            match self.encrypt() {
                Ok(encrypted) => encrypted,
                Err(e) => panic!("Error encrypting message: {}", e),
            }
        } else {
            self.to_json()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_json_all_field() {
        let mut msg = Msg::new("Test Title", "Test Body");
        msg.set_level("timeSensitive");
        msg.set_badge(1);
        msg.set_auto_copy(1);
        msg.set_copy("Test Copy");
        msg.set_sound("chime.caf");
        msg.set_icon("icon.png");
        msg.set_group("Test Group");
        msg.set_is_archive(1);
        msg.set_url("https://example.com");
        let json = msg.to_json();
        println!("{}", json);
        assert_eq!(json, "{\"aps\":{\"mutable-content\":1,\"category\":\"myNotificationCategory\",\"interruption-level\":\"timeSensitive\",\"badge\":1,\"sound\":\"chime.caf\",\"thread-id\":\"Test Group\",\"alert\":{\"title\":\"Test Title\",\"body\":\"Test Body\"},\"icon\":\"icon.png\"},\"isArchive\":1,\"copy\":\"Test Copy\",\"url\":\"https://example.com\"}");
    }

    #[test]
    fn test_to_json_part_field() {
        let mut msg = Msg::new("Test Title", "Test Body");
        msg.set_level("passive");
        msg.set_badge(1);
        msg.set_auto_copy(1);
        msg.set_copy("");
        msg.set_sound("chime.caf");
        msg.set_icon("icon.png");
        let json = msg.to_json();
        println!("{}", json);
        assert_eq!(json, "{\"aps\":{\"mutable-content\":1,\"category\":\"myNotificationCategory\",\"interruption-level\":\"passive\",\"badge\":1,\"sound\":\"chime.caf\",\"alert\":{\"title\":\"Test Title\",\"body\":\"Test Body\"},\"icon\":\"icon.png\"}}");
    }

    #[test]
    fn test_to_json_default() {
        let msg = Msg::new("Test Title", "Test Body");
        let json = msg.to_json();
        println!("{}", json);
        assert_eq!(json, "{\"aps\":{\"mutable-content\":1,\"category\":\"myNotificationCategory\",\"interruption-level\":\"active\",\"alert\":{\"title\":\"Test Title\",\"body\":\"Test Body\"}}}");
    }
}
