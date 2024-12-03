# Deck sort
This project is an util to sort the yaml kong file used by deck

This implementation sorts only the vector "services" included in the yaml file by "host" value and if equals by "name" value.

No other sorts are intended. Extends if needed

## Build
```bash
cargo build --release
# output bin file will be in target/release/deck-sort
```

## Usage
Current build is released under bin folder
### Pipe concatenation
```bash
cat example/kong.yaml | ./bin/deck-sort
```
### File references
With backup
```bash
./bin/deck-sort example/kong.yaml true
```
No backup
```bash
./bin/deck-sort example/kong.yaml
```