use clap::Parser;
use std::path::Path;
use std::process::{Command, exit};

#[derive(Parser)]
#[command(name = "telegram-video-converter")]
#[command(about = "Convert videos to Telegram Mobile compatible format")]
#[command(version = "0.1.0")]
struct Args {
    /// Input video file to convert
    input: String,

    /// Output file path (optional, defaults to input_telegram.mp4)
    #[arg(short, long)]
    output: Option<String>,

    /// Video bitrate in kbps
    #[arg(short, long, default_value = "2000")]
    bitrate: u32,

    /// Audio bitrate in kbps
    #[arg(short = 'a', long, default_value = "128")]
    audio_bitrate: u32,

    /// Frame rate
    #[arg(short, long, default_value = "25")]
    fps: u32,

    /// CRF quality (lower = better quality, 18-28 recommended)
    #[arg(short, long, default_value = "23")]
    crf: u32,

    /// Overwrite output file if it exists
    #[arg(short = 'y', long)]
    overwrite: bool,

    /// Show ffmpeg output (verbose mode)
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    // Check if input file exists
    if !Path::new(&args.input).exists() {
        eprintln!("Error: File '{}' not found", args.input);
        exit(1);
    }

    // Check if ffmpeg is installed
    if !is_ffmpeg_available() {
        eprintln!("Error: ffmpeg is not installed or not in PATH");
        exit(1);
    }

    // Generate output filename
    let output_path = args
        .output
        .unwrap_or_else(|| generate_output_path(&args.input));

    // Check if output file exists and overwrite flag
    if Path::new(&output_path).exists() && !args.overwrite {
        eprintln!(
            "Error: Output file '{}' already exists. Use -y to overwrite.",
            output_path
        );
        exit(1);
    }

    println!("Converting '{}' for Telegram compatibility...", args.input);
    println!("Output: '{}'", output_path);
    println!(
        "Settings: {}kbps video, {}kbps audio, {}fps, CRF {}",
        args.bitrate, args.audio_bitrate, args.fps, args.crf
    );

    // Build ffmpeg command
    let mut cmd = Command::new("ffmpeg");

    // Input file
    cmd.args(["-i", &args.input]);

    // Overwrite flag
    if args.overwrite {
        cmd.arg("-y");
    }

    // Video encoding settings
    cmd.args([
        "-c:v",
        "libx264",
        "-profile:v",
        "baseline",
        "-level",
        "3.0",
        "-pix_fmt",
        "yuv420p",
        "-crf",
        &args.crf.to_string(),
        "-maxrate",
        &format!("{}k", args.bitrate),
        "-bufsize",
        &format!("{}k", args.bitrate * 2),
        "-r",
        &args.fps.to_string(),
    ]);

    // Audio encoding settings
    cmd.args([
        "-c:a",
        "aac",
        "-ar",
        "44100",
        "-ac",
        "2",
        "-b:a",
        &format!("{}k", args.audio_bitrate),
    ]);

    // Output format and optimizations
    cmd.args(["-movflags", "+faststart", "-f", "mp4", &output_path]);

    // Hide ffmpeg output unless verbose
    if !args.verbose {
        cmd.args(["-loglevel", "error"]);
    }

    // Execute conversion
    let start_time = std::time::Instant::now();
    let status = cmd.status();
    let duration = start_time.elapsed();

    match status {
        Ok(exit_status) => {
            if exit_status.success() {
                println!("✓ Conversion successful: {}", output_path);
                println!("  Time taken: {:.2}s", duration.as_secs_f64());

                // Show file sizes
                if let (Ok(input_size), Ok(output_size)) = (
                    std::fs::metadata(&args.input).map(|m| m.len()),
                    std::fs::metadata(&output_path).map(|m| m.len()),
                ) {
                    println!("  Input size: {}", format_bytes(input_size));
                    println!("  Output size: {}", format_bytes(output_size));
                    let ratio = (output_size as f64 / input_size as f64) * 100.0;
                    println!("  Size ratio: {:.1}%", ratio);
                }
            } else {
                eprintln!(
                    "✗ Conversion failed with exit code: {:?}",
                    exit_status.code()
                );
                exit(1);
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to execute ffmpeg: {}", e);
            exit(1);
        }
    }
}

fn is_ffmpeg_available() -> bool {
    Command::new("ffmpeg").arg("-version").output().is_ok()
}

fn generate_output_path(input_path: &str) -> String {
    let path = Path::new(input_path);
    let parent = path.parent().unwrap_or(Path::new("."));
    let stem = path.file_stem().unwrap().to_str().unwrap();

    parent
        .join(format!("{}_telegram.mp4", stem))
        .to_str()
        .unwrap()
        .to_string()
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1} {}", size, UNITS[unit_index])
}
