# Telegram Video Converter

> [!WARNING]  
> This is a super simple program that has only been tested on Linux assumes you have FFmpeg installed.

Convert OBS recordings into files supported by Telegram Mobile. Useful when recording stuff on Linux that you want to share to people on the mobile version of Telegram.

How to run locally:

Build:

```sh
cargo build --release
```

Install:

```sh
cargo install --path .
```

Run with:

```sh
telegram-video-converter test.mp4
```
