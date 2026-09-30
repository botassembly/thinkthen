# The Flutter strict consumer is a copy of Dart consumer bravo

Status: Closed by the quick fix landed as `Land quick fix: the Zig check runs its facts modes and the Flutter host test drops its Dart copy`. Found while building ticket 0314 slice 4c on main at `88649ec0d`. Resolution: `flutter/example/lib/strict.dart` is gone. The Flutter host test `facade_test.dart` makes only the facade call, and `run.py`, `expected-requests.json` and the two planted negatives (`wrong-probability`, `extra-post`) count that one call. Dart consumer bravo keeps the strict behaviors under plain Dart, including the swapped bulk plant. `libraries/dart/check.sh` passed on the branch: the positive `run.py` run counted one facade arrival with a matching body, both planted negatives failed for their named cause, and the embedder passed.

Kind: debt

Pay when: the next change to the Flutter check's request fixture or planted negatives, or before 0.1.

Debt: 021

Paid: 2026-09-30

Severity: low

Keeping it means every change to Dart consumer bravo is made twice, and a missed copy leaves the Flutter host test checking an older reading of the binding.

## The problem

`libraries/dart/flutter/example/lib/strict.dart` (479 nonblank lines) matches `libraries/dart/checks/consumers/bravo/bin/main.dart` except for its import line and its final pass marker. `flutter/example/test/strict_test.dart` runs it under the Flutter toolchain, and `flutter/run.py`, `flutter/plant-check.py` and `flutter/expected-requests.json` count its exact arrivals and bodies. Slice 4c edited both copies the same way.

Deleting `strict.dart` means the Flutter host test keeps only its facade call, and `run.py`'s required counts, the request fixture and the planted negatives move to that one call. The strict behaviors stay covered by bravo under plain Dart. Slice 4c kept the copy because that rewrite changes the Flutter proof, which is outside the port pass.
