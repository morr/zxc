#!/usr/bin/env bash
# Bevy Remote Protocol helper for the running app.
#
#   brp.sh <method> [params-json]
#
# Examples (swap <crate> for this project's crate name):
#   brp.sh rpc.discover
#   brp.sh world.query '{"data":{"components":["<crate>::pawn::components::Pawn"]},"strict":true}'
#   brp.sh world.list_components '{"entity":4294966761}'
#   brp.sh world.get_resources '{"resource":"bevy_time::time::Time<bevy_time::virt::Virtual>"}'
#
# Prints the raw JSON-RPC response. Pipe into python3 to aggregate; always guard
# for the error branch, e.g.
#   | python3 -c "import json,sys; d=json.load(sys.stdin); r=d.get('result'); print(len(r) if r is not None else d['error']['message'])"

set -u

URL="${BRP_URL:-http://127.0.0.1:15702/}"
METHOD="${1:-rpc.discover}"
PARAMS="${2:-}"

if [ -z "$PARAMS" ]; then
    BODY="{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"$METHOD\"}"
else
    BODY="{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"$METHOD\",\"params\":$PARAMS}"
fi

if ! curl -s -m "${BRP_TIMEOUT:-5}" -X POST "$URL" \
    -H 'Content-Type: application/json' -d "$BODY"; then
    echo "brp: no response from $URL — is the app running (cargo run in background)?" >&2
    exit 1
fi
echo
