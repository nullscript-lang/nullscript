#!/bin/bash

# NPM version management wrapper script
# Usage: npm run version:update <new_version>

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_usage() {
    echo -e "${BLUE}Usage:${NC}"
    echo "  npm run version:check                    # Check current version"
    echo "  npm run version:update <new_version>     # Update to new version"
    echo ""
    echo -e "${BLUE}Examples:${NC}"
    echo "  npm run version:check"
    echo "  npm run version:update 2.0.4"
    echo "  npm run version:update 2.1.0"
    echo "  npm run version:update 3.0.0"
    echo ""
    echo -e "${BLUE}Or use the direct script:${NC}"
    echo "  ./scripts/version.sh"
    echo "  ./scripts/version.sh 2.0.4"
}

# Check if version argument is provided
if [ $# -eq 0 ]; then
    print_error "No version provided"
    echo ""
    print_usage
    exit 1
fi

# Get the new version from npm arguments
# npm passes arguments after --, so we need to handle that
new_version=""
for arg in "$@"; do
    if [[ $arg != "--" ]]; then
        new_version=$arg
        break
    fi
done

if [ -z "$new_version" ]; then
    print_error "No version provided"
    echo ""
    print_usage
    exit 1
fi

# Call the main version script
print_status "Updating version to $new_version..."
./scripts/version.sh "$new_version"
