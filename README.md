## Why

## How to use it

- Only works on Linux or Macos OS.

1. Run the cli app with elevated privileges, `sudo cargo run...` `sudo ./...`
2. Pass args to the cli with the website to block:
   1. `sudo cargo run youtube x netflix 10`
      - The app will the block the websites defined (if they find a match in the app, else pr can be open or simply added via `constant` and `matchers` in the code) using the `/etc/hosts` file.
      - The end argument **MUST** be the pomodoro timer in minutes
   2. `sudo cargo run studying 10`
      - Using a _preset_ (**all**, **studying**, **coding** are those supported so far) cli _argument_ you can block multiple websites with one arg, the timer is still needed.
        - **ALL** blocks:
          - youtube
          - x
          - netflix
        - **STUDYING** blocks:
          - youtube
          - x
          - netflix
        - **CODING** blocks:
          - youtube
          - x
