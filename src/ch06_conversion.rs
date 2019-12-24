use std::convert::From;

#[derive(Debug)]
struct Number{
    value: i32
}

//define the 'From' trait for custom objects
impl From<i32> for Number{
    fn from(item: i32) -> Self{
        Number {value: item}
    }
}

fn from(){
    let my_str = "hello";
    println!("my_str: {}", my_str);

    let my_string = String::from(my_str);
    println!("my_string: {}", my_string);

    let num = Number::from(30);
    println!("My number is: {:?}", num);
}

fn into(){
    //essentially the reciprocal of the 'from' trait
    //if you have implemented the From trait, you get Into for free!

    let int = 5;

    //try to remove the type declaration
    let num: Number = int.into();
    println!("My number is: {:?}", num);
}

use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn try_from_and_try_into(){
    //TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    //TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

use::std::fmt;

struct Circle{
    radius: i32
}

impl fmt::Display for Circle{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "Circle with radius {}", self.radius)
    }
}

fn to_and_from_strings(){
    //converting to and from a String is as simple as implementing ToString

    let circle = Circle{
        radius: 6
    };

    //to_string appears to be the same as calling std::fmt::Display
    println!("{}", circle.to_string());
    println!("{}", circle);
}

pub fn conversion() {
    // type traits allow for a simple method of converting between types

    from();

    into();

    try_from_and_try_into();

    to_and_from_strings();
}
