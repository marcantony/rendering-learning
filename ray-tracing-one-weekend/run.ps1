param (
    [Parameter(Mandatory=$true)][string]$example,
    [string]$profile = "dev-raytrace"
)

cargo run --profile $profile --package ray-tracing-one-weekend --example $example | set-content output/$example-$(Get-Date -UFormat %s -Millisecond 0).ppm -encoding String
