# Parco MultiPart

Multi part message parsing in rust

```rust,ignore
use parco_multipart::Message;

const MSG: &str = r##"
--abc123
Content-Type: text/plain

Hello, this is part 1.
--abc123
Content-Type: text/plain

This is part 2.
--abc123--
"##;

let message = Message::try_parse(MSG).unwrap();

for part in message.iter() {
    println!("{part:#?}");
}
```
