#!/bin/bash

REPO="mward-sudo/z80_undead"
ISSUE_FILE=".last_generated_issues"

if [ ! -f "$ISSUE_FILE" ]; then
    echo "No previously generated issues found."
    echo "Run generate_issues.sh first to create issues."
    exit 1
fi

echo "Deleting previously generated issues..."
while IFS= read -r issue; do
    echo "Deleting issue #$issue..."
    gh issue delete "$issue" --repo "$REPO" --yes
done < "$ISSUE_FILE"

rm "$ISSUE_FILE"
echo "Issues deleted successfully!"
