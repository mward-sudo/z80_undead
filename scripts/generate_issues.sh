#!/bin/bash

# Usage: ./generate_issues.sh <template_directory>
# Example: ./generate_issues.sh templates/z80_implementation
#
# This script generates a set of linked GitHub issues from markdown templates.
# Template directory structure:
#   00_*.md - Tracking issue template
#   [01-99]_*.md - Implementation issue templates
#
# The script will:
# 1. Create all issues from templates
# 2. Link implementation issues to the tracking issue
# 3. Set up dependencies between issues based on the template numbering

REPO="mward-sudo/z80_undead"

# Check for template directory argument
if [ $# -ne 1 ]; then
    echo "Usage: $0 <template_directory>"
    echo "Example: $0 templates/z80_implementation"
    exit 1
fi

TEMPLATE_DIR="$1"

# Verify template directory exists
if [ ! -d "$TEMPLATE_DIR" ]; then
    echo "Error: Template directory '$TEMPLATE_DIR' not found"
    exit 1
fi

# Verify tracking issue template exists
if ! ls "$TEMPLATE_DIR"/00_*.md 1> /dev/null 2>&1; then
    echo "Error: No tracking issue template (00_*.md) found in '$TEMPLATE_DIR'"
    exit 1
fi

# Function to print with proper newlines
echo_n() {
    printf "%b" "$1"
}

# Create required labels if they don't exist
create_labels() {
    echo "Ensuring required labels exist..."
    local labels=("enhancement" "instruction-set" "tracking")
    local colors=("a2eeef" "fbca04" "0052cc")
    
    for i in "${!labels[@]}"; do
        if ! gh label list --repo "$REPO" | grep -q "^${labels[$i]}"; then
            echo "Creating label: ${labels[$i]}"
            gh label create "${labels[$i]}" --repo "$REPO" --color "${colors[$i]}" --force
        fi
    done
}

# Get the title from a template file
get_title() {
    local file="$1"
    local title=""
    local in_frontmatter=false
    
    while IFS= read -r line; do
        if [[ "$line" == "---" ]]; then
            if [ "$in_frontmatter" = false ]; then
                in_frontmatter=true
                continue
            else
                break
            fi
        fi
        
        if [ "$in_frontmatter" = true ] && [[ "$line" =~ ^title:\ \"(.*)\"$ ]]; then
            title="${BASH_REMATCH[1]}"
            break
        fi
    done < "$file"
    
    echo "$title"
}

# Create an issue from a template
create_issue() {
    local template="$1"
    local file="$TEMPLATE_DIR/$template"
    local title=""
    local labels=""
    local body=""
    local in_frontmatter=false
    local in_body=false
    
    # Parse the file
    while IFS= read -r line; do
        # Handle frontmatter
        if [[ "$line" == "---" ]]; then
            if [ "$in_frontmatter" = false ]; then
                in_frontmatter=true
                continue
            else
                in_frontmatter=false
                in_body=true
                continue
            fi
        fi
        
        # Process frontmatter
        if [ "$in_frontmatter" = true ]; then
            if [[ "$line" =~ ^title:\ \"(.*)\"$ ]]; then
                title="${BASH_REMATCH[1]}"
            elif [[ "$line" =~ ^labels:\ (.*)$ ]]; then
                # Split labels and trim spaces
                IFS=',' read -ra label_array <<< "${BASH_REMATCH[1]}"
                for label in "${label_array[@]}"; do
                    label=$(echo "$label" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
                    if [ -n "$labels" ]; then
                        labels="$labels,$label"
                    else
                        labels="$label"
                    fi
                done
            fi
            continue
        fi
        
        # Process body
        if [ "$in_body" = true ]; then
            body+="$line"$'\n'
        fi
    done < "$file"
    
    # Create the issue with proper attributes
    if [ -n "$labels" ]; then
        response=$(gh issue create --repo "$REPO" --title "$title" --body "$body" --label "$labels")
    else
        response=$(gh issue create --repo "$REPO" --title "$title" --body "$body")
    fi
    
    # Extract issue number from response URL
    issue_number=$(echo "$response" | grep -o '[0-9]*$')
    echo "$issue_number"
}

# Update tracking issue with implementation tasks and details
update_tracking_issue() {
    local tracking_number="$1"
    shift
    local implementation_numbers=("$@")
    local titles=()
    
    # Get titles for all implementation issues
    for template in $(ls -1 "$TEMPLATE_DIR"/[0-9][0-9]_*.md | grep -v "00_" | sort); do
        titles+=("$(get_title "$template")")
    done
    
    # Create the tracking issue content
    local tracking_content="# Implementation Tasks

"
    # Add implementation tasks
    for i in "${!implementation_numbers[@]}"; do
        tracking_content+="$((i+1)). #${implementation_numbers[$i]}"$'\n'
    done

    tracking_content+="
See individual issues for implementation details.

---

$(cat "$TEMPLATE_DIR"/00_*.md | awk '
    BEGIN { in_frontmatter=0; content=""; }
    /^---$/ { in_frontmatter = !in_frontmatter; next }
    !in_frontmatter { content = content $0 "\n" }
    END { print content }
' | sed -E "
    # Replace implementation issue placeholders
    $(for i in "${!implementation_numbers[@]}"; do
        echo "s/{{IMPL_$((i+1))}}/#${implementation_numbers[$i]}/g;"
        echo "s/#\[${implementation_numbers[$i]}\]/#${implementation_numbers[$i]}/g;"
    done)
")"

    echo "Updating tracking issue (#$tracking_number)..."
    echo_n "$tracking_content" | gh issue edit "$tracking_number" --body-file - --repo "$REPO"
}

# Update implementation issue with dependencies
update_implementation_issue() {
    local issue_number="$1"
    local template_num="$2"
    shift 2
    local impl_numbers=("$@")
    
    # Get the template file corresponding to this issue
    local template_file
    for f in "$TEMPLATE_DIR"/*_*.md; do
        if [[ "$(basename "$f")" =~ ^([0-9]{2})_ ]]; then
            if [ "${BASH_REMATCH[1]}" = "$template_num" ]; then
                template_file="$f"
                break
            fi
        fi
    done
    
    if [ -f "$template_file" ]; then
        # Get the original content without frontmatter
        local original_content
        original_content=$(awk '
            BEGIN { in_frontmatter=0; content=""; }
            /^---$/ { in_frontmatter = !in_frontmatter; next }
            !in_frontmatter { content = content $0 "\n" }
            END { print content }
        ' "$template_file")
        
        # Build dependencies section if dependencies.txt exists
        local deps_section=""
        local deps_file="$TEMPLATE_DIR/dependencies.txt"
        if [ -f "$deps_file" ]; then
            deps_section="## Dependencies\nDepends on:\n"
            while IFS=':' read -r template_id dependencies; do
                if [ "$template_id" = "$template_num" ]; then
                    IFS=',' read -ra dep_array <<< "$dependencies"
                    for dep in "${dep_array[@]}"; do
                        # Replace implementation issue placeholders
                        dep=$(echo "$dep" | sed -E "
                            $(for i in "${!impl_numbers[@]}"; do
                                echo "s/{{IMPL_$((i+1))}}/#${impl_numbers[$i]}/g;"
                                echo "s/##([0-9]+)/#\\1/g;"
                            done)
                        ")
                        deps_section+="- $dep\n"
                    done
                    break
                fi
            done < "$deps_file"
        fi
        
        # Insert dependencies section after Overview if it exists
        local new_body
        if [ -n "$deps_section" ]; then
            new_body=$(echo "$original_content" | awk -v deps="$deps_section" '
                /^## Overview$/ { print; getline; print; print "\n" deps; next }
                { print }
            ')
        else
            new_body="$original_content"
        fi
        
        echo_n "$new_body" | gh issue edit "$issue_number" --body-file - --repo "$REPO"
    fi
}

# Ensure we're authenticated
if ! gh auth status &>/dev/null; then
    echo "Error: Not authenticated with GitHub CLI"
    echo "Please run 'gh auth login' first"
    exit 1
fi

# Create labels first
create_labels

# Create tracking issue
echo "Creating tracking issue..."
tracking_number=$(create_issue "$(basename "$(ls "$TEMPLATE_DIR"/00_*.md)")")
echo "Created tracking issue #$tracking_number"

# Create implementation issues
implementation_numbers=()
template_nums=()
for template in $(ls -1 "$TEMPLATE_DIR"/[0-9][0-9]_*.md | grep -v "00_" | sort); do
    if [[ "$(basename "$template")" =~ ^([0-9]{2})_ ]]; then
        template_nums+=("${BASH_REMATCH[1]}")
        issue_number=$(create_issue "$(basename "$template")")
        implementation_numbers+=("$issue_number")
        echo "Created implementation issue #$issue_number"
        sleep 1
    fi
done

# Save issue numbers for cleanup
echo "$tracking_number" > .last_generated_issues
printf "%s\n" "${implementation_numbers[@]}" >> .last_generated_issues

# Update tracking issue with implementation tasks
update_tracking_issue "$tracking_number" "${implementation_numbers[@]}"

# Update implementation issues with dependencies
for i in "${!implementation_numbers[@]}"; do
    update_implementation_issue "${implementation_numbers[$i]}" "${template_nums[$i]}" "${implementation_numbers[@]}"
done

echo "All issues created and updated successfully!"
