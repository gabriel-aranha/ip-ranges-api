#!/bin/bash

combinations=(
    "ipv4=true:200"
    "ipv6=true:200"
    "ipv4=true&ipv6=true:200"
    ":400"
)

for combo in "${combinations[@]}"; do
    IFS=":" read -r params expected <<< "$combo"

    status=$(curl -s -o /dev/null -w "%{http_code}" "http://localhost:8000/v1/fastly?$params")

    if [ "$status" -ne "$expected" ]; then
        echo "Fastly test failed for params: $params. Expected $expected, got $status"
        exit 1
    else
        echo "Fastly test passed for params: $params. Expected and got $status"
    fi
done
