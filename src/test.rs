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

// If you select some of these you may notice the statusbar char count is "wrong"
const TEXT_ENGLISH: &str = "I love you so much";
const TEXT_ARABIC: &str = "أنا أحبك جداً";
const TEXT_JAPANESE: &str = "あなたをとても愛しています";
const TEXT_CZECH: &str = "moc tě miluji";
const TEXT_RUSSIAN: &str = "Я так сильно тебя люблю";
const TEXT_CHINESE: &str = "我真的很愛你";
const TEXT_FRENCH: &str = "Je t'aime tellement";
const TEXT_SPANISH: &str = "Te amo mucho";
const TEXT_VIETNAMESE: &str = "Em yêu anh rất nhiều";
const TEXT_HEBREW: &str = "אני אוהב אותך כל כך";

#[test]
fn test_languages() {
    // p last char before word, p last char of word
    assert_eq!(TEXT_ENGLISH.nice_slice(2, 6), "love");
    assert_eq!(TEXT_ARABIC.nice_slice(0, 1), "أ");
    assert_eq!(TEXT_JAPANESE.nice_slice(0, 5), "あなたをと");
    assert_eq!(TEXT_CZECH.nice_slice(5, 11), "ě milu");
    assert_eq!(TEXT_RUSSIAN.nice_slice(7, 16), "ильно теб");
    assert_eq!(TEXT_CHINESE.nice_slice(4, 6), "愛你");
    assert_eq!(TEXT_FRENCH.nice_slice(3, 6), "t'a");
    assert_eq!(TEXT_SPANISH.nice_slice(6, 12), " mucho");
    assert_eq!(TEXT_VIETNAMESE.nice_slice(11, 19), "rất nhiề");
    assert_eq!(TEXT_HEBREW.nice_slice(1, 5), "ני א");
}

#[test]
fn readme_test() {
    let cool_days: &str = "Cool days";
    let cool = cool_days.nice_slice(0, 4);
    assert_eq!(cool, "Cool");
}
