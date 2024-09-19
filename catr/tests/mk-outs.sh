#!/usr/bin/env bash

set -u

ROOT="catr/tests/inputs"
OUT_DIR="catr/tests/expected"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

EMPTY="$ROOT/empty.txt"
ONE_LINE="$ROOT/one-line.txt"
THREE_LINES="$ROOT/three-lines.txt"
MULTIPLE_LINES="$ROOT/multiple-lines.txt"
ALL="$EMPTY $ONE_LINE $THREE_LINES $MULTIPLE_LINES"

for FILE in $ALL; do
    BASENAME=$(basename "$FILE")
    cat    $FILE > ${OUT_DIR}/${BASENAME}.out
    cat -n $FILE > ${OUT_DIR}/${BASENAME}.n.out
    cat -b $FILE > ${OUT_DIR}/${BASENAME}.b.out
done

cat    $ALL > $OUT_DIR/all.out
cat -n $ALL > $OUT_DIR/all.n.out
cat -b $ALL > $OUT_DIR/all.b.out

cat    < $MULTIPLE_LINES > $OUT_DIR/$(basename $MULTIPLE_LINES).stdin.out
cat -n < $MULTIPLE_LINES > $OUT_DIR/$(basename $MULTIPLE_LINES).n.stdin.out
cat -b < $MULTIPLE_LINES > $OUT_DIR/$(basename $MULTIPLE_LINES).b.stdin.out
