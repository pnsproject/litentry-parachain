#!/usr/bin/env bash
set -eo pipefail

function usage() {
  echo "Usage: $0 litentry|litmus"
}

[ $# -ne 1 ] && (usage; exit 1)

function print_divider() {
  echo "------------------------------------------------------------"
}

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR/docker"

PARCHAIN_LAUNCH_BIN="$ROOTDIR/docker/node_modules/.bin/parachain-launch"

CHAIN=$1
CONFIG=$CHAIN-parachain-launch-config.yml
OUTDIR=generated-$CHAIN

print_divider

echo "installing parachain-launch ..."
yarn add @open-web3/parachain-launch
print_divider

# pull the polkadot image to make sure we are using the latest
# litentry-parachain image is left as it is, since it could be freshly built
POLKADOT_IMAGE=$(grep 'parity/polkadot' "$CONFIG" | sed 's/.*image: //')
echo "pulling $POLKADOT_IMAGE ..."
docker pull -q "$POLKADOT_IMAGE"

print_divider

"$PARCHAIN_LAUNCH_BIN" generate --config="$CONFIG" --output="$OUTDIR" --yes

echo "Done, please check files under $ROOTDIR/docker/$OUTDIR/"
print_divider
