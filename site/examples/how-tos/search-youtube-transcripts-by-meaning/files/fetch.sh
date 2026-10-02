id=YbzrlpAyCV4
yt-dlp \
  --skip-download \
  --write-auto-subs \
  --sub-langs en \
  --sub-format vtt \
  -o talk \
  "https://youtu.be/$id"
awk -f vtt-to-lines.awk talk.en.vtt > transcript.txt
