use image;

/// Main function
fn main() {
    // Open image and turn and turn it into a Vector of its pixels
    let img = image::open("./src/images/eagle.jpeg").unwrap().into_bytes();
    // Todo, change images width
    let width = 100;
    let pixel_count = img.len();

    for i in 0..(pixel_count / 3) {
        let c: char = rgb_to_ascii(img[i * 3], img[(i * 3) + 1], img[(i * 3) + 2]);

        // Print character twice to make image look nicer
        print!("{}{}", c, c);
        if i % width == 0 {
            println!();
        }
    }
}

/// Given an rgb value, return an ascii character.
///
/// ```
/// assert_eq!(rgb_to_ascii(12, 56, 12), ':');
///
/// // This is the brightest possible character to be returned.
/// assert_eq!(rgb_to_ascii(255, 255, 255), '@');
/// ```
/// The charactes that can be returned are, in order from darkest to lightest: ``` .:-=+*#%@```
fn rgb_to_ascii(red: u8, green: u8, blue: u8) -> char {
    // 10 ascii characters ordered from lesat area to most area
    // In a terminal in light mode, this is ordered from lightest to darkest, but its the opposite for dark mode terminals
    let characters = " .:-=+*#%@";

    // This will range from 0 to 255 too, the higher the brighter the color is
    let brightness: f32 = 0.299 * red as f32 + 0.587 * green as f32 + 0.114 * blue as f32;

    // Multiplying my 0.0392 (10/255) and rounding down, we will get a number between 0 and 9, inclusive. The lower this number is, the darkers the pixels rgb value was
    let char_index: usize = (0.0392 * brightness).floor() as usize;
    characters.chars().nth(char_index).unwrap()
}

#[test]
fn testing() {
    // White should be brighterst (@) and black shoud be darkest ' '
    assert_eq!(rgb_to_ascii(255, 255, 255), '@');
    assert_eq!(rgb_to_ascii(0, 0, 0), ' ');
}
