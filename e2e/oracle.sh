#!/bin/bash

combinations=(
    "region=us-ashburn-1&tag=OCI:200"
    "region=us-ashburn-1:200"
    "tag=OCI:200"   
    ":200"
)

for combo in "${combinations[@]}"; do
	IFS=":" read -r params expected <<< "$combo"
	status=$(curl -s -o /dev/null -w "%{http_code}" "http://localhost:8000/v1/oracle?$params")
	if [ "$status" -ne "$expected" ]; then
		echo "Oracle test failed for params: $params. Expected $expected, got $status"
		exit 1
	else
		echo "Oracle test passed for params: $params. Expected and got $status"
	fi
done
