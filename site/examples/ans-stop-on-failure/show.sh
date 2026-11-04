if thinkthen decide --from jsonl --field /text --json \
    'Does the customer request a refund?' broken.jsonl \
    > outcomes.jsonl; then
  echo "all records answered"
else
  status=$?
  echo "stopped: thinkthen exited $status"
fi
jq '{position, status, error: .error.code}' outcomes.jsonl
