#!/bin/bash

ENV_FILE=".env"

set -a
source $ENV_FILE
set +a

curl https://api.openai.com/v1/responses \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -d '{
    "model": "gpt-5-mini",
    "input": "Tell me a three sentence bedtime story about a unicorn."
  }'
