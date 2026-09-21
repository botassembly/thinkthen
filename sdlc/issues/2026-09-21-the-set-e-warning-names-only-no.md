# The set -e warning names only no

Status: Open

The `decide` help says "Under `set -e` or `set -o pipefail` a no ends the script." An unresolved answer ends the script the same way, and the sentence does not say so.

## Reproduction

    $ sh -c 'set -e; echo before; \
        thinkthen decide "The customer asks for a refund." \
        --threshold 0.1:0.9 --quiet < m.txt; \
        echo AFTER'
    before                      # exit 3, the script ends here
    $ echo $?
    3

Exit 1 and exit 3 both end a `set -e` script. The four-outcome `case` example printed beside the warning does cover exit 3, so a reader who reads the whole page is safe. The sentence itself names one of the two outcomes that kill the script.

## Expected

The sentence names both outcomes that end the script. `specification/channels.md` holds exit 3 as a successful unresolved answer, so the warning can name it without calling it a failure.

## How bad it is for a user

Minor. A script that dies on a not-sure answer reads as a thinkthen bug to a user the help did not warn.

Found by experiment 218, wave 1, areas 4 and 12.
