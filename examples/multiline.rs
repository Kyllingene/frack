use frack::{Code, Error, File, Help, Line, Marker};

fn main() {
    let code = Code(vec![
        Line {
            code: "fn foo() -> String {".to_string(),
            line_number: 3,
            marker: Some(Marker {
                range: 12..=18,
                symbol: '-',
                color: 12,
                message: Some("expected `String` because of return type".to_string()),
                color_span: false,
            }),
        },
        Line {
            code: "    12_i32".to_string(),
            line_number: 7,
            marker: Some(Marker {
                range: 4..=9,
                symbol: '^',
                color: 9,
                message: Some("expected `String`, found `i32`".to_string()),
                color_span: false,
            }),
        },
    ]);

    let error = Error {
        error_code: "E0308".to_string(),
        message: "mismatched types".to_string(),
        file: File {
            path: "main.rs".to_string(),
            line: 7,
            col: 5,
        },

        code,
        helps: vec![Help {
            message: "consider using the available `ToString` impl".to_string(),
            suggestion: Some(Code::single(
                "    12_i32.to_string()",
                7,
                Some(Marker {
                    range: 10..=21,
                    symbol: '~',
                    color: 10,
                    message: Some("convert this into a `String`".to_string()),
                    color_span: true,
                }),
            )),
        }],
        notes: Vec::new(),
    };

    println!("{error}");
}
