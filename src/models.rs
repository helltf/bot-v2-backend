#[derive(Queryable)]
pub struct ChannelEntity {
    pub id: i32,
    pub channel_name: String,
    pub allowed: bool,
    pub allowed_live: bool,
    pub joined: bool,
    pub times_connected: i32,
    pub connect_timestamp: i64,
}
