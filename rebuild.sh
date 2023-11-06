docker stop estuary || true
estuary
cd ~/IdeaProjects/rust-spring-knockoff
./publish_dirty.sh || true
cd ~/IdeaProjects/chrome_streamer_rust
rm ./Cargo.lock || true && knockoff_cli --registry-uri=http://localhost:1234/git/index --mode=dev && cd chrome_streamer_rdev && cargo build || true && cd ..

