question="Does this sentence hedge?"

cat <<'EOF' > notes.txt
The export runs every night at two.
Failed rows go to a separate file.
Each row keeps its id.
EOF

thinkthen filter "$question" \
  < notes.txt > hedges.txt

if [ -s hedges.txt ]; then
  echo "These lines hedge:"
  cat hedges.txt
  exit 1
fi
