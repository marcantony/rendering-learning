param (
    [Parameter(Mandatory=$true)][string]$example
)

cargo run --release --package ray-tracing-one-weekend --example $example | set-content output/$example-$(Get-Date -UFormat %s -Millisecond 0).ppm -encoding String
