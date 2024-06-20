fn main() {
    let warn = frack::warning! {
        "this looks silly";
        "src/main.rs", 3, 4;
        "    foo(bar(baz(qux(quz()))));";
        4..=28;
        note "these are all nonsense names";
    };

    println!("{warn}");
}
