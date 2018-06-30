#[macro_use]
extern crate samp_sdk;
extern crate rand;

mod plugin;
use plugin::MyPlugin;

new_plugin!(MyPlugin);