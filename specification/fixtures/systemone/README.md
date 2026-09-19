# systemone fixtures

Each case is a pair: `NAME.request.json` is the body `encode` must produce, and `NAME.response.json` is a body `decode` must read. A `refused-` file is a response `decode` must refuse. Compare JSON by value and never by bytes.
