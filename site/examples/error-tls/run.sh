# A listener on this machine stands in for the HTTPS service, so no request leaves the machine.
port_file=$(mktemp)
python3 -c '
import socket, sys
s = socket.socket(); s.bind(("127.0.0.1", 0)); s.listen()
open(sys.argv[1], "w").write(str(s.getsockname()[1]))
c, _ = s.accept(); c.close()
' "$port_file" &
trap 'kill $! 2>/dev/null; rm -f "$port_file"' EXIT
while [ ! -s "$port_file" ]; do sleep 0.1; done
SERVICE_URL="https://127.0.0.1:$(cat "$port_file")" bash show.sh
