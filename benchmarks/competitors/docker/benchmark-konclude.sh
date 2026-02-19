#!/bin/bash
set -euo pipefail

ONTOLOGY_FILE="${1:-}"
OPERATION="${2:-consistency}"

if [ -z "${ONTOLOGY_FILE:-}" ]; then
    echo "Usage: benchmark-konclude.sh <ontology.owl> [operation]" >&2
    exit 2
fi

echo "{\"reasoner\": \"Konclude\", \"version\": \"0.7.0\", \"operation\": \"$OPERATION\", \"ontology\": \"$ONTOLOGY_FILE\"}"

tmp_output=$(mktemp)
start_time=$(date +%s%N)
KONCLUDE_HOME="/opt/reasoner/konclude"

if [ "$OPERATION" = "consistency" ]; then
    if (cd "$KONCLUDE_HOME" && ./Konclude consistency -i "$ONTOLOGY_FILE") > "$tmp_output" 2>&1; then
        cmd_status=0
    else
        cmd_status=$?
    fi
elif [ "$OPERATION" = "classification" ]; then
    if (cd "$KONCLUDE_HOME" && ./Konclude classification -i "$ONTOLOGY_FILE") > "$tmp_output" 2>&1; then
        cmd_status=0
    else
        cmd_status=$?
    fi
else
    if (cd "$KONCLUDE_HOME" && ./Konclude "$OPERATION" -i "$ONTOLOGY_FILE") > "$tmp_output" 2>&1; then
        cmd_status=0
    else
        cmd_status=$?
    fi
fi

end_time=$(date +%s%N)
duration_ms=$(( (end_time - start_time) / 1000000 ))

cat "$tmp_output"

reasoning_result="unknown"
if grep -Eqi "is[[:space:]]+inconsistent|[[:space:]]inconsistent\\." "$tmp_output"; then
    reasoning_result="inconsistent"
elif grep -Eqi "is[[:space:]]+consistent|[[:space:]]consistent\\." "$tmp_output"; then
    reasoning_result="consistent"
fi

# Konclude may return exit code 0 even on parser/input errors.
if grep -qi "{error}" "$tmp_output"; then
    cmd_status=1
    reasoning_result="unknown"
fi

if [ "$cmd_status" -eq 0 ]; then
    echo "{\"duration_ms\": $duration_ms, \"status\": \"completed\", \"reasoning_result\": \"$reasoning_result\"}"
else
    echo "{\"duration_ms\": $duration_ms, \"status\": \"failed\", \"reasoning_result\": \"$reasoning_result\"}"
fi

rm -f "$tmp_output"
exit "$cmd_status"
