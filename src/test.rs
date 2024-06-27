use crate::slicing::Slicer;

const TEXT_A: &str = "This is a fantastic day, a great day today. Quite a lovely day.";

#[test]
fn test_slicer_single() {
    let slice = TEXT_A.nice_slice(0, 10);
    println!("Slice: {:?}", slice);
    // note that if you're trimming the count may be less 1 char (a space)
    assert_eq!(slice.char_indices().count(), 10);
    assert_eq!(slice.len(), 10);
    assert_eq!(slice, "This is a ");
}

const EXPECTED_SLICES: [&str; 5] = ["", "This ", "is ", "a", " f"];

#[test]
fn test_slicer_multiple() {
    let slice_positions: [usize; 5] = [0, 5, 8, 9, 11];

    let mut slices: Vec<&str> = Vec::new();
    let mut p_prior: usize = 0;
    for p in slice_positions {
        let slice = TEXT_A.nice_slice(p_prior, p);
        slices.push(slice);
        assert_eq!(slice.char_indices().count(), p - p_prior);
        assert_eq!(slice.len(), p - p_prior);
        p_prior = p;
    }

    assert_eq!(slices.len(), slice_positions.len());

    for i in 0..EXPECTED_SLICES.len() {
        let expected = EXPECTED_SLICES.get(i).unwrap();
        let actual = slices.get(i).unwrap();

        println!("Slice E: {:?}", expected);
        println!("Slice A: {:?}", actual);

        assert_eq!(expected.len(), actual.len());
        assert_eq!(
            expected.char_indices().count(),
            actual.char_indices().count()
        );
        assert_eq!(expected, actual);
    }
}
