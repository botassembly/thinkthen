# Spreadsheet surfaces need a spike: Google Sheets and Excel

Status: Open. Backlog. Asked by Ian on 2026-09-21: "Is there a way we can make this work in Excel? In Google Sheets?" Nothing here is authorized work.

## Why it matters

A spreadsheet is the largest audience of any surface. A column that classifies itself is also the clearest picture of the product for a business reader. The picture: `=DECIDE("Is this a complaint?", A2:A500)` fills a column.

## What the product side believes, unverified

- **Google Sheets** supports custom functions written as script that runs in Google's cloud. A custom function can take a range and return a range, so a column could cross as one call. The script can make web requests. It cannot load the Rust engine, so this surface would build and send requests itself.
- **Excel** supports custom functions through an Office add-in that runs JavaScript. An add-in may be able to load the engine as WebAssembly. It can batch many cells into one call.
- **Python in Excel** runs in Microsoft's cloud. The product side believes it allows no outside packages and no network calls, so it likely cannot carry the Python library.

Each of these is memory, not a checked fact. The spike checks them first.

## The questions a spike answers

1. Can the engine run as WebAssembly inside an Excel add-in, so the one-engine rule holds? What does the request path look like from inside a browser sandbox?
2. Sheets cannot run the engine. Is a thin script surface acceptable, given ADR 0017's rule that one engine does all the work? If so, what is the smallest honest subset (for example `decide`, `choose`, `score`, `tag` over a range) and how do the shared conformance cases prove it?
3. A spreadsheet recalculates. What stops a recalculation from asking paid questions again? The cache has to live somewhere a sandboxed function can reach.
4. Where does a user's key live, and who can read it in a shared workbook?
5. What are the run-time and size caps on a custom function in each host, and how many rows fit under them?
6. How does "not sure" show in a cell: an empty cell, or the host's own not-available value?

## Recommendation

Run the spike after the libraries land. Excel through an add-in with the WebAssembly engine is the road that keeps the architecture whole, so check that one first. Ian rules on whether a script-only Sheets surface is allowed.
