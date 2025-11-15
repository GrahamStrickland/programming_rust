use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

struct Anime {
    name: &'static str,
    bechdel_pass: bool,
}

struct Point {
    x: i32,
    y: i32,
}

static mut STASH: &i32 = &128;
fn f(p: &'static i32) {
    // still not good enough
    unsafe {
        STASH = p;
    }
}

static WORTH_POINTING_AT: i32 = 1000;

// This does not compile.
struct S<'a> {
    r: &'a i32,
}

struct D<'a> {
    s: S<'a>, // not adequate
}

fn main() {
    let mut table = Table::new();

    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    show(&table);
    assert_eq!(table["Gesualdo"][0], "many madrigals");

    sort_works(&mut table);
    show(&table);

    let x = 10;
    let r = &x; // &x is a shared reference to x
    assert!(*r == 10); // explicity dereference r

    let mut y = 32;
    let m = &mut y; // &mut y is a mutable reference to y
    *m += 32; // explicitly dereference m to set y's value
    assert!(*m == 64); // and to see y's new value

    let aria = Anime {
        name: "Aria: The Animation",
        bechdel_pass: true,
    };
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");

    // Equivalent to the above, but with the dereference written out:
    assert_eq!((*anime_ref).name, "Aria: The Animation");

    let mut v = vec![1973, 1968];
    v.sort(); // implicitly borrows a mutable reference to v 
    (&mut v).sort(); // equivalent, but more verbose

    let x = 10;
    let y = 20;
    let mut r = &x;
    let b = true;

    if b {
        r = &y;
    }

    assert!(*r == 10 || *r == 20);

    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;

    assert_eq!(rrr.y, 729);

    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    assert!(rrx <= rry);
    assert!(rrx == rry);

    assert!(rx == ry); // their referents are equal
    assert!(!std::ptr::eq(rx, ry)); // but occupy different addresses

    // assert!(rx == rrx);     // error: type mismatch: `&i32` vs `&&i32`
    assert!(rx == *rrx); // this is okay

    let r = &factorial(6);
    // Arithmetic operators can see through one level of references.
    assert_eq!(r + &1009, 1729);

    {
        let r;
        {
            let x = 1;
            r = &x;
        }
        // assert_eq!(*r, 1); // bad: reads memory `x` used to occupy
    }

    {
        let x = 1;
        {
            let r = &x;
            // ...
            assert_eq!(*r, 1);
            // ...
        }
    }

    let v = vec![1, 2, 3];
    let r = &v[1];

    f(&WORTH_POINTING_AT);

    let x = 10;
    g(&x);
    // f(&x);

    let s;
    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        s = smallest(&parabola);
        assert_eq!(*s, 0); // fine: parabola still alive
    }
    // assert_eq!(*s, 0); // bad: points to elements of dropped array

    let s;
    {
        let x = 10;
        s = S { r: &x };
    }
    // assert_eq!(*s.r, 10); // bad: reads from dropped `x`
}

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

fn factorial(n: usize) -> usize {
    (1..n + 1).product()
}

// This could be written more briefly: fn g(p: &i32),
// but let's write out the lifetimes for now.
fn g<'a>(p: &'a i32) { /* ... */
}

// v should have at least one element.
fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}
