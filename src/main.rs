// Clap used for CLI argument parsing
use clap::Parser;

// RSS crate to build RSS files
use rss::ChannelBuilder;

// Used to check if a file exists
use std::path::Path;

// Used to write to a file
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;


#[derive(Parser, Debug, Clone)]
struct Args {

    #[clap(long="title", short='t', default_value="foobar", required=true)]
    title: String,

    #[clap(long="link", short='l', default_value="http://0.0.0.0", required=true)]
    link: String,

    #[clap(long="description", short='d', default_value="foobar", required=true)]
    description: String,

    #[clap(long="file", short='f', required=true)]
    file: String
}


// Handles building a feed
fn build_channel(args: Args) -> String {

    // Builds the RSS data
    let channel = ChannelBuilder::default()
        .title(args.title)
        .link(args.link)
        .description(args.description)
        .build()
        .to_string() + "\n";

    return channel;
}

// Handles writing to a file
fn write_to_file(args : Args, feed: String) {

    // Check if the file exists
    // If not, create it
    if Path::new(&args.file).exists() == false {
        let result = File::create(&args.file);
    }

    // Open the file in append mode
    // to prevent overwriting
    let mut file_ref = OpenOptions::new()
        .append(true)
        .open(args.file)
        .expect("Unable to open file");

    // Write the feed to the file
    file_ref.write_all(feed.as_bytes())
        .expect("Writing to file failed");

    return;
}

fn main() {
    let args = Args::parse();
    let feed = build_channel(args.clone());
    write_to_file(args.clone(), feed);
    println!("RSS file generated successfully");
}
