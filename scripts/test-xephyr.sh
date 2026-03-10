#!/bin/bash

# Kill existing Xephyr
pkill Xephyr
sleep 1

# Start Xephyr
Xephyr -br -ac -noreset -screen 1280x720 :1 &
sleep 2

# Compile
cargo build

# Run BlinkWM
DISPLAY=:1 ./target/debug/blinkwm &
WM_PID=$!
sleep 2

# Run BlinkWM Bar
DISPLAY=:1 ./target/debug/blinkwm-bar &
BAR_PID=$!

echo "BlinkWM started in Xephyr (:1)"
echo "WM PID: $WM_PID"
echo "Bar PID: $BAR_PID"

# Wait for WM to exit
wait $WM_PID

# Cleanup
kill $BAR_PID
pkill Xephyr
