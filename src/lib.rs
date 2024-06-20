//! # pipe_macros
//! A small macro library that allows you to pipe functions
//! similar to the pipe operator in Elixir and F# (|>)

// #![deny(missing_docs)]
#![deny(warnings)]

/// Internal
#[macro_export]
macro_rules! __internal_pipe_fun {
    (&, $ret:expr) => {
        &$ret
    };
    ((as $typ:ty), $ret:expr) => {
        $ret as $typ
    };
    (($funs_head:tt $(:: $funs_tail:tt)* ($($arg:expr),*)), $ret:expr) => {
        $funs_head $(:: $funs_tail)* ($ret $(,$arg)*)
    };
    (($fun:ident!($($arg:expr),*)), $ret:expr) => {
        $fun!($ret $(,$arg)*)
    };
    (($fun:expr), $ret:expr) => {
        $fun($ret)
    };
    ($fun:path, $ret:expr) => {
        $fun($ret)
    };
    (!, $fun:ident, $ret:expr) => {
        $fun!($ret)
    }
}

/// The pipe operator |> allows you to establish "pipelines" of functions in a flexible manner.
/// ```rust
/// fn times(a: u32, b: u32) -> u32{
///     a * b
/// }
///
/// fn times2(n: u32) -> u32 {
///     times(n, 2)
/// }
///
/// // Passes the preceding expression as the only argument of proceding function.
/// let num = pike::pipe!(
///   2
///   |> times2
///   |> times2
/// );
/// assert_eq!(num, 2 * 2 * 2);
///
/// // Passes the preceding expression as the first argument of proceding function.
/// // by wrapping the function in parentheses we can pass the remanining arguments by partially
/// // calling the `times` as `times(?, 2)` and passing 2 as its first argument via the pipeline.
/// let num = pike::pipe!(
///   1
///   |> (times(2))
///   |> (times(3))
/// );
/// assert_eq!(num, 1 * 2 * 3);
///
/// // call a method using pipelines
/// let len = pike::pipe!(
///   "abcd"
///   |> str::len
/// );
/// assert_eq!(len, "abcd".len());
///
/// // Closures can also be pipelined similar to partial functions.
/// let c = pike::pipe!(
///   ['a', 'b', 'c', 'd']
///   |> (|it: [char; 4]| it[2])
/// );
/// assert_eq!(c, 'c');
///
/// // Piping through `&` symbol would get a reference to the preceding expression.
/// let it = "it";
/// let is_it = |r: &&str| it == *r;
///
/// let is_it = pike::pipe!(
///   it
///   |> &
///   |> is_it
/// );
/// assert_eq!(is_it, true);
///
/// // takes a string length, doubles it and converts it back into a string
/// let len = pike::pipe!(
///     "abcd"
///     |> str::len
///     |> (as u32)
///     |> (times(2))
///     |> &
///     |> u32::to_string
/// );
///
/// assert_eq!(len, "8");
/// ```
#[macro_export]
macro_rules! pipe {
    ($head:tt $(|> $funs_head:tt $(:: $funs_tail:tt)*)+) => {
        {
        let ret = $head;
        $(
            let ret = $crate::__internal_pipe_fun!($funs_head $(:: $funs_tail)*, ret);
        )+
            ret
        }
    }
}

/// Works similar to `pipe` but `pipe_res` exits the pipeline early if a function returns an Err()
/// ```rust,ignore
/// let result = pike::pipe_res!("http://rust-lang.org" |> download |> parse |> get_links)
/// ```
#[macro_export]
macro_rules! pipe_res {
    ($head:tt $(|> $funs_head:tt $(:: $funs_tail:tt)*)+) => {
        {
            let ret = Ok($head);
            $(
                let ret = match ret {
                    Ok(x) => $crate::__internal_pipe_fun!($funs_head $(:: $funs_tail)*, x),
                    _ => ret
                };
            )*
            ret
        }
    };
}

/// Works similar to `pipe_res` but `pipe_opt` exits the pipeline early if a function returns an None()
/// ```rust,ignore
/// let result = pike::pipe_res!("http://rust-lang.org" |> download |> parse |> get_links)
/// ```
#[macro_export]
macro_rules! pipe_opt {
    ($head:tt $(|> $funs_head:tt $(:: $funs_tail:tt)*)+) => {
        {
            let ret = None;
            $(
                let ret = match ret {
                    None => $crate::__internal_pipe_fun!($funs_head $(:: $funs_tail)*, $head),
                    _ => ret
                };
            )*
            ret
        }
    };
}

#[cfg(test)]
mod test_pipe_opt {
    fn times2(a: u32) -> Option<u32> {
        return Some(a * 2);
    }

    fn nope(_a: u32) -> Option<u32> {
        return None;
    }

    #[test]
    fn accepts_options() {
        let ret = pipe_opt!(
            4
            |> times2
        );

        assert_eq!(ret, Some(8));
    }

    #[test]
    fn accepts_unwrap() {
        let ret = pipe_opt!(
            4
            |> times2
        )
        .unwrap();

        assert_eq!(ret, 8);
    }

    #[test]
    fn exits_early() {
        let ret = pipe_opt!(
            4
            |> times2
            |> times2
            |> times2
        );

        assert_eq!(ret, Some(8));
    }

    #[test]
    fn goes_until_some() {
        let ret = pipe_opt!(
            4
            |> nope
            |> nope
            |> (|_i: u32| None)
            |> times2
            |> nope
        );

        assert_eq!(ret, Some(8));
    }

    #[test]
    fn ends_with_none() {
        let ret = pipe_opt!(
            4
            |> nope
            |> nope
            |> (|_i| None)
            |> nope
        );

        assert_eq!(ret, None);
    }
}

#[cfg(test)]
mod test_pipe_res {
    fn times2(a: u32) -> Result<u32, String> {
        return Ok(a * 2);
    }

    fn fail_if_over_4(a: u32) -> Result<u32, String> {
        if a > 4 {
            return Err("This number is larger than four".to_string());
        }
        return Ok(a);
    }

    #[test]
    fn accepts_results() {
        let ret = pipe_res!(
            4
            |> times2
        );

        assert_eq!(ret, Ok(8));
    }

    #[test]
    fn accepts_unwrap() {
        let ret = pipe_res!(
            4
            |> times2
        )
        .unwrap();

        assert_eq!(ret, 8);
    }

    #[test]
    fn chains_result_values() {
        let ret = pipe_res!(
            4
            |> times2
            |> times2
            |> times2
        );

        assert_eq!(ret, Ok(32));
    }

    #[test]
    fn exits_early() {
        let ret = pipe_res!(
            4
            |> times2
            |> fail_if_over_4
            |> times2
            |> times2
        );

        assert_eq!(ret, Err("This number is larger than four".to_string()));
    }
}

#[cfg(test)]
mod test_pipe {
    fn times2(a: u32) -> u32 {
        return a * 2;
    }

    fn times(a: u32, b: u32, c: u32) -> u32 {
        return a * b * c;
    }

    #[test]
    fn test_int() {
        let multiply = |i: u32| i * 2;
        let ret = pipe!(
            4
            |> times2
            |> (|i: u32| i * 2)
            |> multiply
            |> (times(100, 10))
        );

        assert_eq!(ret, 32000);
    }

    #[test]
    fn test_string() {
        let ret = pipe!(
            "abcd"
            |> str::len
            |> (as u32)
            |> times2
            |> (times(100, 10))
            |> &
            |> u32::to_string
        );

        //let ret = "abcd";
        //let ret = ret.len();
        //let ret = ret as u32;
        //let ret = times2(ret);
        //let ret = times(ret, 100, 10);
        //let ret = ret.to_string();

        assert_eq!(ret, times(times2("abcd".len() as u32), 100, 10).to_string());
        assert_eq!(ret, "8000");
    }
}
