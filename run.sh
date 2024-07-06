#!/bin/bash

readonly PREFIX="$HOME/GoogleDrive/Pictures/AtCoder"
readonly ORIGINAL_IMAGE_NAME="canvas.png"

if [ $# -ne 2 ]; then
    echo "Usage: ./run.sh [CONTEST_TYPE] [CONTEST_NUMBER]" >&2
    exit 1
fi

readonly target_dir="$PREFIX/$1/$2"
readonly source_image="$target_dir/$ORIGINAL_IMAGE_NAME"

rating-graph-processor $source_image $target_dir
