for note in list fetch wipe; do
  jq -Rs '{tool_input:{command:.}}' "proposed/$note.txt" |
    bash guard.txt |
    jq -r '.hookSpecificOutput.permissionDecision'
done
