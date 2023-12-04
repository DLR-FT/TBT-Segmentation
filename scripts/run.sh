# SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
# SPDX-License-Identifier: Apache-2.0
# This script runs the provided TBT on eight logfiles and stores the results in the res-folder.
#!/bin/bash

# Build the Rust project
cargo build --release

# Define variables
previous_pwd="$(pwd)"
call="../target/release/tbt-segmentation -s -c -f"

# Get the directory of the script
script_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd $script_dir

# Use find to get a list of directories
directories=$(find "$script_dir/../res/" -mindepth 1 -type d -printf "%P\n")

# Loop through each directory
for dir in $directories; do
    dir="../res/${dir}/"
    echo "Running Lazy eval with subsampling using ${dir}"
    ../target/release/tbt-segmentation -l -c -s -f $dir> "${dir}/subsamplingAndLazy_result.txt"
    python3 infer_parameters_visualization.py "${dir}/subsamplingAndLazy_result.txt"
    echo "Running eval with subsampling using ${dir}"
    ../target/release/tbt-segmentation -c -s -f $dir> "${dir}/subsampling_result.txt"
    python3 infer_parameters_visualization.py "${dir}/subsampling_result.txt"
done

# Keep the terminal open
cd $previous_pwd
read -p "Press Enter to exit..."