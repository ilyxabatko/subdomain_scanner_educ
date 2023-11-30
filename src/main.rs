use rayon::prelude::*;
use reqwest::{blocking::Client, redirect};
use std::{env, time::Duration};
use anyhow;

mod model;
mod error;
mod ports;
mod subdomains;
mod common_ports;

fn main() -> Result<(), anyhow::Error> {
    println!("Hello, world!");
    Ok(())
}
