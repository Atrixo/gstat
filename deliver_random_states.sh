#!/bin/sh
set -e

function postState() {
    # Get a random numbers in [0, 100)
    RMS=$((`od -An -N2 -tu2 /dev/urandom` % 100))
    PEAK=$((`od -An -N2 -tu2 /dev/urandom` % 100))
    WHATEVER=$((`od -An -N2 -tu2 /dev/urandom` % 100))

    # Create the JSON with it
    JSON=`printf '{ "Levels": [{ "rms": %d, "peak": %d, "whatever": %d }] }' $RMS $PEAK $WHATEVER`
    echo "-> $JSON"

    curl --data "$JSON" "http://localhost:8080"
}

# Post infinite new random states
while true; do
    postState
    sleep 0.5
done