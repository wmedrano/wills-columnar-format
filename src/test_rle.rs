// [[file:../wills-columnar-format.org::#IntroductionCargotoml-cqc696o03tj0][Dependencies:9]]
use crate::rle::*;
use itertools::assert_equal;
// Dependencies:9 ends here

// [[file:../wills-columnar-format.org::#DataEncodingRunLengthEncoding-0vm696o03tj0][Run Length Encoding:5]]
#[test]
fn test_repeated_sum_equal_to_multiplication() {
    let rle_values = Values {
        value: 3u64,
        run_length: 5,
    };
    // Technically valid.
    assert_eq!(rle_values.repeated().sum::<u64>(), 15,);
    // More efficient.
    assert_eq!(rle_values.value * rle_values.run_length, 15);
}
// Run Length Encoding:5 ends here

// [[file:../wills-columnar-format.org::#DataEncodingRunLengthEncodingTests-xhn696o03tj0][Tests:1]]
#[test]
fn test_encode_data_without_values_produces_no_values() {
    let data: Vec<String> = vec![];
    assert_equal(encode_iter(data.into_iter()), []);
}

#[test]
fn test_encode_data_combines_repeated_values() {
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
            Values {
                run_length: 3,
                value: "repeated-3",
            },
            Values {
                run_length: 1,
                value: "no-repeat",
            },
            Values {
                run_length: 2,
                value: "repeated-2",
            },
            Values {
                run_length: 3,
                value: "repeated-3",
            },
        ],
    );
}
// Tests:1 ends here
