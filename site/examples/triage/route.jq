if .answer.refund.status == "unsure" then "\(.id) person: refund not sure"
elif .answer.refund.answer then "\(.id) refunds"
else "\(.id) \(.answer.topic.answer)"
end
