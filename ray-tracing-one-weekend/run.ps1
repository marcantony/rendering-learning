cargo run --release --package ray-tracing-one-weekend | set-content output/$(Get-Date -UFormat %s -Millisecond 0).ppm -encoding String
