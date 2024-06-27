# Gather

Gather provides utilities for simplifying tricky scenarios. One such simplification is slicing text with chars of varying byte sizes, sizes that lead to uneven char positions that the standard slice[0..1] can't handle for you.

## Example Usage

After adding this crate as a dependency, to use the `Slicer` **trait** functionality add the relevant `use` import to the top of the file. You will then be given access to the relevant functions that it implements such as `str.nice_slice`.

```rust
use gather::slicing::Slicer;

fn readme_test() {
    let cool_days: &str = "Cool days";
    let cool = cool_days.nice_slice(0, 4);
    assert_eq!(cool, "Cool");
}
```
