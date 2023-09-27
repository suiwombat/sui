/// Returns `Some(output)` in case the given expression matches one of the given patterns
/// of the form `pattern => output`, and None otherwise.
///
/// Like in a `match` expression, the pattern can be optionally followed by `if`
/// and a guard expression that has access to names bound by the pattern. There can be several patterns.
///
/// # Examples
///
/// ```
/// use match_opt::match_opt;
/// let foo = 'f';
/// let some_true = match_opt!(foo, 'A'..='Z' | 'a'..='z' => true);
///
/// let bar = Some(4);
/// let some_42 = match_opt!(bar, Some(x) if x > 2 => 42);
/// ```
#[macro_export]
macro_rules! match_opt {
    ($expression:expr, $( $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)? => $output:expr ),* ) => {
        match $expression {
            $(  $( $pattern )|+ $( if $guard )? => Some($output), )*
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn cond_opt_positive_one() {
        let result = |v| match_opt!(v,  x if x > 0 => 42);
        assert_eq!(result(5), Some(42));
        assert_eq!(result(0), None);
    }

    #[test]
    fn cond_opt_positive_several() {
        let result = |v| match_opt!(v, y if y > 5 => y + 1, x if x > 0 => 42);
        assert_eq!(result(5), Some(42));
        assert_eq!(result(7), Some(8));
        assert_eq!(result(0), None);
    }

    #[test]
    fn cond_opt_positive_none() {
        let result = |v| match_opt!(v,);
        // type inference requires typing the output
        let foo: Option<usize> = result(5);
        assert_eq!(foo, None);
    }

    #[test]
    fn cond_opt_positive_many() {
        let result = |v| match_opt!(v,  z if z > 10 => z -1, y if y > 5 => y + 1, x if x > 0 => 42);
        assert_eq!(result(5), Some(42));
        assert_eq!(result(7), Some(8));
        assert_eq!(result(11), Some(10));
        assert_eq!(result(0), None);
    }
}
