// @ts-check
// npm's os and cpu lists cannot express supported platform pairs.
import { PLATFORMS } from "./platforms.js";
if (!PLATFORMS.includes(process.platform + "-" + process.arch)) {
  throw new Error(`thinkthen does not ship a native engine for ${process.platform}/${process.arch}`);
}

if (process.platform === "linux") {
  const report = /** @type {{header?: {glibcVersionRuntime?: string}}} */(process.report.getReport());
  if (!report.header?.glibcVersionRuntime) throw new Error("thinkthen requires glibc on Linux");
}
