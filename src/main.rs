use std::io::Write;

const BRIGHTNESS_PATH: &str = "/sys/class/backlight/intel_backlight/brightness";
const MAX_BRIGHTNESS_PATH: &str = "/sys/class/backlight/intel_backlight/max_brightness";

fn parse_file(path: &str) -> u32 {
    std::fs::read_to_string(path)
        .unwrap()
        .trim_end()
        .parse::<u32>()
        .unwrap()
}

fn write_brightness(brightness: u32) {
    std::fs::File::create(BRIGHTNESS_PATH)
        .unwrap()
        .write_all(&brightness.to_string().into_bytes())
        .unwrap();
}

fn main() {
    let brightness = parse_file(BRIGHTNESS_PATH);
    let max_brightness = parse_file(MAX_BRIGHTNESS_PATH);

    let args: Vec<String> = std::env::args().collect();
    if args.contains(&String::from("inc")) {
        write_brightness(std::cmp::min(brightness * 2, max_brightness));
    } else if args.contains(&String::from("dec")) {
        write_brightness(std::cmp::max(brightness / 2, 1));
    } else {
        println!("{}/{}", brightness, max_brightness);
    }
}
