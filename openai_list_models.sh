#!/bin/bash

ENV_FILE=".env"

set -a
source $ENV_FILE
set +a

curl https://api.openai.com/v1/models \
  -H "Authorization: Bearer $OPENAI_API_KEY"
