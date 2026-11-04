# Starts the stand-in server on a free port, then runs the command the page shows.
python3 -B server.py > ready.txt &
server=$!
trap 'kill $server 2>/dev/null; rm -f ready.txt' EXIT
until [ -s ready.txt ]; do sleep 0.1; done
port=$(cat ready.txt) bash show.sh
