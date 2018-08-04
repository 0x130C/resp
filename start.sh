#!/usr/bin/env bash

function run() {
    cd client
    if [[ ! -d "./node_modules" ]]; then
        yarn
    fi
    cargo watch -w src -s 'cp ./target/wasm32-unknown-unknown/release/app.* ../static && notify-send "Client build completed"' -x '+nightly web build --release' &
    P1=$!
    yarn watch-sass &
    P2=$!
    trap "kill $P1 $P2" SIGINT

    cd ..
    DEV_MODE=true RUST_BACKTRACE=1 cargo watch -w src -w templates -x run &
    P3=$!

    echo "Setup dev development completed..."
    wait $P1 $P2 $P3

    echo "Stopped..."
}

case "$1" in
    run)
        run
    ;;
    *)
        run
    ;;
esac