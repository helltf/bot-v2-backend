extern crate diesel;
extern crate dotenv;
mod ops;

use crate::ops::channel_ops::show_channels;
fn main() {
    show_channels();
}
