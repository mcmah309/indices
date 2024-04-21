

#[cfg(test)]
pub mod should_not_compile_tests {

    #[test]
    fn no_negative_arguments() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/trybuild/indices/no_negative_arguments.rs");
        t.compile_fail("tests/trybuild/indices_ordered/no_negative_arguments.rs");
        t.compile_fail("tests/trybuild/try_indices/no_negative_arguments.rs");
        t.compile_fail("tests/trybuild/try_indices_ordered/no_negative_arguments.rs");
    }
}