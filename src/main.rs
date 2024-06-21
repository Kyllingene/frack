use frack::*;

use std::iter::Peekable;

fn main() {
    let mut args = std::env::args();
    let exe = args.next().unwrap();

    let Some(command) = args.next() else {
        let start = exe.len() + 2;
        let err = error! {
            "MISSING", "must provide command";
            "arg", 1, 1;
            exe;
            start..=start + 3 => "no command provided";
            help "try `frack help` for usage";
        };

        eprintln!("{err}");
        std::process::exit(1);
    };

    match command.as_str() {
        "help" => {
            let help = include_str!("help.txt")
                .replace("{OFF}", "\x1b[0m")
                .replace("{BOLD}", "\x1b[1m")
                .replace("{EMPH}", "\x1b[4m")
                .replace("{RED}", "\x1b[38;5;9m")
                .replace("{GREEN}", "\x1b[38;5;2m")
                .replace("{YELLOW}", "\x1b[38;5;3m")
                .replace("{BLUE}", "\x1b[38;5;12m");

            println!("{help}");
        }

        "example" => {
            let example = include_str!("example.txt")
                .replace("{OFF}", "\x1b[0m")
                .replace("{BOLD}", "\x1b[1m")
                .replace("{EMPH}", "\x1b[4m")
                .replace("{RED}", "\x1b[38;5;1m")
                .replace("{YELLOW}", "\x1b[38;5;3m")
                .replace("{GREY}", "\x1b[38;5;8m")
                .replace("{LIME}", "\x1b[38;5;10m")
                .replace("{BLUE}", "\x1b[38;5;12m");

            println!("{example}");
        }

        "error" => {
            gen(args.peekable(), true);
        }

        "warning" => {
            gen(args.peekable(), false);
        }

        other => {
            let end = other.len();
            let err = error! {
                "INVALID", "invalid command";
                "arg", 1, 1;
                other;
                0..=end => "unrecognized command";
                help "valid commands are `help`, `example`, `error`, `warning`";
                help "try `frack help` for usage";
            };

            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}

fn gen(mut args: Peekable<impl Iterator<Item = String>>, is_error: bool) {
    let (command, error_code, i) = if is_error {
        let ec = args.next().unwrap_or_else(|| {
            let err = error! {
                "MISSING", "must provide error code";
                "arg", 1, 2;
                "error";
                6..=9 => "no error code provided";
                help "try `frack help` for usage";
            };

            eprintln!("{err}");
            std::process::exit(1);
        });

        (format!("error '{ec}'"), Some(ec), 3)
    } else {
        ("warning".to_string(), None, 2)
    };

    let Some(message) = args.next() else {
        let start = command.len() + 2;
        let err = error! {
            "MISSING", "must provide message";
            "arg", 1, i;
            command;
            start..=start + 3 => "no message provided";
            help "try `frack help` for usage";
        };

        eprintln!("{err}");
        std::process::exit(1);
    };

    let Some(code) = args.next() else {
        let msg = format!("{command} '{message}'");
        let start = msg.len() + 2;
        let err = error! {
            "MISSING", "must provide code";
            "arg", 1, i + 1;
            msg;
            start..=start + 3 => "no code provided";
            help "try `frack help` for usage";
        };

        eprintln!("{err}");
        std::process::exit(1);
    };

    let (start, end) = span(
        &mut args,
        format!("{command} '{message}' '{code}'"),
        1,
        i + 2,
    );

    if is_error {
        let mut error = error! {
            error_code.unwrap(), message;
            "src/main.rs", 7, start + 1;
            code;
            start..=end;
        };

        let mut i = 2;
        while let Some(cmd) = args.next() {
            match cmd.as_str() {
                "note" => error.notes.push(note(&mut args, i)),
                "help" => error.helps.push(help(&mut args, i)),
                "fix" => error.helps.push(fix(&mut args, i)),
                other => {
                    let err = error! {
                        "INVALID", "invalid subcommand";
                        "arg", i, 1;
                        other;
                        0..=other.len() => "unrecognized subcommand";
                        help "valid subcommands are `note`, `help`, `fix`";
                        help "try `frack help` for usage";
                    };

                    eprintln!("{err}");
                    std::process::exit(1);
                }
            }

            i += 1;
        }

        println!("{error}");
    } else {
        let mut warning = warning! {
            message;
            "src/main.rs", 7, start + 1;
            code;
            start..=end;
        };

        let mut i = 2;
        while let Some(cmd) = args.next() {
            match cmd.as_str() {
                "note" => warning.notes.push(note(&mut args, i)),
                "help" => warning.helps.push(help(&mut args, i)),
                "fix" => warning.helps.push(fix(&mut args, i)),
                other => {
                    let err = error! {
                        "INVALID", "invalid subcommand";
                        "arg", i, 1;
                        other;
                        0..=other.len() => "unrecognized subcommand";
                        help "valid subcommands are `note`, `help`, `fix`";
                        help "try `frack help` for usage";
                    };

                    eprintln!("{err}");
                    std::process::exit(1);
                }
            }

            i += 1;
        }

        println!("{warning}");
    }
}

fn note(args: &mut impl Iterator<Item = String>, major: usize) -> Note {
    Note(args.next().unwrap_or_else(|| {
        let err = error! {
            "MISSING", "must provide note message";
            "arg", major, 2;
            "note";
            6..=9 => "no note message provided";
            help "try `frack help` for usage";
        };

        eprintln!("{err}");
        std::process::exit(1);
    }))
}

fn help(args: &mut impl Iterator<Item = String>, major: usize) -> Help {
    Help {
        message: args.next().unwrap_or_else(|| {
            let err = error! {
                "MISSING", "must provide help message";
                "arg", major, 2;
                "help";
                6..=9 => "no help message provided";
                help "try `frack help` for usage";
            };

            eprintln!("{err}");
            std::process::exit(1);
        }),
        suggestion: None,
    }
}

fn fix(args: &mut Peekable<impl Iterator<Item = String>>, major: usize) -> Help {
    let Some(message) = args.next() else {
        let err = error! {
            "MISSING", "must provide help message";
            "arg", major, 2;
            "fix";
            5..=8 => "no help message provided";
            help "try `frack help` for usage";
        };

        eprintln!("{err}");
        std::process::exit(1);
    };

    let Some(code) = args.next() else {
        let msg = format!("fix '{message}'");
        let start = msg.len() + 2;
        let err = error! {
            "MISSING", "must provide help code";
            "arg", major, 3;
            msg;
            start..=start + 3 => "no help code provided";
            help "try `frack help` for usage";
        };

        eprintln!("{err}");
        std::process::exit(1);
    };

    let (start, end) = span(args, format!("fix '{message}' '{code}'"), major, 4);

    let next = args.peek().map(String::as_str) == Some("span");
    let marker_message = if next {
        Some(args.nth(1).unwrap_or_else(|| {
            let msg = format!("fix '{message}' '{code}' {start}-{end} span");
            let start = msg.len() + 2;
            let err = error! {
                "MISSING", "must provide span message";
                "arg", major, 6;
                msg;
                start..=start + 3 => "no span message provided";
                help "if you don't want a span message, omit `span`";
                help "try `frack help` for usage";
            };

            eprintln!("{err}");
            std::process::exit(1);
        }))
    } else {
        None
    };

    Help {
        message,
        suggestion: Some(Code::single(
            code,
            7,
            Some(Marker {
                range: start..=end,
                symbol: '~',
                color: 2,
                message: marker_message,
                color_span: true,
            }),
        )),
    }
}

fn span(
    args: &mut impl Iterator<Item = String>,
    command: String,
    major: usize,
    minor: usize,
) -> (usize, usize) {
    let Some(span) = args.next() else {
        let start = command.len() + 2;
        let err = error! {
            "MISSING", "must provide span";
            "arg", major, minor;
            command;
            start..=start + 3 => "no span provided";
            help "try `frack help` for usage";
        };

        eprintln!("{err}");
        std::process::exit(1);
    };

    span.split_once('-')
        .and_then(|(s, e)| Some((s.parse().ok()?, e.parse().ok()?)))
        .unwrap_or_else(|| {
            let msg = format!("{command} {span}");
            let start = command.len() + 2;
            let end = msg.len() + 1;
            let err = error! {
                "INVALID", "invalid span";
                "arg", 1, 5;
                msg;
                start..=end => "invalid span";
                help "span must be in the format `start-end`, e.g. `3-15`";
                help "try `frack help` for usage";
            };

            eprintln!("{err}");
            std::process::exit(1);
        })
}
