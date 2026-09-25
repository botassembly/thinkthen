'use strict';
// Load the native addon for this platform and chip. build-addon.sh places it
// beside this file, and the published package carries all four (ticket 0128).
const { join } = require('node:path');

const SHIPPED = ['linux-x64', 'linux-arm64', 'darwin-x64', 'darwin-arm64'];
const pair = `${process.platform}-${process.arch}`;
if (!SHIPPED.includes(pair)) {
  throw new Error(`thinkthen: no native addon for ${pair}; this package ships linux-x64, linux-arm64, darwin-x64, and darwin-arm64`);
}
const file = join(__dirname, `thinkthen-${pair}.node`);
try {
  module.exports = require(file);
} catch (error) {
  throw new Error(`thinkthen: the native addon did not load from ${file}; run build-addon.sh`, { cause: error });
}
