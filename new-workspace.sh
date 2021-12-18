#!/bin/bash

set -eu

set -x
# trap read debug

SPACE="$1"

echo creating $SPACE
git worktree add --d ../"$SPACE"
cd ../"$SPACE"

git checkout --orphan "$SPACE"

echo "# workspace

## $SPACE" > README.md
git add README.md
git commit -a -m "initial"

git push --set-upstream origin "$SPACE"

echo
echo https://github.com/cablehead/workspace/tree/"$SPACE"
