# Nost

Nost, Note + Rust is a tool for extracting and publishing some parts of Not texts.

## Commands

Not implemented

### Get some stats from not files

```
cargo run stats
```

Give some stats about the not instance.

### Extract content from not files

```
cargo run extract <keyword>
```

Extract all the comments with the specific keyword.

### Append content to a not file

```
cargo run add-file-content ?<date[yyyy-mm-dd]>
```
Append content on this file.

Or 

```
cargo run add-file-content
```

Append content to the last existing file in not.