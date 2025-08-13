# Version Management System

This directory contains scripts for managing the NullScript version across all files.

## Centralized Version Management

NullScript uses a **centralized version management system** where you only need to change the version in one place, and it automatically updates everywhere.

### How It Works

1. **Source of Truth**: `Cargo.toml` is the single source of truth for version
2. **Automatic Propagation**: All Rust files use `env!("CARGO_PKG_VERSION")` to get the version at compile time
3. **Cross-Platform Sync**: The version script automatically syncs versions across different file formats

### Files That Get Updated

- ✅ `Cargo.toml` (Rust package version)
- ✅ `package.json` (NPM package version)
- ✅ `README.md` (Documentation version references)

### Files That Automatically Use the Version

- ✅ All Rust source files via `env!("CARGO_PKG_VERSION")`
- ✅ CLI output and help messages
- ✅ System information display

## Usage

### NPM Scripts (Recommended)

For convenience, you can use npm scripts to manage versions:

```bash
# Check current version
npm run version:check

# Update to new version
npm run version:update 2.0.4
```

### Direct Script Usage

You can also use the scripts directly:
```bash
# Using npm script
npm run version:check

# Using direct script
./scripts/version.sh
```

### Update to New Version
```bash
# Using npm script
npm run version:update 2.0.4

# Using direct script
./scripts/version.sh 2.0.4
```

### Examples

```bash
# Check current version and where it's used
npm run version:check
./scripts/version.sh

# Update to patch version
npm run version:update 2.0.4
./scripts/version.sh 2.0.4

# Update to minor version
npm run version:update 2.1.0
./scripts/version.sh 2.1.0

# Update to major version
npm run version:update 3.0.0
./scripts/version.sh 3.0.0
```

## Benefits

1. **Single Point of Change**: Update version in one place
2. **Automatic Sync**: All files stay in sync automatically
3. **Validation**: Script validates version format
4. **Safety**: Shows what changes will be made
5. **Cross-Platform**: Works with both Rust and NPM ecosystems

## Version Format

The script enforces semantic versioning format: `MAJOR.MINOR.PATCH`

- **MAJOR**: Breaking changes
- **MINOR**: New features, backward compatible
- **PATCH**: Bug fixes, backward compatible

Example: `2.0.3`

## After Version Update

After running the version script:

1. **Build the project**: `cargo build`
2. **Test the version**: `./target/debug/nsc -v`
3. **Commit changes**: `git add . && git commit -m "chore: bump version to 2.0.3"`
4. **Push to repository**: `git push origin dev`

## Troubleshooting

### Version Not Updating in Binary
- Run `cargo clean && cargo build` to ensure clean rebuild
- Check that `env!("CARGO_PKG_VERSION")` is used in code

### Script Permission Denied
```bash
chmod +x scripts/version.sh
```

### Invalid Version Format
- Use semantic versioning: `X.Y.Z`
- Example: `2.0.3`, `1.5.0`, `3.2.1`
