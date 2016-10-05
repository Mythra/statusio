# Status IO #

This is a rust-lang library for the status hosting page https://status.io. It aims to implement
V2 of their api which is documented [HERE][api_doc_link]. This crate provides not only models
for decoding their responses from your own HTTP Client. The library also comes built in with
an HTTP Client for you so all you have to do is specify your api key/api id, and you're off to
the races.

## Installing the Status IO Library ##

This crate works with cargo, and is on https://crates.io.

```
[dependencies]
statusio = "1.0.0"
```

Building statusio from source should be easy the standard `cargo build` should work.

[api_doc_link]: http://docs.statusio.apiary.io/
