#!/usr/bin/env bash
# Usage: all-examples.sh <some-command>

if [ -z "$*" ]; then
  echo "Usage: $0 <some-command>"
  exit 1
fi

example_dirs=$(find $(dirname $0)/../examples/ -mindepth 1 -maxdepth 1 -type d)

for example_dir in $example_dirs; do
  pushd $example_dir 1> /dev/null
  pwd
  $@
  popd 1> /dev/null
done
