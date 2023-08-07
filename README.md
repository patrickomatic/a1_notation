![github workflow](https://github.com/patrickomatic/a1_notation/actions/workflows/rust.yml/badge.svg)
![crates.io](https://img.shields.io/crates/v/a1_notation.svg)

# a1_notation

A Rust crate for manipulating and parsing to and from A1 notation.  A1 notation is what you 
typically see in spreadsheets where the first cell (at `(0, 0)`) is referred to as cell `A1` where
`A` represents the first column (`0`) and `1` represents the first row.

You can parse an A1-notation value using the `FromStr` trait:

```
use a1_notation::A1;
let a1 = A1::from_str("A1").unwrap();

assert_eq!(a1.x(), Some(0));
assert_eq!(a1.y(), Some(0));
```

Or if you have absolute coordinates and want to build A1 notation, you can use the builder
functions:

```
// Cell A1
let a1_absolute = A1::builder()
    .xy(0, 0)
    .sheet_name("Important_stuff")
    .build()
    .unwrap();

assert_eq!(a1_absolute.to_string(), "Important_stuff!A1");

// Column A
let a1_relative = A1::builder().x(0).build().unwrap();

assert_eq!(a1_relative.to_string(), "A:A");

// Range A:D
let a1_range = A1::builder()
    .range()
    .from(A1::builder().x(0).build().unwrap())
    .to(A1::builder().x(3).build().unwrap())
    .build()
    .unwrap();
    
assert_eq!(a1_range.to_string(), "A:D");
```

Once you have an `A1`, you can shift/move it around using `shift_up`, `shift_down`,
`shift_left` and `shift_right`:

```
use a1_notation::Shift;
let a1 = A1::from_str("C3").unwrap();

assert_eq!(a1.clone().shift_down(2).to_string(), "C5");
assert_eq!(a1.clone().shift_right(1).to_string(), "D3");
assert_eq!(a1.clone().shift_left(1).to_string(), "B3");
assert_eq!(a1.clone().shift_up(2).to_string(), "C1");
```

Here is a table illustrating A1 references:

|   **Reference**   |   **Meaning**             |
|:------------------|:--------------------------|
| `"A1"`            | Cell A1                   |
| `"A1:B5"`         | Cells A1 through B5       |
| `"C5:D9,G9:H16"`  | A multiple-area selection |
| `"A:A"`           | Column A                  |
| `"1:1"`           | Row 1                     |
| `"A:C"`           | Columns A through C       |
| `"1:5"`           | Rows 1 through 5          |
| `"1:1,3:3,8:8"`   | Rows 1, 3, and 8          |
| `"A:A,C:C,F:F"`   | Columns A, C, and F       |

For more info take a look at the [package on crates.io](https://crates.io/crates/a1_notation/) and it's [Rust docs](https://docs.rs/a1_notation/latest/a1_notation/).

## Additional Reading

* [Refer to Cells and Ranges by Using A1 Notation](https://learn.microsoft.com/en-us/office/vba/excel/concepts/cells-and-ranges/refer-to-cells-and-ranges-by-using-a1-notation)
* [Google Sheets API Overview](https://developers.google.com/sheets/api/guides/concepts)
