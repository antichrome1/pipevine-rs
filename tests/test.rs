use pipevine::pipevine;

// helper
fn add(a: i32, b: i32) -> i32 {a + b}
fn mul(a: i32, b: i32) -> i32 {a * b}
fn double(x: i32) -> i32 {x * 2}
fn negate(x: i32) -> i32 {-x}
fn increment(x: i32) -> i32 {x + 1}

#[test]
fn one() {
    let result = pipevine!(5,
                           add(1, <>),
                           mul(<>, 3),
                           double,
                           add(10, mul(<>, 2)),
                           |x| x + 4,
                           <> + <> - 2,
                           if <> % 2 == 0 { <> / 2 } else { <> * 3 + 1 },
                           negate,
    );
    assert_eq!(-85, result);
}

#[test]
fn string() {
    let result = pipevine!(
        "  hello ",
        str::trim,
        <>.to_uppercase(),
        format!("{} WORLD", <>)
    );
    assert_eq!("HELLO WORLD", result);
}

#[test]
fn tuple() {
    let result = pipevine!(
        (3, 8),
        <>.0 + <>.1,
        increment
    );
    assert_eq!(12, result);
}

#[test]
fn vec() {
    let result = pipevine!(
        vec![1, 2, 3],
        <>.into_iter().map(|x| x * 2).collect::<Vec<_>>(),
        <>.iter().sum::<i32>()
    );
    assert_eq!(12, result);
}
