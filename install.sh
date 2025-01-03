#!/bin/bash

RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
CYAN="\033[0;36m"
RESET="\033[0m"

URL="https://github.com/dybdeskarphet/mcfetch/releases/download/v0.1.0/mcfetch"
BINARY_NAME="mcfetch"
DEST_DIR="/usr/bin"
TEMP_DIR="/tmp"

success() {
    echo -e "${GREEN}[SUCCESS]${RESET} $1"
}

error() {
    echo -e "${RED}[ERROR]${RESET} $1"
    exit 1
}

info() {
    echo -e "${CYAN}[INFO]${RESET} $1"
}

uninstall() {
    info "Uninstalling $BINARY_NAME..."
    if [[ -f "$DEST_DIR/$BINARY_NAME" ]]; then
        if sudo rm "$DEST_DIR/$BINARY_NAME"; then
            success "$BINARY_NAME has been successfully uninstalled from $DEST_DIR."
        else
            error "Failed to remove $BINARY_NAME. Please check your permissions."
        fi
    else
        info "$BINARY_NAME is not installed in $DEST_DIR. Nothing to remove."
    fi
    exit 0
}

install() {
    info "Starting the installation process for $BINARY_NAME..."

    if [[ -f "$DEST_DIR/$BINARY_NAME" ]]; then
        info "$BINARY_NAME already exists in $DEST_DIR. Removing it..."
        if sudo rm "$DEST_DIR/$BINARY_NAME"; then
            success "Old version of $BINARY_NAME removed successfully."
        else
            error "Failed to remove existing $BINARY_NAME. Please check your permissions."
        fi
    else
        info "$BINARY_NAME does not exist in $DEST_DIR. Proceeding with installation..."
    fi

    TEMP_DIR=$(mktemp -d) || error "Failed to create a temporary directory."

    info "Downloading $BINARY_NAME to $TEMP_DIR..."
    if curl -L "$URL" -o "$TEMP_DIR/$BINARY_NAME"; then
        success "Download completed!"
    else
        error "Failed to download $BINARY_NAME. Please check the URL or your network connection."
    fi

    info "Setting execute permission for $BINARY_NAME..."
    if chmod +x "$TEMP_DIR/$BINARY_NAME"; then
        success "Execute permission set!"
    else
        error "Failed to set execute permission for $BINARY_NAME."
    fi

    info "Moving $BINARY_NAME to $DEST_DIR..."
    if sudo mv "$TEMP_DIR/$BINARY_NAME" "$DEST_DIR"; then
        success "$BINARY_NAME has been successfully installed to $DEST_DIR!"
    else
        error "Failed to move $BINARY_NAME. Please check your permissions."
    fi

    info "Verifying the installation..."
    if command -v "$BINARY_NAME" > /dev/null; then
        success "$BINARY_NAME is ready to use! Run the command: ${YELLOW}$BINARY_NAME${RESET}"
    else
        error "Installation verification failed. $BINARY_NAME is not in PATH."
    fi

    rm -rf "$TEMP_DIR"
    info "Installation process completed!"
}

case "$1" in
    --uninstall)
        uninstall
        ;;
    *)
        install
        ;;
esac

