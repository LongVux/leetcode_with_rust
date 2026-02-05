#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -lt 1 ]; then
  echo "Usage: $0 \"problem name\""
  exit 1
fi

name="$*"
slug=$(printf "%s" "$name" | tr '[:upper:]' '[:lower:]' | sed -E 's/[^a-z0-9]+/-/g; s/^-+//; s/-+$//')

if [ -z "$slug" ]; then
  slug="problem"
fi

dir="problems/${slug}"
crate="lc_${slug//-/_}"

if [ -e "$dir" ]; then
  echo "Path already exists: $dir"
  exit 1
fi

mkdir -p problems
cargo new "$dir" --name "$crate" --bin --vcs none

cat <<EOT > "$dir/README.md"
# $name

- LeetCode: 
- Notes:

## Approach

## Complexity
- Time:
- Space:
EOT

echo "Created $dir (crate: $crate)"
