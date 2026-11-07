// @ts-check
// npm's os and cpu lists cannot express supported platform pairs.
import { libcRefusal } from "./libc.js";
import { PLATFORMS } from "./platforms.js";
const refusal = !PLATFORMS.includes(process.platform + "-" + process.arch)
  ? `thinkthen does not ship a native engine for ${process.platform}/${process.arch}`
  : process.platform === "linux" ? libcRefusal(process.report.getReport()) : undefined;
if (refusal) {
  console.error(refusal);
  process.exitCode = 1;
}
