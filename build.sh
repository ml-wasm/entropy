#############################################################################
# #! /bin/bash															    #
# RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' \    #
# 	rustup run nightly \												    #
# 	wasm-pack build --target web --scope ml.wasm \						    #
# 	-- . -Z build-std=panic_abort,std									    #
#############################################################################

cd `dirname "${BASH_SOURCE[0]}"`

RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals -Z macro-backtrace' \
        rustup run nightly \
        wasm-pack build --target web --scope ml.wasm --dev . \
        -- -Z build-std=panic_abort,std

sed -i '/"files":/a \    "snippets",' pkg/package.json
