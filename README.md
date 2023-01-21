# Rustflix

> A CLI application written the purposes of learning Diesel and Clap.

This project is an extension onto the [Rust & SQL Databases (With Diesel)](https://www.youtube.com/watch?v=tRC4EIKhMzw) and [Rust Command Line Argument Parsing (A Better Way With Clap)](https://www.youtube.com/watch?v=fD9ptABVQbI) videos provided by [Code to the Moon](https://www.youtube.com/@codetothemoon).

## Prerequisites

You will need to install the following dependency on your host to use `diesel`:

```bash
sudo apt-get install libpq-dev
```

## Setting up the database

To get started with Diesel, I recommend visiting the [getting started](https://diesel.rs/guides/getting-started.html) page to understand the context behind the commands.

### Applying migrations

```bash
diesel migration run
```

## Installing `rustflix`

To build and install `rustflix` on your path, run the following command:

```bash
cargo build && cargo install --path .
```

### Running `rustflix`

`rustflix` will provide `--help` however the following are examples of what example commands for adding a video and viewing it.

#### Example Command

```bash
rustflix video create --help

> Create a new video
>
> Usage: rustflix video create <TITLE> <DESCRIPTION>
>
> Arguments:
>  <TITLE>        The title of the video to create
>  <DESCRIPTION>  The description of the video to create
>
> Options:
>  -h, --help  Print help
```

#### Creating a video

```bash
rustflix video create "Career Opportunities" "A movie from the early 90's recently popularised by dark synthwave artist Mr.Kitty in his song `After Dark`"

> Creating video: CreateVideo { title: "Career Opportunities", description: "A movie from the early 90's recently popularised by dark synthwave artist Mr.Kitty in his song `After Dark`" }
```

#### Viewing a video

```shell
rustflix video show

> Showing videos
> Displaying 1 videos
> Video { id: 1, title: "Trick R Treat", description: "Awesome", removed: false }
```
