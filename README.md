# Deck sort
This project is an util to sort the yaml kong file used by deck

This implementation sorts only the vector "services" included in the yaml file by "host" value and if equals by "name" value.

No other sorts are intended. Extends if needed

## Build
Requires rust installed
```bash
cargo build --release
# output bin file will be in target/release/deck-sort
```

## Build using docker
The output file will be placed into bin/deck-sort and it's compiled using x86_64-unknown-linux-musl target. No local rust environment is required
```bash
docker build -t deck-sort-builder .
docker create --name deck-sort-container deck-sort-builder
docker cp deck-sort-container:/output/deck-sort bin/deck-sort
docker rm deck-sort-container
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

