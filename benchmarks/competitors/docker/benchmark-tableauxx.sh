#!/bin/bash
set -euo pipefail

ONTOLOGY_FILE="${1:-}"
OPERATION="${2:-consistency}"

if [ -z "$ONTOLOGY_FILE" ]; then
    echo "Usage: benchmark-tableauxx.sh <ontology.owl> [operation]" >&2
    exit 2
fi

LARGE_PARSE="${OWL2_REASONER_LARGE_PARSE:-1}"
MAX_FILE_SIZE="${OWL2_REASONER_MAX_FILE_SIZE:-0}"
AUTO_CACHE="${OWL2_REASONER_AUTO_CACHE:-0}"
FORCE_TEXT="${OWL2_REASONER_FORCE_TEXT:-0}"
BIN_ONLY="${OWL2_REASONER_BIN_ONLY:-0}"
DISABLE_PARSE_FALLBACK="${OWL2_REASONER_DISABLE_PARSE_FALLBACK:-}"
ENABLE_PARSE_FALLBACK="${OWL2_REASONER_ENABLE_PARSE_FALLBACK:-}"
PARSE_PROGRESS_EVERY="${OWL2_REASONER_PARSE_PROGRESS_EVERY:-}"
PARSE_IO_PROGRESS_BYTES="${OWL2_REASONER_PARSE_IO_PROGRESS_BYTES:-}"
EXPERIMENTAL_XML_PARSER="${OWL2_REASONER_EXPERIMENTAL_XML_PARSER:-}"
EXPERIMENTAL_XML_STRICT="${OWL2_REASONER_EXPERIMENTAL_XML_STRICT:-}"
EXPERIMENTAL_XML_BATCH="${OWL2_REASONER_EXPERIMENTAL_XML_BATCH:-}"
EXPERIMENTAL_XML_QUEUE="${OWL2_REASONER_EXPERIMENTAL_XML_QUEUE:-}"
EXPERIMENTAL_XML_WORKERS="${OWL2_REASONER_EXPERIMENTAL_XML_WORKERS:-}"
EXPERIMENTAL_XML_CACHE="${OWL2_REASONER_EXPERIMENTAL_XML_CACHE:-}"
STAGE_TIMING="${OWL2_REASONER_STAGE_TIMING:-}"

INPUT_FILE="$ONTOLOGY_FILE"
if [[ "$ONTOLOGY_FILE" == *.owl ]]; then
    BIN_CANDIDATE="${ONTOLOGY_FILE%.owl}.owlbin"
    if [ "$BIN_ONLY" = "1" ] && [ ! -f "$BIN_CANDIDATE" ]; then
        echo "{\"duration_ms\": -1, \"status\": \"failed\", \"reasoning_result\": \"unknown\", \"error\": \"bin_only_requested_but_missing\"}"
        exit 1
    fi
    if [ "$BIN_ONLY" = "1" ] && [ -f "$BIN_CANDIDATE" ]; then
        INPUT_FILE="$BIN_CANDIDATE"
    elif [ "$FORCE_TEXT" != "1" ] && [ -f "$BIN_CANDIDATE" ]; then
        INPUT_FILE="$BIN_CANDIDATE"
    fi
fi

echo "{\"reasoner\": \"Tableauxx\", \"version\": \"0.2.0\", \"operation\": \"$OPERATION\", \"ontology\": \"$ONTOLOGY_FILE\", \"input_file\": \"$INPUT_FILE\"}"
echo "[bench] tableauxx input_file=$INPUT_FILE" >&2

tmp_output=$(mktemp)
start_time=$(date +%s%N)

run_and_capture() {
    local output_file="$1"
    shift
    set +e
    "$@" 2>&1 | tee "$output_file"
    local status="${PIPESTATUS[0]}"
    set -e
    return "$status"
}

if [ "$OPERATION" = "consistency" ]; then
    if run_and_capture "$tmp_output" env \
        OWL2_REASONER_LARGE_PARSE="$LARGE_PARSE" \
        OWL2_REASONER_MAX_FILE_SIZE="$MAX_FILE_SIZE" \
        OWL2_REASONER_AUTO_CACHE="$AUTO_CACHE" \
        OWL2_REASONER_FORCE_TEXT="$FORCE_TEXT" \
        OWL2_REASONER_BIN_ONLY="$BIN_ONLY" \
        OWL2_REASONER_DISABLE_PARSE_FALLBACK="$DISABLE_PARSE_FALLBACK" \
        OWL2_REASONER_ENABLE_PARSE_FALLBACK="$ENABLE_PARSE_FALLBACK" \
        OWL2_REASONER_PARSE_PROGRESS_EVERY="$PARSE_PROGRESS_EVERY" \
        OWL2_REASONER_PARSE_IO_PROGRESS_BYTES="$PARSE_IO_PROGRESS_BYTES" \
        OWL2_REASONER_EXPERIMENTAL_XML_PARSER="$EXPERIMENTAL_XML_PARSER" \
        OWL2_REASONER_EXPERIMENTAL_XML_STRICT="$EXPERIMENTAL_XML_STRICT" \
        OWL2_REASONER_EXPERIMENTAL_XML_BATCH="$EXPERIMENTAL_XML_BATCH" \
        OWL2_REASONER_EXPERIMENTAL_XML_QUEUE="$EXPERIMENTAL_XML_QUEUE" \
        OWL2_REASONER_EXPERIMENTAL_XML_WORKERS="$EXPERIMENTAL_XML_WORKERS" \
        OWL2_REASONER_EXPERIMENTAL_XML_CACHE="$EXPERIMENTAL_XML_CACHE" \
        OWL2_REASONER_STAGE_TIMING="$STAGE_TIMING" \
        /opt/reasoner/owl2-reasoner check-auto "$INPUT_FILE"
    then
        cmd_status=0
    else
        cmd_status=$?
    fi
elif [ "$OPERATION" = "classification" ]; then
    if run_and_capture "$tmp_output" /opt/reasoner/owl2-reasoner stats "$ONTOLOGY_FILE"; then
        cmd_status=0
    else
        cmd_status=$?
    fi
else
    echo "{\"duration_ms\": -1, \"status\": \"unsupported_operation\", \"reasoning_result\": \"unknown\"}"
    exit 2
fi

end_time=$(date +%s%N)
duration_ms=$(( (end_time - start_time) / 1000000 ))

reasoning_result="unknown"
if grep -qi "INCONSISTENT" "$tmp_output"; then
    reasoning_result="inconsistent"
elif grep -qi "CONSISTENT" "$tmp_output"; then
    reasoning_result="consistent"
fi

parse_ms="$(grep -Eo '\[phase\][[:space:]]+parse_done[[:space:]]+ms=[0-9]+' "$tmp_output" | tail -1 | grep -Eo '[0-9]+' || true)"
reason_ms="$(grep -Eo '\[phase\][[:space:]]+reason_done[[:space:]]+ms=[0-9]+' "$tmp_output" | tail -1 | grep -Eo '[0-9]+' || true)"
if [[ -z "$parse_ms" ]]; then
    parse_ms=-1
fi
if [[ -z "$reason_ms" ]]; then
    reason_ms=-1
fi

if [ "$cmd_status" -eq 0 ]; then
    echo "{\"duration_ms\": $duration_ms, \"status\": \"completed\", \"reasoning_result\": \"$reasoning_result\", \"parse_time_ms\": $parse_ms, \"reason_time_ms\": $reason_ms}"
else
    echo "{\"duration_ms\": $duration_ms, \"status\": \"failed\", \"reasoning_result\": \"$reasoning_result\", \"parse_time_ms\": $parse_ms, \"reason_time_ms\": $reason_ms}"
fi

rm -f "$tmp_output"
exit "$cmd_status"
