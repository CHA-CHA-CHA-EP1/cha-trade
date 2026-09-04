#!/usr/bin/env bash
# Rule 2: No direct commit to release/* branches
# Must merge via PR only

BRANCH=$(git symbolic-ref --short HEAD 2>/dev/null)

if echo "$BRANCH" | grep -qE "^release/"; then
  echo ""
  echo "❌ Direct commits to '$BRANCH' are not allowed."
  echo ""
  echo "   Release branches are protected."
  echo "   Please merge your changes via Pull Request only."
  echo ""
  exit 1
fi

exit 0
