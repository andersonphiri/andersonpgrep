// pub mod apgrep_unit_tests;

mod tests {
    use andersonpgrep::*;
    #[test]
    fn should_find_one_line() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn should_find_one_line_ignore_case() {
        let query = "Duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn should_map_correctly() {
        let mut source = vec![1, 2, 3];
        let mapped = andersonpgrep::closures_and_iterators::closures::generic_list_map(
            &mut source,
            |x: &mut i32| {
                let res = 2 * *x;
                res
            },
        );

        assert_eq!(
            vec![2, 4, 6],
            mapped,
            "mapped vector [1,2,3] should be [2,4,6]"
        )
    }
    #[test]
    fn should_map_and_iterate_correctly() {
        let mut source = vec![1, 2, 3];
        let mapped = andersonpgrep::closures_and_iterators::closures::generic_list_map(
            &mut source,
            |x: &mut i32| {
                let res = 2 * *x;
                res
            },
        );
        let mut v_iter = mapped.iter();
        assert_eq!(v_iter.next(), Some(&2));
        assert_eq!(v_iter.next(), Some(&4));
        assert_eq!(v_iter.next(), Some(&6));
        assert_eq!(v_iter.next(), None);

        assert_eq!(
            vec![2, 4, 6],
            mapped,
            "mapped vector [1,2,3] should be [2,4,6]"
        )
    }

    #[test]
    fn should_map_and_iterate_correctly_using_built_in() {
        let source = vec![1, 2, 3, 4, 5, 6, 7];
        let mapped: Vec<_> = source.iter().filter(|num| (**num) & 1 == 1).collect();
        assert_eq!(
            vec![&1, &3, &5, &7],
            mapped,
            "should mapped to odd correctly"
        )
    }
}
