# QuickFix: portable Windows checksum sidecars

Hosted release rehearsal 37397802256 built all five platforms, then its wheels job 112068632359 refused the Windows wheel checksum on Linux. The downloaded Windows platform artifact confirms that the wheel, command ZIP, C ZIP and first-run sidecars end in CRLF. Linux shasum reads the carriage return as part of the filename. The shared Windows producer used Python text stdout, which performs native newline translation.

The producer now writes ASCII bytes with two spaces and LF directly to stdout. Archive and wheel bytes stay unchanged. The existing Windows packing test models native Python text translation, checks exact LF sidecars for the command and wheel, and sends the packed wheel through the unchanged strict family verifier. Restoring the old producer makes that regression fail. Wrong digests, wrong filenames and malformed archive refusals remain covered.

Focused Windows command packing, Windows C packing and release archive checks passed, as did shell syntax and diff checks. Full landing gates and a new hosted rehearsal remain with the coordinator. No dependency, approval rule, checksum reader or additional verification tool changed.
