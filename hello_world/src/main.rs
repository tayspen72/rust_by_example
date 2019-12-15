fn section_1_1(){
    println!("Hello world!");
    println!("I'm a Rustacean!");

    /*
    Printing is handled by a series of macros defined in std::fmt some of which include:

    format!: write formatted text to String
    print!: same as format! but the text is printed to the console (io::stdout).
    println!: same as print! but a newline is appended.
    eprint!: same as format! but the text is printed to the standard error (io::stderr).
    eprintln!: same as eprint!but a newline is appended.
    */

    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Without a suffix, 31 becomes an i32. You can change what type 31 is
    // by providing a suffix.

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number=1, width=6);

    // Rust even checks to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    // Create a structure named `Structure` which contains an `i32`.
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.

    //setting precisionfloat display will auto round! Woah!
    let pi = 3.141592;
    println!("Pi: {:.3}", pi);
}

//import std::fmt library for use
use std::fmt::{self, Formatter, Display};

// //define some structure for which fmt::display will need manual implemtation
// struct Structure(i32);

// //to use '{}' in printing, manually define the fmt::display trait
// impl fmt::Display for Structure{
//     //this implementation requires 'fmt' with this exact signature
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
//         //write structly the first element to be supplied to the output
//         //stream: 'f'. Returns fmt::Result which indicated whether operation
//         //succeeded or failed. 

//         //write syntax is very similar to println
//         write!(f, "{}", self.0)
//     }
// }

struct MinMax(i64, i64);

impl fmt::Display for MinMax{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.0, self.1)
    }
}

#[derive(Debug)]
struct ComplexNumber{
    real: f64,
    imag: f64
}

impl fmt::Display for ComplexNumber{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn section_1_2_2(){
    let minmax = MinMax(0, 14);
    println!("MinMax: {}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range);    

    let complex_one = ComplexNumber{
        real: 3.3,
        imag: 7.2
    };
    println!("Complex Number: {}", complex_one);

}

struct List(Vec<i32>);

impl fmt::Display for List{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        //Extract the value by tuple indexing
        let vec = &self.0;

        //write the opening bracket for the vector
        write!(f, "[")?;

        //iterate for all 'v' in 'vec' while enumerating ieration in 'count'
        for (count, v) in vec.iter().enumerate(){
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }

        //write the final close bracket for the vector and use return type
        write!(f, "]")
    }
}

fn section_1_2_2_1(){
    let my_vector = List(vec!(0, 2, 4));
    println!("Vector: {}", my_vector);
}

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl Display for City {
    // `f` is a buffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn section_1_2_3(){
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{:?}", *color);
    }
}

fn main() {
    section_1_1();

    section_1_2_2();

    section_1_2_2_1();

    section_1_2_3();
}
