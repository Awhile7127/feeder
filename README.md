# feeder


## Description

`feeder` is a simple command-line tool to generate RSS feeds.

Written in Rust, so should be platform-independent.


## Compiling

Tested using cargo:

### Debian / Ubuntu

```
sudo apt install cargo
cargo build -r
./target/feeder
```


## Usage

```
feeder --help        See help and available options
feeder <title> <link> <description> <file>
```

- title **REQUIRED**: The title to append to the feed
- link **REQUIRED**: The link to append to the feed
- description **REQUIRED**: The description to append to the feed
- file **REQUIRED**: The file to write to
