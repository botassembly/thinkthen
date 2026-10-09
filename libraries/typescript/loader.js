'use strict';
// Load the native addon for this platform and chip. build-addon.sh places it
// beside this file, and the generated inventory selects the bundled asset.
const { join } = require('node:path');

const assets = require('./native-platforms.json');
const pair = `${process.platform}-${process.arch}`;
if (!Object.hasOwn(assets, pair)) {
  throw new Error(`thinkthen: no native addon for ${pair}; this package ships ${Object.keys(assets).join(', ')}`);
}
const file = join(__dirname, assets[pair]);
try {
  module.exports = require(file);
} catch (error) {
  throw new Error(`thinkthen: the native addon did not load from ${file}; run build-addon.sh`, { cause: error });
}
