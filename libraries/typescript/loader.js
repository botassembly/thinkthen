'use strict';
// Load the native addon that build-addon.sh places beside this file.
const { join } = require('node:path');

try {
  module.exports = require(join(__dirname, 'thinkthen.node'));
} catch (error) {
  throw new Error(`thinkthen: the native addon did not load from ${join(__dirname, 'thinkthen.node')}; run build-addon.sh`, { cause: error });
}
