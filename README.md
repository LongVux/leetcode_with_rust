# LeetCode with Rust

This repo is a Rust practice workspace. Each solved LeetCode problem lives in its own binary crate under `problems/`.

## Convention
- One folder per problem.
- Folder name: `problem-slug`
- Crate name: `lc_problem_slug`

Example:
```
problems/two-sum/
  Cargo.toml
  src/main.rs
  README.md
```

## Create a new problem
```
./scripts/new_problem.sh "Two Sum"
```

## Run a solution
From repo root:
```
cargo run -p lc_two_sum
```
