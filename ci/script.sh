set -euxo pipefail

main() {
    cargo check --target $TARGET --no-default-features

    if [ $TARGET != x86_64-unknown-linux-gnu ]; then
        cargo check --target $TARGET
    fi
}

main
