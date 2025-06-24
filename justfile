set shell := ["bash", "-uc"]

default:
  @just --choose --justfile {{justfile()}}

web:
  #!/usr/bin/env bash
  set -euo pipefail
  dx serve --platform web -p app

build:
  #!/usr/bin/env bash
  set -euo pipefail
  podman-compose build app

clear:
  #!/usr/bin/env bash
  set -euo pipefail
  cargo clean
  rm *.lock

