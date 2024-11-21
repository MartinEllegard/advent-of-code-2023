# Use `just work day-01 part1` to work on the specific binary for a specific day's problems
rust-run-day:
	@cargo run aoc${year}day${day}
rust-lint-day:
	@cargo clippy -p aoc${year}day${day}
rust-test-day:
	@cargo test -p aoc${year}day${day}
rust-bench-all:
	@cargo bench -q > benchmarks.txt
rust-bench-day:
	@cargo bench --bench bench-aoc${year}day${day} >> ${year}/${day}.bench.txt
rust-create-day:
	@cargo generate --path ./rust-day --destination ./${year}/ --name aoc${year}day${day}
