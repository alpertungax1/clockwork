#!/bin/bash

# TODO Borrow the increment-cargo-version.sh script from Solana

# Get new version
current_version=$(cat ./VERSION)
echo "Current version: $current_version"
read -r -p "    New version: " new_version

# Build
RUSTFLAGS="--deny warnings" cargo build || (echo "Build failed" && exit)

# Bump clockwork-cron
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' cron/Cargo.toml

# Bump programs
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' programs/network/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' programs/queue/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' programs/pool/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' programs/webhook/Cargo.toml

# Bump inter-program dependencies
sed -i '' -e 's/^clockwork-cron =.*/clockwork-cron = { path = "..\/..\/cron", version = "'${new_version}'" }/g' programs/queue/Cargo.toml
sed -i '' -e 's/^clockwork-pool-proram =.*/clockwork-pool-proram = { path = "..\/pool", features = ["cpi"], version = "'${new_version}'" }/g' programs/network/Cargo.toml
sed -i '' -e 's/^clockwork-pool-program =.*/clockwork-pool-program = { path = "..\/pool", features = ["cpi"], version = "'${new_version}'" }/g' programs/queue/Cargo.toml
sed -i '' -e 's/^clockwork-pool-program =.*/clockwork-pool-program = { path = "..\/pool", features = ["cpi"], version = "'${new_version}'" }/g' programs/network/Cargo.toml
sed -i '' -e 's/^clockwork-queue-program =.*/clockwork-queue-program = { path = "..\/queue", features = ["cpi"], version = "'${new_version}'" }/g' programs/network/Cargo.toml
sed -i '' -e 's/^clockwork-pool-program =.*/clockwork-pool-program = { path = "..\/pool", features = ["cpi"], version = "'${new_version}'" }/g' programs/webhook/Cargo.toml

# Bump clockwork-client
sed -i '' -e 's/^clockwork-network-program =.*/clockwork-network-program = { path = "..\/programs\/network", features = ["no-entrypoint"], version = "'${new_version}'" }/g' client/Cargo.toml
sed -i '' -e 's/^clockwork-pool-program =.*/clockwork-pool-program = { path = "..\/programs\/pool", features = ["no-entrypoint"], version = "'${new_version}'" }/g' client/Cargo.toml
sed -i '' -e 's/^clockwork-queue-program =.*/clockwork-queue-program = { path = "..\/programs\/queue", features = ["no-entrypoint"], version = "'${new_version}'" }/g' client/Cargo.toml
sed -i '' -e 's/^clockwork-webhook-program =.*/clockwork-webhook-program = { path = "..\/programs\/webhook", features = ["no-entrypoint"], version = "'${new_version}'" }/g' client/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' client/Cargo.toml

# Bump clockwork-bench
sed -i '' -e 's/^clockwork-client =.*/clockwork-client = { path = "..\/client", version = "'${new_version}'" }/g' bench/Cargo.toml
sed -i '' -e 's/^clockwork-cron =.*/clockwork-cron = { path = "..\/cron", version = "'${new_version}'" }/g' bench/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' bench/Cargo.toml

# Bump clockwork-cli
sed -i '' -e 's/^clockwork-client =.*/clockwork-client = { path = "..\/client", version = "'${new_version}'" }/g' cli/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' cli/Cargo.toml

# Bump clockwork-plugin
sed -i '' -e 's/^clockwork-client =.*/clockwork-client = { path = "..\/client", version = "'${new_version}'" }/g' plugin/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' plugin/Cargo.toml

# Bump clockwork-sdk
sed -i '' -e 's/^clockwork-client =.*/clockwork-client = { optional = true, path = "..\/client", version = "'${new_version}'" }/g' sdk/Cargo.toml
sed -i '' -e 's/^clockwork-queue-program =.*/clockwork-queue-program = { path = "..\/programs\/queue", features = ["cpi"], version = "'${new_version}'" }/g' sdk/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' sdk/Cargo.toml

# Update version
echo $new_version > VERSION

# Wait for Cargo.toml update
sleep 25

# Git commit 
echo "$(git diff --stat | tail -n1)"
git add .
git commit -m "Bump from $current_version to $new_version"
git tag "v$new_version"
git push && git push --tags
