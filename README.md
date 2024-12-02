# advent-of-code-2023
Advent of code 2023 challenge

## Dependancies
- [all] make
- [rust] If solving using rust
  - Rust
  - [cargo] cargo-binstall
  - [cargo] cargo-generate
- [go] If solving using go
  - Go

## Go
### Create day
```bash
go run go-day/main.go 2024 1
```
- 2024: year
- 1: day
### Work with day
```bash
cd 2024/aoc2024day1
```
### Test day against example
```bash
make test
```
### Get day result
```bash
make run
```
### Benchmark all days
```bash
./bench-go.sh
```

## Rust
### Setup
Installing dependencies:
```bash
cargo install cargo-binstall
cargo binstall cargo-generate
```
### Create day
```bash
make rust-create-day -e day=1 -e year=2024
```
* day={WORKING_DAY}
* year={WORKING_YEAR}

### Run day aka getting final result
```bash
make rust-run-day -e day=1 -e year=2024
```
* day={WORKING_DAY}
* year={WORKING_YEAR}

### Test day aka getting testing against example result
```bash
make rust-run-day -e day=1 -e year=2024
```
* day={WORKING_DAY}
* year={WORKING_YEAR}

### Benchmark day ~ Documenting performance
```bash
make rust-bench-day -e day=1 -e year=2024
```
* day={WORKING_DAY}
* year={WORKING_YEAR}

### Benchmark all ~ Documenting performance
```bash
make rust-bench-all
```
