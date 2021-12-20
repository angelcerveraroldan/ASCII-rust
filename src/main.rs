/// Main function
fn main() {
    println!("Hello, world");

    // Todo: Implemnt proper testing
    testing();
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

    // This will range from 0 to 255 too, the higher the lighter the color is
    let darkness: f32 = 0.299 * red as f32 + 0.587 * green as f32 + 0.114 * blue as f32; 

    let char_index: usize = (0.0392 * darkness).floor() as usize;

    characters.chars().nth(char_index).unwrap()
}


fn testing() {
    assert_eq!(rgb_to_ascii(255, 255, 255), '@');
}