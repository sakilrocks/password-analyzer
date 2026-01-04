# password-analyzer

A command line password strength checker written in Rust.  
It analyzes a password’s composition, estimates how long it would take to brute force and gives a simple strength rating.  

---

## Features

  - Analyzes password length
  - Detects:
      > Lowercase letters  
      > Uppercase letters  
      > Digits  
      > Symbols  
  - Calculates character set size
  - Estimates brute force crack time
  - Assigns a strength label: Weak, Medium or Strong
  - Displays a detailed breakdown of the analysis

---

## How It Works

1.	The password is scanned character by character.
2.	Flags are set based on the types of characters used.
3.	A character set size is calculated from detected character types.
4.	Total possible combinations are estimated.
5.	Crack time is calculated assuming 10^10 guesses per second.
6.	A score is computed and mapped to a strength label.


### Strength Scoring Logic

- Password length >= 12 characters: +2
- Contains lowercase letters: +1
- Contains uppercase letters: +1
- Contains digits: +1
- Contains symbols: +1

### Score mapping:
  - 0–2 -> Weak
  - 3–4 -> Medium
  - 5+ -> Strong

---

## Project Structure

```
src/
 └── main.rs      # all logic
```

---

## Build & Run

### Build

```
cargo build
```

### Run

```
cargo run -- "MyP@ssw0rd123"
```
