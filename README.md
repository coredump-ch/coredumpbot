# coredumpbot
[![Build Status](https://travis-ci.org/coredump-ch/coredumpbot.svg?branch=master)](https://travis-ci.org/coredump-ch/coredumpbot)

Start the Bot like this:

    TELEGRAM_BOT_TOKEN=XXXXXXXXXXXXXXXXXXXXXXX cargo run --jobs 8

# Install on Arch Linux

    sudo pacman -S git gcc cargo
    git clone https://github.com/coredump-ch/coredumpbot.git
    cd coredumpbot
    TELEGRAM_BOT_TOKEN=XXXXXXXXXXXXXXXXXXXXXXX cargo run --jobs 8 --release

# Update local Client

    cd coredumpbot
    rm -rf target
    git pull
    TELEGRAM_BOT_TOKEN=XXXXXXXXXXXXXXXXXXXXXXX cargo run --jobs 8 --release
