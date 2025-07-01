set shell := ["bash", "-uc"]

default:
  @just --choose --justfile {{justfile()}}

web:
  #!/usr/bin/env bash
  set -euo pipefail
  dx serve --platform web -p app

clear:
  #!/usr/bin/env bash
  set -euo pipefail
  cargo clean
  rm *.lock

install-dx:
  #!/usr/bin/env bash
  set -euo pipefail
  cargo install --git https://github.com/DioxusLabs/dioxus.git dioxus-cli --force --features no-downloads

bundle:
  #!/usr/bin/env bash
  dx bundle -p app --release --trace --verbose

run-bundled:
  #!/usr/bin/env bash
  set -euo pipefail
  PORT=8080 IP=0.0.0.0 ./dist/app