# Deck sort
This project is an util to sort the yaml kong file used by deck

This implementation sorts "services" by host and path values

## Build
```bash
cargo build --release
# output bin file will be in target/release/deck-sort
cp target/release/deck-sort deck-sort
chmod +x deck-sort
```

## Usage

### Pipe concatenation
```bash
cat example/kong.yaml | ./deck-sort
```
### File references
With backup
```bash
./deck-sort example/kong.yaml true
```
No backup
```bash
./deck-sort example/kong.yaml
```