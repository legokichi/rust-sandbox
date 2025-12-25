#!/bin/sh
set -e

RULE_URL="https://probe.rs/files/69-probe-rs.rules"
RULE_DST="/etc/udev/rules.d/69-probe-rs.rules"

# fetch rule file
if command -v curl >/dev/null 2>&1; then
  curl -sSf "$RULE_URL" -o "$RULE_DST"
elif command -v wget >/dev/null 2>&1; then
  wget -qO "$RULE_DST" "$RULE_URL"
else
  echo "curl or wget is required" >&2
  exit 1
fi

chmod 0644 "$RULE_DST"

# reload udev
udevadm control --reload-rules
udevadm trigger

echo "Installed $RULE_DST. Replug the debug probe."
