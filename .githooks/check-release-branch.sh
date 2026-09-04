#!/usr/bin/env bash
# Rule 2: No direct commit to release/* branches
# Must merge via PR only

BRANCH=$(git symbolic-ref --short HEAD 2>/dev/null)

PROTECTED_BRANCHES="^(main|master|develop|release/.*)$"

if echo "$BRANCH" | grep -qE "$PROTECTED_BRANCHES"; then
  echo ""
  echo "❌ Direct commits to '$BRANCH' are not allowed."
  echo ""
  echo "   Protected branches: main, master, develop, release/*"
  echo "   Please commit on a feature/hotfix/bugfix branch and merge via Pull Request."
  echo ""
  exit 1
fi

exit 0
