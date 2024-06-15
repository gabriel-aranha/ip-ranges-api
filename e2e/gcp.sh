#!/bin/bash

combinations=(
	"scope=africa-south1&ipv4=true:200"
	"scope=europe-west1&service=google%20cloud&ipv4=true:200"
	"service=google%20cloud:404"
	"ipv4=true:200"
	"ipv6=true:200"
	":404"
)

for combo in "${combinations[@]}"; do
	IFS=":" read -r params expected <<< "$combo"
	status=$(curl -s -o /dev/null -w "%{http_code}" "http://localhost:8000/v1/gcp?$params")
	if [ "$status" -ne "$expected" ]; then
		echo "GCP test failed for params: $params. Expected $expected, got $status"
		exit 1
	else
		echo "GCP test passed for params: $params. Expected and got $status"
	fi
done
