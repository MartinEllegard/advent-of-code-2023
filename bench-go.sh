#!/bin/bash

echo "# Bench Go Projects" >go-benchmarks.md

# loop for every year folder in this dierctory
for year in $(ls -d 20*); do
  echo "## $year" >>go-benchmarks.md
  # loop for every project in the year folder
  for project in $(ls -d "$year"/*); do
    if [ -f "$project/go.mod" ]; then
      echo "### $project" >>go-benchmarks.md
      echo "Running $project"
      cd "$project" || exit
      make bench | grep "^Benchmark" >>../../go-benchmarks.md
      cd "../../"
    fi
  done
done
