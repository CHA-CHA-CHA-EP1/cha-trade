#!/usr/bin/env bash
# Rule 1: Branch naming must be <type>/YYYY.MM/<ticket-id>
# Allowed: feature/2026.08/DGL-1234, hotfix/2026.08/DGL-1235
# Not allowed: fix-quick, feature/DGL-1234

BRANCH=$(git symbolic-ref --short HEAD 2>/dev/null)

# Allow special branches
EXEMPT_BRANCHES="^(main|master|develop|HEAD)$"
if echo "$BRANCH" | grep -qE "$EXEMPT_BRANCHES"; then
  exit 0
fi

PATTERN="^(features|feature|hotfix|bugfix|release|chore|refactor)/[0-9]{4}\.[0-9]{2}/[A-Za-z0-9]([A-Za-z0-9._-]*[A-Za-z0-9])?$"

if ! echo "$BRANCH" | grep -qE "$PATTERN"; then
  echo ""
  echo "❌ Branch name '$BRANCH' does not follow naming convention."
  echo ""
  echo "   Required format: <type>/YYYY.MM/<TICKET-ID>"
  echo "   Example:         feature/2026.08/DGL-1234"
  echo "                    hotfix/2026.09/DGL-5678"
  echo ""
  echo "   Allowed types: feature, hotfix, bugfix, release, chore, refactor"
  echo ""
  exit 1
fi

echo "✅ Branch name '$BRANCH' is valid."
exit 0
