use frack::{Code, Error, File, Help, Marker, Note};

fn main() {
    let code = Code {
        code: "    let Foo { x } = z;".to_string(),
        line_number: 8,
        marker: Some(Marker {
            range: 8..=16,
            symbol: '^',
            color: 9,
            message: Some("what'd field `y` ever do to you?".to_string()),
            color_span: false,
        }),
    };

    let fix = Code {
        code: "    let Foo { x, y } = z;".to_string(),
        line_number: 8,
        marker: Some(Marker {
            range: 15..=17,
            symbol: '~',
            color: 10,
            message: None,
            color_span: true,
        }),
    };

    let error = Error {
        error_code: "AMOGUS".to_string(),
        message: "this code is sus".to_string(),
        file: File {
            path: "src/some_random_file.rs".to_string(),
            line: 8,
            col: 9,
        },

        code,
        helps: vec![
            Help {
                message: "`y` lives matter".to_string(),
                suggestion: Some(fix),
            },
            Help {
                message: "don't discriminate next time".to_string(),
                suggestion: None,
            },
        ],
        notes: vec![Note("error generated by Kyllingene/frack".to_string())],
    };

    println!("{error}");
}
