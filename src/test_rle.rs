// [[file:../wills-columnar-format.org::#IntroductionCargotoml-cqc696o03tj0][Dependencies:7]]
use crate::rle::*;
use itertools::assert_equal;
// Dependencies:7 ends here

// [[file:../wills-columnar-format.org::#DataEncodingRunLengthEncodingTests-xhn696o03tj0][Tests:1]]
#[test]
fn test_encode_data_without_elements_produces_no_elements() {
    let data: Vec<String> = vec![];
    assert_equal(encode_iter(data.into_iter()), []);
}

#[test]
fn test_encode_data_combines_repeated_elements() {
    let data = [
        "repeated-3",
        "repeated-3",
        "repeated-3",
        "no-repeat",
        "repeated-2",
        "repeated-2",
        "repeated-3",
        "repeated-3",
        "repeated-3",
    ];
    assert_equal(
        encode_iter(data.into_iter()),
        [
            Element {
                run_length: 3,
                element: "repeated-3",
            },
            Element {
                run_length: 1,
                element: "no-repeat",
            },
            Element {
                run_length: 2,
                element: "repeated-2",
            },
            Element {
                run_length: 3,
                element: "repeated-3",
            },
        ],
    );
}
// Tests:1 ends here
