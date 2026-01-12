use crate::{Message, StreamMessage};

const MSG_ONE: &str = r##"
--abc123
Content-Type: text/plain

Hello, this is part 1.
--abc123
Content-Type: text/plain

This is part 2.
--abc123--
"##;

const MSG_TWO: &str = r##"
--boundary42

Content-Type: text/html

<html><body>Part 1</body></html>

--boundary42
Content-Type: text/plain

Part 2 text

--boundary42--
"##;

const MSG_THREE: &str = r##"
--singlepart
Content-Type: application/json

{"key":"value"}
--singlepart--
"##;

const MSG_FOUR: &str = r##"
--weird
Content-Type: text/plain

This text mentions --weird in the body but it's not a boundary.
--weird
Content-Type: text/plain

Second part content
--weird--
"##;

const MSG_FIVE: &str = "
--rn-test\r\n
Content-Type: text/plain\r\n
\r\n
Line endings are CRLF.\r\n
--rn-test\r\n
Content-Type: text/plain\r\n
\r\n
Second part with CRLF\r\n
--rn-test--\r\n
";

#[test]
fn test() {
    let msgs = [MSG_ONE, MSG_TWO, MSG_THREE, MSG_FOUR, MSG_FIVE];
    for msg in msgs {
        let msg = msg.trim();

        let message = Message::try_parse(msg).unwrap();

        for part in message.iter() {
            println!("{part:#?}");
        }
    }

    for msg in msgs {
        let msg = msg.trim();

        let message = StreamMessage::try_parse(msg).unwrap();

        for part_result in message {
            let part = part_result.unwrap();
            println!("{part:#?}");
        }
    }
}
