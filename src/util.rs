/// Create a new [Error](crate::Error).
///
/// Auto-fills certain details for you: for example, sets the proper symbols/colors for
/// [`Marker`](crate::Marker)s.
///
/// # Example
///
/// ```rust,no_run
/// let error = frack::error! {
///     "E0507", "cannot move out of `string` which is behind a shared reference";
///     "src/main", 4, 4;
///     "    let unallowed = *string;";
///     21..=28 => "move occurs because `*string` has type `String`, which does not implement the `Copy` trait";
/// };
/// ```
///
/// # Syntax
///
/// ```rust,no_run
/// # let error_code = "errcode";
/// # let message = "message";
/// # let path = "path";
/// # let line = 1;
/// # let col = 2;
/// # let code = "code";
/// # let span = 0..=3;
/// # let span_message = "span message";
/// # let help_message = "help message";
/// # let suggestion = "suggestion";
/// # let diff = 0..=9;
/// # let tip = "tip";
/// # let note = "note";
/// let error = frack::error! {
///     error_code, message;
///     path, line, col;
///     code;
///     span /* optional: */ => span_message;
///
///     // any number of:
///     help help_message /* optional: */ => [
///         suggestion
///         /* optional: */ ; diff
///         /* optional: */ ; tip
///     ];
///
///     // any number of:
///     note note;
/// };
/// ```
#[macro_export]
macro_rules! error {
    (
        $error_code:expr, $message:expr;
        $path:expr, $line:expr, $col: expr;
        $code:expr;
        $span:expr $(=> $span_message:expr)?;
        $(
            help $help:expr
            $( => [$suggestion:expr $(; $diff:expr $(; $tip:expr)? )?] )?
            ;
        )*
        $( note $note:expr; )*
    ) => {
        $crate::Error {
            error_code: $error_code.into(),
            message: $message.into(),
            file: $crate::File {
                path: $path.into(),
                line: $line,
                col: $col,
            },

            code: $crate::Code {
                code: $code.into(),
                line_number: $line,
                marker: Some($crate::Marker {
                    range: $span,
                    symbol: '^',
                    color: 9,
                    message: $crate::if_else!([$( Some($span_message.into()) )?][None]),
                    color_span: false,
                }),
            },
            helps: ::std::vec![$(
                $crate::Help {
                    message: $help.into(),
                    suggestion: $crate::if_else!([$(Some($crate::Code {
                        code: $suggestion.into(),
                        line_number: $line,
                        marker: $crate::if_else!([$(Some($crate::Marker {
                            range: $diff,
                            symbol: '~',
                            color: 10,
                            message: $crate::if_else!([$( Some($tip.into()) )?][None]),
                            color_span: true,
                        }))?][None]),
                    }))?][None]),
                },
           )*],
            notes: ::std::vec![$( $crate::Note($note.into()), )*],
        }
    };
}

/// Create a new [Warning](crate::Warning).
///
/// Auto-fills certain details for you: for example, sets the proper symbols/colors for
/// [`Marker`](crate::Marker)s.
///
/// # Example
///
/// ```rust,no_run
/// let warning = frack::warning! {
///     "unused variable: `x`";
///     "src/main", 4, 8;
///     "    let x = value;";
///     8..=8 => "help: ";
///     help "if this is intentional, prefix it with an underscore" => [
///         "    let _x = value;";
///         8..=9
///     ];
/// };
/// ```
///
/// # Syntax
///
/// ```rust,no_run
/// # let message = "message";
/// # let path = "path";
/// # let line = 1;
/// # let col = 2;
/// # let code = "code";
/// # let span = 0..=3;
/// # let span_message = "span message";
/// # let help_message = "help message";
/// # let suggestion = "suggestion";
/// # let diff = 0..=9;
/// # let tip = "tip";
/// # let note = "note";
/// let error = frack::warning! {
///     message;
///     path, line, col;
///     code;
///     span /* optional: */ => span_message;
///
///     // any number of:
///     help help_message /* optional: */ => [
///         suggestion
///         /* optional: */ ; diff
///         /* optional: */ ; tip
///     ];
///
///     // any number of:
///     note note;
/// };
/// ```
#[macro_export]
macro_rules! warning {
    (
        $message:expr;
        $path:expr, $line:expr, $col: expr;
        $code:expr;
        $span:expr $(=> $span_message:expr)?;
        $(
            help $help:expr
            $( => [$suggestion:expr $(; $diff:expr $(; $tip:expr)? )?] )?
            ;
        )*
        $( note $note:expr; )*
    ) => {
        $crate::Warning {
            message: $message.into(),
            file: $crate::File {
                path: $path.into(),
                line: $line,
                col: $col,
            },

            code: $crate::Code {
                code: $code.into(),
                line_number: $line,
                marker: Some($crate::Marker {
                    range: $span,
                    symbol: '^',
                    color: 3,
                    message: $crate::if_else!([$( Some($span_message.into()) )?][None]),
                    color_span: false,
                }),
            },
            helps: ::std::vec![$(
                $crate::Help {
                    message: $help.into(),
                    suggestion: $crate::if_else!([$(Some($crate::Code {
                        code: $suggestion.into(),
                        line_number: $line,
                        marker: $crate::if_else!([$(Some($crate::Marker {
                            range: $diff,
                            symbol: '~',
                            color: 10,
                            message: $crate::if_else!([$( Some($tip.into()) )?][None]),
                            color_span: true,
                        }))?][None]),
                    }))?][None]),
                },
           )*],
            notes: ::std::vec![$( $crate::Note($note.into()), )*],
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! if_else {
    ([][$($t:tt)*]) => {$($t)*};
    ([$($t:tt)*][$($_:tt)*]) => {$($t)*};
}
