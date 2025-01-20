param (
    [Parameter(Mandatory=$true)][string]$example,
    [string]$profile = "dev-raytrace"
)

cargo build --profile $profile --package ray-tracing-one-weekend --example $example
Measure-Command { cargo run --profile $profile  --example $example | set-content output/$example-$(Get-Date -UFormat %s -Millisecond 0).png -Encoding Byte }
