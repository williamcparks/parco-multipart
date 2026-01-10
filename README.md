# Parco MultiPart

Multi part message parsing in rust

```rust,ignore
use parco_multipart::Message;

let msg = r##"
--abc123
Content-Type: text/plain

Hello, this is part 1.
--abc123
Content-Type: text/plain

This is part 2.
--abc123--
"##.trim();

let message = Message::try_parse(msg).unwrap();

for part in message.iter() {
    println!("{part:#?}");
}
```
