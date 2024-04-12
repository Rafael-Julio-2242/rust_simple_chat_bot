use serde::{Deserialize, Serialize};

use crate::models::helper_structs::ChatPhoto;

use super::helper_structs::{BirthDate, BusinessIntro, BusinessLocation};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub is_premium: Option<bool>,
    pub added_to_attachment_menu: Option<bool>,
    pub can_join_groups: Option<bool>,
    pub can_read_all_group_messages: Option<bool>,
    pub supports_inline_queries: Option<bool>,
    pub can_connect_to_business: Option<bool>,
}


pub struct Chat {
    pub id: i64,
    pub type_: String,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_forum: Option<bool>,
    pub photo: Option<ChatPhoto>,
    pub active_usernames: Option<Vec<String>>,
    pub birthdate: Option<BirthDate>,
    pub business_intro: Option<BusinessIntro>,
    pub business_location: Option<BusinessLocation>,
    // business_opening_hours
}