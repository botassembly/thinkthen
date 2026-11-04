# grep matches characters; filter keeps the lines that answer yes.
grep -i refund notes.txt
echo '--- filter'
thinkthen filter --from lines \
  'Is the note about money going back to the customer?' notes.txt > kept.txt
cat kept.txt
echo '--- count'
wc -l < kept.txt
