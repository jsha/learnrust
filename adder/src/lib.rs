use std::{
    cmp::{max, min},
    io::stdin,
};

pub fn ask_and_reply() {
    let x = get_number("first");
    let y = get_number("second");
    let result = mul(x, y);

    println!("Upon multiplying {} and {}, we get {}", x, y, result);
}

fn get_number(which: &str) -> i64 {
    println!("What would you like the {} number to be:", which);
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim_end().parse().unwrap()
}

fn mul(x: i64, y: i64) -> i64 {
    let mut accum = 0;
    let negative: bool = min(x, y) < 0 && max(x, y) > 0;

    let iterations = min(x.abs(), y.abs());
    let step = max(x.abs(), y.abs());

    let mut i = 0;
    while i < iterations {
        if i == 0 || i + i > iterations {
            accum += step;
            i += 1;
        } else {
            accum += accum;
            i += i;
        }
    }

    if negative {
        0 - accum
    } else {
        accum
    }
}

#[test]
fn test_add() {
    assert_eq!(mul(0, 0), 0);
    assert_eq!(mul(0, 1), 0);
    assert_eq!(mul(1, 0), 0);
    assert_eq!(mul(-1, 0), 0);
    assert_eq!(mul(0, -1), 0);
    assert_eq!(mul(0, 1_000_000), 0);

    assert_eq!(mul(1, 7), 7);
    assert_eq!(mul(7, 1), 7);
    assert_eq!(mul(7, 3), 21);
    assert_eq!(mul(3, 7), 21);
    assert_eq!(mul(-3, -7), 21);

    assert_eq!(mul(-3, 7), -21);
    assert_eq!(mul(3, -7), -21);

    assert_eq!(mul(3, 1_000_000_000), 3_000_000_000);
    assert_eq!(mul(1_000_000_000, 3), 3_000_000_000);

    assert_eq!(mul(-1_000_000_000, 3), -3_000_000_000);

    assert_eq!(mul(1_000_000_000, 1_000_000_000), 1_000_000_000_000_000_000);
}
