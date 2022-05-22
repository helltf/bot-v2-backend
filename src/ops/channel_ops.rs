extern crate bot_v2_backend;
extern crate diesel;

use self::diesel::prelude::*;
use bot_v2_backend::establish_connection;
use bot_v2_backend::models::ChannelEntity;

pub fn show_channels() {
    use bot_v2_backend::schema::channel::dsl::*;

    let connection = establish_connection();
    let results = channel
        .filter(allowed.eq(true))
        .load::<ChannelEntity>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for channel_result in results {
        println!("{}", channel_result.channel_name);
        println!("----------\n");
        println!("{}", channel_result.channel_name);
    }
}
