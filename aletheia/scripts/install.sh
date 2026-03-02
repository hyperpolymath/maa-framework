#!/usr/bin/env bash
# Aletheia installation script
# This script installs Aletheia on Unix-like systems

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
REPO_URL="https://gitlab.com/maa-framework/6-the-foundation/aletheia.git"
INSTALL_DIR="${HOME}/.local/bin"
TEMP_DIR=$(mktemp -d)

# Functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_dependencies() {
    log_info "Checking dependencies..."

    if ! command -v git &> /dev/null; then
        log_error "git is not installed. Please install git first."
        exit 1
    fi

    if ! command -v cargo &> /dev/null; then
        log_error "cargo is not installed. Please install Rust first."
        log_info "Visit: https://rustup.rs/"
        exit 1
    fi

    log_info "All dependencies satisfied."
}

clone_repository() {
    log_info "Cloning Aletheia repository..."
    cd "$TEMP_DIR"
    git clone "$REPO_URL" aletheia
    cd aletheia
}

build_binary() {
    log_info "Building Aletheia (release mode)..."
    cargo build --release

    if [ ! -f "target/release/aletheia" ]; then
        log_error "Build failed - binary not found"
        exit 1
    fi

    log_info "Build successful."
}

install_binary() {
    log_info "Installing binary to ${INSTALL_DIR}..."

    # Create install directory if it doesn't exist
    mkdir -p "$INSTALL_DIR"

    # Copy binary
    cp target/release/aletheia "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/aletheia"

    log_info "Binary installed to ${INSTALL_DIR}/aletheia"
}

check_path() {
    if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
        log_warn "${INSTALL_DIR} is not in your PATH."
        log_warn "Add this to your shell profile:"
        echo ""
        echo "    export PATH=\"\$PATH:${INSTALL_DIR}\""
        echo ""
    else
        log_info "${INSTALL_DIR} is in your PATH."
    fi
}

cleanup() {
    log_info "Cleaning up temporary files..."
    rm -rf "$TEMP_DIR"
}

verify_installation() {
    log_info "Verifying installation..."

    if command -v aletheia &> /dev/null; then
        log_info "Installation verified successfully!"
        log_info "Run 'aletheia' to verify a repository."
    else
        log_warn "Binary installed but not in PATH."
        log_warn "You can run it directly: ${INSTALL_DIR}/aletheia"
    fi
}

main() {
    echo "Aletheia Installation Script"
    echo "============================"
    echo ""

    check_dependencies
    clone_repository
    build_binary
    install_binary
    cleanup
    check_path
    verify_installation

    echo ""
    log_info "Installation complete!"
    echo ""
    echo "Next steps:"
    echo "  1. Run 'aletheia' in any repository to verify RSR compliance"
    echo "  2. Run 'aletheia /path/to/repo' to verify a specific repository"
    echo "  3. Read the documentation at:"
    echo "     https://gitlab.com/maa-framework/6-the-foundation/aletheia"
    echo ""
}

# Run main function
main
