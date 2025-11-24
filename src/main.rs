macro_rules! my_macro {
    () => {
        ()
    };

    ($first_func:ident $(, $rest_func:ident)*) => {
        (
            $first_func(),
            $( $rest_func() ),*
        )
    };
}

fn foo() -> i64 {
    20
}

fn bar() -> String {
    "hello world".to_owned()
}

fn baz() -> bool {
    true
}

fn main() {
    let (foo_result, bar_result, baz_result) = my_macro!(foo, bar, baz);

    println!("{}", foo_result);
    println!("{}", bar_result);
    println!("{}", baz_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_function() {
        let (result,) = my_macro!(foo);
        assert_eq!(result, 20);
    }

    #[test]
    fn test_multiple_functions() {
        let (foo_result, bar_result, baz_result) = my_macro!(foo, bar, baz);
        assert_eq!(foo_result, 20);
        assert_eq!(bar_result, "hello world");
        assert!(baz_result);
    }

    #[test]
    fn test_four_functions() {
        let (a, b) = my_macro!(foo, baz);
        assert_eq!(a, 20);
        assert!(b);
    }

    #[test]
    fn test_empty_tuple() {
        let result = my_macro!();
        assert_eq!(result, ());
    }

    #[test]
    fn test_mixed_types() {
        fn get_num() -> i32 {
            100
        }
        fn get_text() -> String {
            String::from("world")
        }

        let (num, text) = my_macro!(get_num, get_text);
        assert_eq!(num, 100);
        assert_eq!(text, "world");
    }
}
