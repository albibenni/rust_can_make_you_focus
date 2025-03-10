## Why

## How to use it

- Only works on Linux or Macos OS.

1. Run the cli app with elevated privileges, `sudo cargo run...` `sudo ./...`
2. Pass args to the cli with the website to block:
   - `sudo cargo run youtube x netflix`
   - The app will the block the websites defined (if they find a match in the app, else pr can be open or simply added via `constant` and `matchers` in the code) using the `/etc/hosts` file.
