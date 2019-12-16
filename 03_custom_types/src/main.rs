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


fn main() {
    println!("Section 3: Custom Types");

    structures();
}