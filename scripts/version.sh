#!/bin/bash

# Version management script for NullScript
# Usage: ./scripts/version.sh [new_version]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to get current version from Cargo.toml
get_current_version() {
    grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/'
}

# Function to update version in Cargo.toml
update_cargo_version() {
    local new_version=$1
    sed -i.bak "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
    rm Cargo.toml.bak
    print_status "Updated Cargo.toml version to $new_version"
}

# Function to update version in package.json
update_package_version() {
    local new_version=$1
    sed -i.bak "s/\"version\": \".*\"/\"version\": \"$new_version\"/" package.json
    rm package.json.bak
    print_status "Updated package.json version to $new_version"
}

# Function to update version in README.md if it exists
update_readme_version() {
    local new_version=$1
    if [ -f README.md ]; then
        # Update version in README.md (look for common patterns)
        sed -i.bak "s/version.*2\.0\.[0-9]*/version $new_version/g" README.md
        sed -i.bak "s/v2\.0\.[0-9]*/v$new_version/g" README.md
        rm README.md.bak 2>/dev/null || true
        print_status "Updated README.md version references to $new_version"
    fi
}

# Function to validate version format
validate_version() {
    local version=$1
    if [[ ! $version =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        print_error "Invalid version format. Use semantic versioning (e.g., 2.0.3)"
        exit 1
    fi
}

# Function to show current version
show_current_version() {
    local current_version=$(get_current_version)
    echo -e "${BLUE}Current version:${NC} $current_version"
    echo -e "${BLUE}Version is used in:${NC}"
    echo "  - Cargo.toml (source of truth)"
    echo "  - package.json (npm package)"
    echo "  - All Rust files via env!(\"CARGO_PKG_VERSION\")"
    echo ""
    echo -e "${BLUE}Files that reference version:${NC}"
    grep -r "CARGO_PKG_VERSION" src/ || true
}

# Main script logic
main() {
    local current_version=$(get_current_version)
    
    if [ $# -eq 0 ]; then
        # No arguments provided, show current version
        show_current_version
        exit 0
    fi
    
    local new_version=$1
    
    print_status "Current version: $current_version"
    print_status "New version: $new_version"
    
    if [ "$current_version" = "$new_version" ]; then
        print_warning "Version is already $new_version"
        exit 0
    fi
    
    # Validate version format
    validate_version "$new_version"
    
    # Update versions
    update_cargo_version "$new_version"
    update_package_version "$new_version"
    update_readme_version "$new_version"
    
    print_status "Version updated successfully to $new_version"
    print_status "Run 'cargo build' to rebuild with new version"
    
    # Show what changed
    echo ""
    echo -e "${BLUE}Changes made:${NC}"
    git diff --stat || true
}

# Run main function with all arguments
main "$@"
