# Use `just work day-01 part1` to work on the specific binary for a specific day's problems
rust-run-day:
	@cargo run aoc${year}day${day}
rust-lint-day:
	@cargo clippy -p aoc${year}day${day}
rust-test-day:
	@cargo test -p aoc${year}day${day}
rust-bench-all:
	@echo "# Rust aoc benchmarks" > rust-benchmarks.md
	@cargo bench -q --workspace >> rust-benchmarks.md
rust-bench-day:
	@cargo bench --bench bench-aoc${year}day${day} >> ${year}/aoc${year}day${day}/bench.txt
rust-create-day:
	@cargo generate --path ./rust-day --destination ./${year}/ --name aoc${year}day${day}
