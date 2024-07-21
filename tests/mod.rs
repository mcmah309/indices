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

    #[test]
    fn needs_to_provide_indices_comma() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/trybuild/indices/needs_to_provide_indices_comma.rs");
        t.compile_fail("tests/trybuild/indices_ordered/needs_to_provide_indices_comma.rs");
        t.compile_fail("tests/trybuild/try_indices/needs_to_provide_indices_comma.rs");
        t.compile_fail("tests/trybuild/try_indices_ordered/needs_to_provide_indices_comma.rs");
    }

    #[test]
    fn needs_to_provide_indices() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/trybuild/indices/needs_to_provide_indices.rs");
        t.compile_fail("tests/trybuild/indices_ordered/needs_to_provide_indices.rs");
        t.compile_fail("tests/trybuild/try_indices/needs_to_provide_indices.rs");
        t.compile_fail("tests/trybuild/try_indices_ordered/needs_to_provide_indices.rs");
    }

    #[test]
    fn cannot_mutate_once_borrowed() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/trybuild/indices/cannot_mutate_once_borrowed.rs");

        t.compile_fail("tests/trybuild/try_indices/cannot_mutate_once_borrowed.rs");
    }

    #[test]
    fn cannot_return_local_data() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/trybuild/indices/cannot_return_local_data.rs");

        t.compile_fail("tests/trybuild/try_indices/cannot_return_local_data.rs");
    }
}
