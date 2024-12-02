# Deck sort
This project is an util to sort the yaml kong file used by deck

This implementation sorts "services" by host and path values

## Usage
### pipe concatenation
```bash
cat kong.yaml | deck-sort
```
### file references
With backup
```bash
./deck-sort kong.yaml true
```
No backup
```bash
./deck-sort kong.yaml
```