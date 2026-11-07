// @ts-check
import { GLIBC_FLOOR } from "./platforms.js";

/** @param {string} version @returns {[number, number]} */
function release(version) {
  const [major = 0, minor = 0] = version.split(".").map(part => Number.parseInt(part, 10));
  return [major, minor];
}

/**
 * The line that refuses a Linux runtime below the glibc floor, or nothing when it meets the floor.
 * @param {object} report the runtime's `process.report.getReport()`
 * @returns {string | undefined}
 */
export function libcRefusal(report) {
  const found = /** @type {{header?: {glibcVersionRuntime?: string}}} */(report).header?.glibcVersionRuntime;
  if (found) {
    const [major, minor] = release(found);
    const [floorMajor, floorMinor] = release(GLIBC_FLOOR);
    if (!(major < floorMajor || (major === floorMajor && minor < floorMinor))) return undefined;
  }
  return `thinkthen needs glibc ${GLIBC_FLOOR} or later on Linux; this system has ${found ? `glibc ${found}` : "none"}`;
}
