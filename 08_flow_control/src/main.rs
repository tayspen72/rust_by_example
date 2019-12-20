fn the_if_else(){
    //unlike other languages, the boolean condition does not need parenthesis
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `i32`.
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            // This expression must return an `i32` as well.
            n / 2
            // TODO ^ Try suppressing this expression with a semicolon.
        };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

    println!("{} -> {}", n, big_n);
}

// fn the_loop();

// fn the_while();

// fn the_for_and_range();

// fn the_match();

// fn the_if_loop();

// fn the_while_let();

fn main() {
    the_if_else();

    // the_loop();

    // the_while();

    // the_for_and_range();

    // the_match();

    // the_if_loop();

    // the_while_let();
}
