fn main() {
    let error = frack::error! {
        "EGGS", "this code smells bad";
        "src/main.rs", 12, 34;
        "  rotten(eggs);";
        2..=13;
        help "clean the eggs" => ["  clean(eggs);"; 2..=12];
        help "alternatively, toss them out" => ["  drop(eggs);"];
        help "just don't keep them here";
        note "ferris has feelings too";
    };

    println!("{error}");
}
