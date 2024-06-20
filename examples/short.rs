fn main() {
    let error = frack::error! {
        "EGGS", "this code smells bad";
        "src/main.rs", 12, 34;
        "  rotten(eggs);";
        3..=15;
        help "clean the eggs" => ["  clean(eggs);"; 3..=14];
        help "alternatively, toss them out" => ["  drop(eggs);"];
        help "just don't keep them here";
        note "ferris has feelings too";
    };

    println!("{error}");
}
