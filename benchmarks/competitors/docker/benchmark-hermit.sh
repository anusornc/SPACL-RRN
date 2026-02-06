#!/bin/sh
ONTOLOGY_FILE="$1"
OPERATION="${2:-consistency}"

echo "{\"reasoner\": \"HermiT\", \"version\": \"1.4.5.519\", \"operation\": \"$OPERATION\", \"ontology\": \"$ONTOLOGY_FILE\"}"

start_time=$(date +%s%N)

java -cp "/opt/reasoner/target/dependency/*" org.semanticweb.HermiT.cli.CommandLine -k "$ONTOLOGY_FILE" 2>&1

end_time=$(date +%s%N)
duration_ms=$(( (end_time - start_time) / 1000000 ))

echo "{\"duration_ms\": $duration_ms, \"status\": \"completed\"}"
