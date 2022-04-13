// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print
enum Color {
    RED,
    GREEN,
    BLUE,
    YELLOW,
}

fn print(color: Color) {
    match color {
        Color::RED => println!("color is red"),
        Color::GREEN => println!("color is green"),
        Color::BLUE => println!("color is blue"),
        Color::YELLOW => println!("color is yellow"),
    }
}

fn main() {
    print(Color::RED);
}
