![github workflow](https://github.com/patrickomatic/a1_notation/actions/workflows/rust.yml/badge.svg)
![crates.io](https://img.shields.io/crates/v/a1_notation.svg)

# a1_notation

A Rust crate for manipulating and parsing to and from A1 notation.  A1 notation is what you 
typically see in spreadsheets where the first cell (at `(0, 0)`) is referred to as cell `A1` where
`A` represents the first column (`0`) and `1` represents the first row.

You can parse an A1-notation value using the `FromStr` trait or the `new` function:

```rust
let b5 = a1_notation::new("B5").unwrap();

assert_eq!(b5.x(), Some(1));
assert_eq!(b5.y(), Some(4));
```

## Creating a new `A1`

There are several functions you can use to create an `A1`:

```rust
// from a &str
let a1 = a1_notation::new("Foo!A1").unwrap();

assert_eq!(A1 { 
    sheet_name: Some("Foo".to_string()),
    reference: RangeOrCell::Cell(Position::Absolute(0, 0)),
}, a1);
assert_eq!("Foo!A1", a1.to_string());

// from an x/y
let b2 = a1_notation::cell(1, 1);

assert_eq!(A1 { 
    sheet_name: None,
    reference: RangeOrCell::Cell(Position::Absolute(1, 1)),
}, b2);
assert_eq!("B2", b2.to_string());

// a column reference (an `x` but no `y`)
let col_c = a1_notation::column(2);

assert_eq!(A1 { 
    sheet_name: None,
    reference: RangeOrCell::Cell(Position::ColumnRelative(2)),
}, col_c);
assert_eq!("C:C", col_c.to_string());

// a row reference (a `y` but no `x`)
let row_4 = a1_notation::row(3);

assert_eq!("4:4", row_4.to_string());
```

## Manipulating an `A1`

Once you have an `A1`, you can shift/move it around using `shift_up`, `shift_down`,
`shift_left` and `shift_right`:

```rust
let a1 = a1_notation::new("A1").unwrap();
assert_eq!(a1.shift_down(2).to_string(), "A3");

let b2 = a1_notation::new("B2").unwrap();
assert_eq!(b2.shift_down(2).shift_right(3).shift_up(1).to_string(), "E3");
```

And explicitly set it's X or Y components or sheet_name:
```rust
let a1 = a1_notation::new("A1").unwrap();
assert_eq!("F1", a1.with_x(5).to_string());

let c3 = a1_notation::new("C3").unwrap();
assert_eq!("C6", c3.with_y(5).to_string());

let in_foo_sheet = a1_notation::new("Foo!B22").unwrap();
// change the sheet name:
assert_eq!("Bar!B22".to_string(), in_foo_sheet.clone().with_sheet_name("Bar").to_string());
// or remove it:
assert_eq!("B22".to_string(), in_foo_sheet.clone().without_sheet_name().to_string());
```

## Builder

You can call the builder to build a more complex reference (with sheet name, range, etc):

```rust
let a1_absolute = A1::builder()
                    .xy(0, 0)
                    .sheet_name("Important_stuff")
                    .build()
                    .unwrap();
// Cell A1
assert_eq!(a1_absolute.to_string(), "Important_stuff!A1");

let a1_relative = A1::builder().x(0).build().unwrap();
// Column A
assert_eq!(a1_relative.to_string(), "A:A");

let a1_range = A1::builder()
                .range()
                .from(A1::builder().x(0).build().unwrap())
                .to(A1::builder().x(3).build().unwrap())
                .build()
                .unwrap();
// Range A:D
assert_eq!(a1_range.to_string(), "A:D");
```

### A1 Reference Examples

Here is a table illustrating A1 references:

| **Reference**   | **Meaning**               |
|:----------------|:--------------------------|
| `"A1"`          | Cell A1                   |
| `"A1:B5"`       | Cells A1 through B5       |
| `"C5:D9,G9:H16"`| A multiple-area selection |
| `"A:A"`         | Column A                  |
| `"1:1"`         | Row 1                     |
| `"A:C"`         | Columns A through C       |
| `"1:5"`         | Rows 1 through 5          |
| `"1:1,3:3,8:8"` | Rows 1, 3, and 8          |
| `"A:A,C:C,F:F"` | Columns A, C, and F       |

For more info take a look at the [package on crates.io](https://crates.io/crates/a1_notation/) and it's [Rust docs](https://docs.rs/a1_notation/latest/a1_notation/).

## Additional Reading

* [Refer to Cells and Ranges by Using A1 Notation](https://learn.microsoft.com/en-us/office/vba/excel/concepts/cells-and-ranges/refer-to-cells-and-ranges-by-using-a1-notation)
* [Google Sheets API Overview](https://developers.google.com/sheets/api/guides/concepts)
