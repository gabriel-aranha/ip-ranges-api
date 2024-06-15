#!/bin/bash

combinations=(
	"region=us-east-2&service=s3:200"
	"region=us-west-1&service=ec2&network_border_group=us-west-1:200"
	"service=s3:200"
	"region=eu-central-1:200"
	"network_border_group=ap-southeast-1:200"
	":404"
)

for combo in "${combinations[@]}"; do
	IFS=":" read -r params expected <<< "$combo"
	status=$(curl -s -o /dev/null -w "%{http_code}" "http://localhost:8000/v1/aws?$params")
	if [ "$status" -ne "$expected" ]; then
		echo "AWS test failed for params: $params. Expected $expected, got $status"
		exit 1
	else
		echo "AWS test passed for params: $params. Expected and got $status"
	fi
done
