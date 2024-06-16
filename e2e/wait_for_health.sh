#!/bin/bash

# Define the health check URL
HEALTH_URL="http://localhost:8000/health"
# Maximum number of attempts
MAX_ATTEMPTS=30
# Delay between attempts in seconds
SLEEP_DURATION=2
# Counter for attempts
ATTEMPT=0

# Loop until the health check returns "ok" or the max attempts are reached
until [ $ATTEMPT -ge $MAX_ATTEMPTS ]; do
  RESPONSE=$(curl -s $HEALTH_URL)
  STATUS=$(echo $RESPONSE | grep -o '"status":"[^"]*"' | grep -o '[^"]*$')
  if [ "$STATUS" = "ok" ]; then
    echo "Health check passed!"
    exit 0
  fi
  echo "Health check failed. Attempt $((ATTEMPT+1))/$MAX_ATTEMPTS. Retrying in $SLEEP_DURATION seconds..."
  ATTEMPT=$((ATTEMPT + 1))
  sleep $SLEEP_DURATION
done

echo "Health check did not pass within the expected time."
exit 1
