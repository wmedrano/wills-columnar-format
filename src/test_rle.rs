// [[file:../wills-columnar-format.org::*Introduction][Introduction:5]]
use crate::rle::*;
// Introduction:5 ends here

// [[file:../wills-columnar-format.org::*RLE Tests][RLE Tests:1]]
#[test]
fn test_encode_data_compacts_repeated_elements() {
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
    assert_eq!(
        encode_data(data.into_iter()).collect::<Vec<_>>(),
        vec![
            Element {
                run_length: 3,
                element: "repeated-3"
            },
            Element {
                run_length: 1,
                element: "no-repeat"
            },
            Element {
                run_length: 2,
                element: "repeated-2"
            },
            Element {
                run_length: 3,
                element: "repeated-3"
            },
        ],
    );
}
// RLE Tests:1 ends here

// [[file:../wills-columnar-format.org::*RLE Tests][RLE Tests:2]]
#[test]
fn test_decode_repeats_elements_by_run_length() {
    let data = vec![
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
    ];
    let decoded_data: Vec<&str> = decode_data(data.iter()).cloned().collect();
    assert_eq!(
        decoded_data,
        [
            "repeated-3",
            "repeated-3",
            "repeated-3",
            "no-repeat",
            "repeated-2",
            "repeated-2",
            "repeated-3",
            "repeated-3",
            "repeated-3",
        ]
    );
}
// RLE Tests:2 ends here
