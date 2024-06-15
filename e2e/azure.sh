#!/bin/bash

combinations=(
	"region=westus2&ipv4=true:200"
	"region=eastus&system_service=azurestorage&ipv4=true&ipv6=true:200"
	"system_service=azurestorage:404"
	"ipv4=true:200"
	"ipv6=true:200"
	":404"
)

for combo in "${combinations[@]}"; do
	IFS=":" read -r params expected <<< "$combo"
	status=$(curl -s -o /dev/null -w "%{http_code}" "http://localhost:8000/v1/azure?$params")
	if [ "$status" -ne "$expected" ]; then
		echo "Azure test failed for params: $params. Expected $expected, got $status"
		exit 1
	else
		echo "Azure test passed for params: $params. Expected and got $status"
	fi
done
