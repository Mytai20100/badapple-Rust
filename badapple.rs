use std::process::Command;
use std::thread;
use std::time::Duration;

const ASCII_CHARS: &str = " .:-=+*#%@";
const WIDTH: usize = 80;
const HEIGHT: usize = 40;

fn download_video(url: &str) {
    println!("Downloading video...");
    Command::new("yt-dlp")
        .args(&["-f", "worst", "-o", "badapple.mp4", url])
        .output()
        .expect("Failed to download");
}

fn rgb_to_ascii(r: u8, g: u8, b: u8) -> char {
    let brightness = (r as usize + g as usize + b as usize) / 3;
    let index = brightness * (ASCII_CHARS.len() - 1) / 255;
    ASCII_CHARS.chars().nth(index).unwrap()
}

fn extract_and_display_frame(time: f64) {
    let output = Command::new("ffmpeg")
        .args(&[
            "-ss", &format!("{:.2}", time),
            "-i", "badapple.mp4",
            "-vframes", "1",
            "-vf", &format!("scale={}:{}", WIDTH, HEIGHT),
            "-f", "rawvideo",
            "-pix_fmt", "rgb24",
            "-"
        ])
        .stderr(std::process::Stdio::null())
        .output()
        .expect("Failed to extract frame");
    
    if output.status.success() && !output.stdout.is_empty() {
        print!("\x1B[2J\x1B[H");
        
        let pixels = &output.stdout;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let idx = (y * WIDTH + x) * 3;
                if idx + 2 < pixels.len() {
                    let r = pixels[idx];
                    let g = pixels[idx + 1];
                    let b = pixels[idx + 2];
                    print!("{}", rgb_to_ascii(r, g, b));
                }
            }
            println!();
        }
    }
}

fn main() {
    let url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "https://youtu.be/FtutLA63Cp8".to_string());
    
    download_video(&url);
    
    let fps = 10.0;
    let duration = 30.0;
    let mut time = 0.0;
    
    while time < duration {
        extract_and_display_frame(time);
        thread::sleep(Duration::from_millis((1000.0 / fps) as u64));
        time += 1.0 / fps;
    }
}
