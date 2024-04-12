use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatPhoto {
    pub small_file_id: String,
    pub small_file_unique_id: String,
    pub big_file_id: String,
    pub big_file_unique_id: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BirthDate {
    pub day: u8,
    pub month: u8,
    pub year: u16
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PhotoSize {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: u64,
    pub height: u64,
    pub file_size: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: Option<u64>,
    pub file_path: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MaskPosition {
    pub point: String,
    pub x_shift: f64,
    pub y_shift: f64,
    pub scale: f64
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sticker {
    pub file_id: String,
    pub file_unique_id: String,
    pub type_: String,
    pub width: u64,
    pub height: u64,
    pub is_animated: bool,
    pub is_video: bool,
    pub thumbnail: Option<PhotoSize>,
    pub emoji: Option<String>,
    pub set_name: Option<String>,
    pub premium_animation: Option<File>,
    pub mask_position: Option<MaskPosition>,
    pub custom_emoji_id: Option<String>,
    pub needs_repaiting: Option<bool>,
    pub file_size: Option<u64>,

}

#[derive(Debug, Deserialize, Serialize)]
pub struct BusinessIntro {
    pub title: Option<String>,
    pub message: Option<String>,
    pub sticker: Option<Sticker>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<i64>,
    pub heading: Option<i64>,
    pub proximity_alert_radius: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BusinessLocation {
    pub address: String,
    pub location: Location
}