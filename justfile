set shell := ["bash", "-uc"]

default:
  @just --choose --justfile {{justfile()}}

web:
  #!/usr/bin/env bash
  set -euo pipefail
  dx serve

clear:
  #!/usr/bin/env bash
  set -euo pipefail
  cargo clean
  rm *.lock

install-dx:
  #!/usr/bin/env bash
  set -euo pipefail
  cargo install dioxus-cli@0.7.0-alpha.2 --force

bundle:
  #!/usr/bin/env bash
  dx bundle --release --trace --verbose

run-bundled:
  #!/usr/bin/env bash
  set -euo pipefail
  PORT=8080 IP=0.0.0.0 ./dist/developer-portfolio