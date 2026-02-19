#!/bin/sh
set -eu

ONTOLOGY_FILE="${1:-unknown.owl}"
OPERATION="${2:-consistency}"

echo "{\"reasoner\": \"Pellet\", \"version\": \"2.x\", \"operation\": \"$OPERATION\", \"ontology\": \"$ONTOLOGY_FILE\"}"
echo "{\"duration_ms\": -1, \"status\": \"not_available\", \"reasoning_result\": \"unknown\", \"message\": \"Pellet 2.x is not reproducibly available in this containerized setup; use Openllet and JFact for open-source head-to-head results.\"}"
exit 0
