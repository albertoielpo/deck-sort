# Deck sort
This project is an util to sort the yaml kong file used by deck

This implementation sorts only the vector "services" included in the yaml file by "host" value and if equals by "path" value.

No other sorts are intended. Extends if needed

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