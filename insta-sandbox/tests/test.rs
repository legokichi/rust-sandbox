use rstest::*;

#[rstest]
#[case(0, 0)]
#[case(1, 1)]
#[case(2, 1)]
#[case(3, 2)]
#[case(4, 3)]
#[rstest]
fn test_hello_world(#[case] input: u32, #[case] expected: u32) {
    insta::assert_debug_snapshot!(vec![input, expected]);
}