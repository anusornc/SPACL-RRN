#!/bin/bash
ONTOLOGY_FILE="$1"
OPERATION="${2:-consistency}"

echo "{\"reasoner\": \"Tableauxx\", \"version\": \"0.2.0\", \"operation\": \"$OPERATION\", \"ontology\": \"$ONTOLOGY_FILE\"}"

start_time=$(date +%s%N)

if [ "$OPERATION" = "consistency" ]; then
    /build/target/release/owl2-reasoner check "$ONTOLOGY_FILE" 2>&1
elif [ "$OPERATION" = "classification" ]; then
    /build/target/release/owl2-reasoner stats "$ONTOLOGY_FILE" 2>&1
fi

end_time=$(date +%s%N)
duration_ms=$(( (end_time - start_time) / 1000000 ))

echo "{\"duration_ms\": $duration_ms, \"status\": \"completed\"}"
