**This tool allows you to hide, extract, and manage secret messages inside PNG image chunks. It works by manipulating custom ancillary chunks without affecting the visible image.**


***The project is based on this [document](https://jrdngr.github.io/pngme_book/)***


## How to Use It

### 📥 Encode a Message

To encode a message into a specific PNG chunk, run:

```bash
cargo run -- encode ./picture.png ruSt "A secret message"
```

### 📤 Decode a Message

To decode the message stored in a given chunk, run:

```bash
cargo run -- decode ./picture.png ruSt
```

### 🗑️ Remove a Chunk

To remove a chunk from the PNG file:

```bash
cargo run -- remove ./picture.png ruSt
```

### 📄 Print All Chunks

To print all chunks in a PNG file:

```bash
cargo run -- print ./picture.png
```
