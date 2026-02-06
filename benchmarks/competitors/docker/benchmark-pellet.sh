#!/bin/sh
ONTOLOGY_FILE="$1"
OPERATION="${2:-consistency}"

echo "{\"reasoner\": \"Pellet/Openllet\", \"version\": \"2.5.1", \"operation\": \"$OPERATION\", \"ontology\": \"$ONTOLOGY_FILE\"}"

start_time=$(date +%s%N)

if [ "$OPERATION" = "consistency" ]; then
    java -cp ".:target/dependency/*" PelletConsistency "$ONTOLOGY_FILE" 2>&1
else
    echo "Operation $OPERATION not yet supported"
fi

end_time=$(date +%s%N)
duration_ms=$(( (end_time - start_time) / 1000000 ))

echo "{\"duration_ms\": $duration_ms, \"status\": \"completed\"}"
