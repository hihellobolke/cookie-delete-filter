# Cookie Delete Filter - envoy/istio WASM filter

This can selected cookies from the http request before it reaches the application. You can use the configuration example in `envoy.yaml` to remove cookies from the request.

## Build

- Install [func-e](https://func-e.io/) - helps running envoy
- Example
    ```BASH

    cargo build --target wasm32-unknown-unknown --release;
    [[ -e target/wasm32-unknown-unknown/release/cookie_delete_filter.wasm ]] && func-e run --config-path envoy.yaml

    # test, by sending cookies, and checking the response headers from httpbin.org/headers
    while true; do
        curl -q -s \
          http://127.0.0.1:8080/headers \
          --header 'cookie: a=1; b=2; remove-this-cookie=no; remove-this-cookie-too=no; c=3' | jq '.headers.Cookie'
    done
    ```
