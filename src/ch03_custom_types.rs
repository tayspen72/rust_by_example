#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    (rect.p2.x - rect.p1.x) * (rect.p2.y - rect.p1.y) 
}

fn square(p: Point, len: f32) -> Rectangle{
    let sp1 = p;
    let sp2 = Point{
        x: sp1.x + len,
        y: sp1.y + len
    };

    println!("SP1: ({}, {})", sp1.x, sp1.y);
    println!("SP2: ({}, {})", sp2.x, sp2.y);


    let rect = Rectangle{
        p1: sp1,
        p2: sp2,
    };

    rect

}

use std::fmt;

impl fmt::Display for Rectangle{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point: ({}, {})\n", self.p1.x, self.p1.y);
        write!(f, "Length: {}\n", self.p2.x - self.p1.x);
        write!(f, "Height: {}", self.p2.y - self.p1.y)
    }
}

fn structures(){
    println!("\nStructs!");

    /*
    There are three types of structures ("structs") that can be created using the struct keyword:
        Tuple structs, which are, basically, named tuples.
        The classic C structs
        Unit structs, which are field-less, are useful for generics.
    */

    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let new_point = Point { x: 0.1, ..point };
    // `new_point.y` will be the same as `point.y` because we used that field from `point`
    println!("second point: ({}, {})", new_point.x, new_point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    let rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point {x: my_x, y: my_y},
        p2: Point {x: 2.3, y: 2.4}
    };

    println!("Area of a rectangle: {}", rect_area(rectangle));

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    //create a square
    let p0 = Point { x: 0.0, y: 0.0};
    let sq0 = square(p0, 4.0);
    println!("Square: {}", sq0);
}

// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
    // An `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

// enum with implicit discriminator (starts at 0)
#[allow(dead_code)]
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
#[allow(dead_code)]
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn enums(){
    println!("\nEnums!");

    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    //type aliasing
    //if you have a difficult struct name, enum can be aliased
    type MyWebEvent = WebEvent;
    let event = MyWebEvent::KeyPress('e');
    inspect(event);

    let default_color: Color = Color::Blue;
    match default_color {
        Color::Red => println!("Default color red! {}", Number::Zero as i32),
        Color::Green => println!("Default color green! {}", Number::One as i32),
        Color::Blue => println!("Default color blue! {}", Number::Two as i32)
    }
}

// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn constants() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    // THRESHOLD = 5;
    // FIXME ^ Comment out this line
}


pub fn custom_types() {
    println!("Section 3: Custom Types");

    structures();

    enums();

    constants();
}
