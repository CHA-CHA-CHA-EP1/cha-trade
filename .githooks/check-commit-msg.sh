#!/usr/bin/env bash
# Rule 3: Commit message must be "<TICKET-ID>: <description>"
# Allowed: DGL-1234: fix payment retry logic
# Not allowed: fix bug, update code

COMMIT_MSG_FILE="${1:-$GIT_COMMIT_EDITMSG}"

# Fallback for pre-commit framework (passes .git/COMMIT_EDITMSG)
if [ -z "$COMMIT_MSG_FILE" ]; then
  COMMIT_MSG_FILE=".git/COMMIT_EDITMSG"
fi

MSG=$(cat "$COMMIT_MSG_FILE" 2>/dev/null | head -1)

# Skip merge commits
if echo "$MSG" | grep -qE "^Merge "; then
  exit 0
fi

PATTERN="^[A-Z]+-[0-9]+: .{5,}$"

if ! echo "$MSG" | grep -qE "$PATTERN"; then
  echo ""
  echo "❌ Commit message format is invalid."
  echo ""
  echo "   Your message: '$MSG'"
  echo ""
  echo "   Required format: <TICKET-ID>: <description>"
  echo "   Example:         DGL-1234: fix payment retry logic"
  echo "                    DGL-5678: add user authentication endpoint"
  echo ""
  echo "   Rules:"
  echo "   - Must start with ticket ID (e.g. DGL-1234)"
  echo "   - Followed by ': ' (colon + space)"
  echo "   - Description must be at least 5 characters"
  echo ""
  exit 0
fi

echo "✅ Commit message is valid."
exit 0
