#!/bin/bash

# Check if a day number is provided
if [ $# -eq 0 ]; then
  echo "Please provide a day number (1-25)"
  exit 1
fi

# Pad the day number with a leading zero if needed
day=$(printf "%02d" $1)
day_no_zero=$(printf "%d" $1)

# Directories
SRC_DIR="src"
INPUTS_DIR="inputs"
TEMPLATE_FILE="new_day.tmpl"
MAIN_RS="$SRC_DIR/main.rs"

# Create day directory
mkdir -p "$SRC_DIR/day$day"

# Copy template to new module file and replace day placeholder
sed "s/%%DAY%%/$day_no_zero/g" "$TEMPLATE_FILE" >"$SRC_DIR/day$day/mod.rs"

# Add use statement to main.rs
use_statement="mod day$day;"
if ! grep -q "^$use_statement" "$MAIN_RS"; then
  # Find the last 'mod' statement and insert after it
  sed -i "/^mod day/a $use_statement" "$MAIN_RS"
fi

# Update match statement
match_statement="        $day_no_zero => day$day::run(example)?,"
# Use sed to insert the new match arm before the final _ => line
sed -i "/^        _ => return Err(anyhow!(\"Unknown day\"))/i $match_statement" "$MAIN_RS"

# Create input files
touch "$INPUTS_DIR/$day.txt"
touch "$INPUTS_DIR/${day}e.txt"

echo "Added day $day_no_zero to the project:"
echo "- Created $SRC_DIR/day$day/mod.rs (replaced %%DAY%% with $day_no_zero)"
echo "- Added use statement to $MAIN_RS"
echo "- Added match arm to $MAIN_RS"
echo "- Created $INPUTS_DIR/$day.txt"
echo "- Created $INPUTS_DIR/${day}e.txt"
