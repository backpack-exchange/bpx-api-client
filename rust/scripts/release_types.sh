#!/usr/bin/env bash
set -e

# Constants.
CRATE_NAME="bpx-api-types"
CRATE_DIR="types"

# Function to get current version from Cargo.toml.
get_current_version() {
    grep '^version = ' "$CRATE_DIR/Cargo.toml" | cut -d '"' -f2
}

# Function to get latest published version from crates.io.
get_published_version() {
    cargo search "$CRATE_NAME" | grep "^$CRATE_NAME = " | cut -d '"' -f2
}

# Function to get current commit hash.
get_current_commit_hash() {
    git rev-parse HEAD
}

# Function to check if tag exists.
tag_exists() {
    local version=$1
    git tag -l "v$version" | grep -q "v$version"
}

# Function to increment patch version.
increment_version() {
    local version=$1
    local major minor patch
    IFS='.' read -r major minor patch <<< "$version"
    echo "$major.$minor.$((patch + 1))"
}

# Main script.
echo "🔍 Checking versions for $CRATE_NAME..."

CURRENT_VERSION=$(get_current_version)
PUBLISHED_VERSION=$(get_published_version)
CURRENT_HASH=$(get_current_commit_hash)

echo "Current version: $CURRENT_VERSION"
echo "Published version: $PUBLISHED_VERSION"

# If versions are different or this is a new commit, we need to release.
if [ "$CURRENT_VERSION" != "$PUBLISHED_VERSION" ] || [ -n "$(git diff origin/master)" ]; then
    echo "📦 Changes detected, preparing new release..."

    # If versions are the same but we have changes, increment the version.
    if [ "$CURRENT_VERSION" = "$PUBLISHED_VERSION" ]; then
        NEW_VERSION=$(increment_version "$CURRENT_VERSION")
        echo "Incrementing version to $NEW_VERSION"

        # Update version in Cargo.toml.
        sed -i.bak "s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" "$CRATE_DIR/Cargo.toml"
        rm "$CRATE_DIR/Cargo.toml.bak"

        # Stage and commit version bump.
        git add "$CRATE_DIR/Cargo.toml"
        git commit -m "release: bump $CRATE_NAME version to $NEW_VERSION"

        # Use the new version for tagging.
        CURRENT_VERSION=$NEW_VERSION
    fi

    # Create git tag if it doesn't exist.
    if ! tag_exists "$CURRENT_VERSION"; then
        echo "🏷️  Creating git tag v$CURRENT_VERSION..."
        git tag -a "v$CURRENT_VERSION" -m "Release version $CURRENT_VERSION"
        git push origin "v$CURRENT_VERSION"
    else
        echo "ℹ️  Git tag v$CURRENT_VERSION already exists"
    fi

    # Publish to crates.io.
    echo "🚀 Publishing to crates.io..."
    (cd "$CRATE_DIR" && cargo publish --dry-run)

    echo "✅ Successfully published $CRATE_NAME! -- Just kidding: This is STILL [experimental]: Check and publish manually"
else
    echo "✨ No changes detected, skipping release"
fi
