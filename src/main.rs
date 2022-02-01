use image::{self, EncodableLayout};
use std::env;
use termion::{self, terminal_size};

/// Main function
fn main() {
    let args: Vec<String> = env::args().collect();

    let path: String = String::from(&args[1]);

    // If no width is entered, use the terminals width as a default
    let desired_width = args[2]
        .parse::<u32>()
        .expect("Please make sure that that you have entered a valid resolution");

    // Open image and turn and turn it into a Vector of its pixels
    let img = image::open(path).expect("Could not open image");

    image_to_ascii(img, desired_width);
}

/// This function will take an image, and print it using ascii characters
fn image_to_ascii(img: image::DynamicImage, desired_width: u32) {
    let term_size = terminal_size().expect("Couldn't get terminal size");

    // Nearest is the fastest filter
    let img = img
        .resize(desired_width, 10000000, image::imageops::Nearest)
        .to_rgb8();

    let img_width = img.width();
    let img_height = img.height();

    println!(
        "Some info: Screen size: ({},{}) Image size: ({},{})",
        term_size.0,
        term_size.1,
        img.width(),
        img.height()
    );

    if (term_size.0 as u32) < img_width || (term_size.1 as u32) < img_height {
        panic!("The image does not fit in the terminal, please zoom out or lower resolution");
    }

    let img_bytes = img.as_bytes();
    println!("{}", termion::clear::All);

    for column in 0..img_height as usize {
        for row in 0..img_width as usize {
            let i = (column * img_width as usize) + row;

            let pixel_as_char: char = rgb_to_ascii(
                img_bytes[i * 3],
                img_bytes[(i * 3) + 1],
                img_bytes[(i * 3) + 2],
            );

            // Display the pixel as a character
            print!(
                "{}{}{}",
                termion::cursor::Goto((2 * row as u16) + 1, column as u16 + 1),
                pixel_as_char,
                pixel_as_char
            );
        }
    }
}

/// Given an rgb value, return an ascii character.
///
/// ```
/// assert_eq!(rgb_to_ascii(12, 56, 12), ':');
///
/// //  This is the brightest possible character to be returned.
/// assert_eq!(rgb_to_ascii(255, 255, 255), '@');
/// ```
/// The character that can be returned are, in order from darkest to lightest: ``` .:-=+*#%@```
fn rgb_to_ascii(red: u8, green: u8, blue: u8) -> char {
    // 10 ascii characters ordered from least area to most area
    // In a terminal in light mode, this is ordered from lightest to darkest, but its the opposite for dark mode terminals
    let simple_characters = " .:-=+*#%@";
    let characters = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
    // This will range from 0 to 255 too, the higher the brighter the color is
    let brightness: f32 = 0.299 * red as f32 + 0.587 * green as f32 + 0.114 * blue as f32;

    // Multiplying my 0.0392 (10/255) and rounding down, we will get a number between 0 and 9, inclusive. The lower this number is, the darker the pixels rgb value was
    let char_index: usize = (0.0392 * brightness).floor() as usize;

    //let char_index: usize = (72 - (0.28235294118 * brightness).floor() as i32) as usize;
    simple_characters.chars().nth(char_index).unwrap()
}

#[test]
fn testing() {
    // White should be brightness (@) and black should be darkest ' '
    assert_eq!(rgb_to_ascii(255, 255, 255), '$');
    assert_eq!(rgb_to_ascii(0, 0, 0), ' ');
}
