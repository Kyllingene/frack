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
