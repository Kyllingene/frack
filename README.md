# Frack

![error generated by frack](https://github.com/Kyllingene/frack/assets/69094702/83c4f1e9-ce51-4e3f-901e-7fcff619ac8a)

A fake Rust error generator - both CLI and as a lib! Can be used for joke errors
or, if you desire, real ones.

The goal is to be as accurate to real `rustc` errors/warnings as possible. If
I'm missing the mark, please make an issue or a PR.

## CLI

See `frack help`.

Command for the image:

```bash
frack
    error AMOGUS
        "this code is sus" \
        "    let Foo { x } = z;" \
        8-16 \
    fix "`y` lives matter" \
        "    let Foo { x, y } = z;" \
        15-17 \
    help "don't discriminate next time" \
    note "error generated by Kyllingene/frack"
```

## Lib

See files in `examples/`: they're small and easy-to-read. To try them out, do
`cargo run --example NAME`, e.g. `cargo run --example demo`.
