# Run the unchanged example against the staged R package.
set -eu
root=$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../../../.." && pwd)
packages=${SITE_PACKAGES:-$root/target/site-packages}
R_LIBS="$packages/r" Rscript show.R
