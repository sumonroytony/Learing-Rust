// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
enum Flavor{
    Sweet,
    Crispy,
    Hot
}

struct Drink{
    flavor: Flavor,
    fluid: f64
} 

fn print_drink(drink: Drink){
    match drink.flavor {
        Flavor::Sweet => println!("Sweet!"),
        Flavor::Crispy => println!("Crispy!"),
        Flavor::Hot => println!("Hot!"),
    }
    println!("oz: {:?}", drink.fluid);
}
fn main() {
    let sweet = Drink{
        flavor: Flavor::Crispy,
        fluid: 6.0
    };

    print_drink(sweet);

    let sweet2 = Drink{
        flavor: Flavor::Sweet,
        fluid: 6.0
    };

    print_drink(sweet2);

    let sweet3 = Drink{
        flavor: Flavor::Hot,
        fluid: 6.0
    };

    print_drink(sweet3);
}
