The project is based on this document: https://jrdngr.github.io/pngme_book/


How to use it?

To encode a message into a particular chunk run the following command:
cargo run -- encode ./picture.png ruSt "A secret message"

To decode the message stored in a given chunk run this:
cargo run -- decode ./picture.png ruSt

Remove a chunk:
cargo run -- remove ./picture.png ruSt

Print all of the chunks in a png file:
cargo run -- print ./picture.png
