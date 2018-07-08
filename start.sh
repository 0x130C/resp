#!/usr/bin/env bash
PID=-1

function ctrl_c() {
#    echo "11111"
#    echo "$(ps -ho pid,ppid )"
#    echo "$PID"
#    kill -KILL "$(ps -ho pid,ppid | grep ^$PID | cut -d' ' -f2)"
    echo "Killed the client."
}

function run() {
    trap ctrl_c INT
    cd client
    cargo watch -w src -s 'cp ./target/wasm32-unknown-unknown/release/app.* ../static && notify-send "Client build completed"' -x '+nightly web build --release' &
    PID=$!
#    cd ../build_tools/
#    cargo run &
    yarn watch-sass &
    cd ..
    DEV_MODE=true systemfd --no-pid -s http::8080 -- cargo watch -x run
}

case "$1" in
    run)
        run
    ;;
    *)
        run
    ;;
esac