// Clap used for CLI argument parsing
use clap::Parser;

// RSS crate to build RSS files
use rss::ChannelBuilder;
use rss::ItemBuilder;
use rss::Channel;

// Used to check if a file exists
use std::path::Path;

// Used to write to a file
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::BufReader;


#[derive(Parser, Debug, Clone)]
struct Args {

    #[clap(long="title", short='t', required=true)]
    title: String,

    #[clap(long="link", short='l', required=true)]
    link: String,

    #[clap(long="description", short='d', required=true)]
    description: String,

    #[clap(long="file", short='f', required=true)]
    file: String
}


// Check whether a file exists
// If it doesn't, create it
// and write default RSS XML to it
fn check_file(file: String) {
    if Path::new(&file).exists() == false {
        println!("File not found, creating...");

        let _result = File::create(&file);

        let channel = ChannelBuilder::default()
            .build()
            .to_string();

        write_to_file(file.clone(), channel);
    }

    return;
}


// Open the file to a buffer,
// and add to a Channel
fn read_file(file: String) -> Channel {
    let f = File::open(&file)
        .unwrap();
    let buffer = BufReader::new(f);
    let channel = Channel::read_from(buffer)
        .unwrap();

    return channel;
}


// Build an item and append to a channel
fn build_rss(args: Args, mut channel: Channel) -> String {
    let item = ItemBuilder::default()
        .title(args.title)
        .link(args.link)
        .description(args.description)
        .build();

    // Add the built item to the feed
    channel.items.push(item);

    return channel.to_string();
}


// Handles writing to a file
fn write_to_file(file: String, feed: String) {
    fs::write(&file, feed.as_bytes());
    
    return;
}


fn main() {
    let args = Args::parse();
    check_file(args.file.clone());
    let channel = read_file(args.file.clone());
    let feed = build_rss(args.clone(), channel);
    write_to_file(args.file.clone(), feed);
    println!("RSS file generated successfully");
}
