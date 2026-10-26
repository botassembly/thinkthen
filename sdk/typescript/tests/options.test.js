import test from "node:test";
import assert from "node:assert/strict";
import { sdk } from "./support.js";

// Feature: each named function maps its own required arguments to admission.
test("required lists numeric thresholds and removed settings reach typed admission", async () => {
  const client = new sdk.ThinkThen({ backend: "stub" });
  const usage = error => error instanceof sdk.UsageError;
  const band = { low: 0.2, high: 0.8 };
  try {
    for (const call of [
      () => client.choose("Team?", "text"),
      () => client.tag("Topics?", "text"),
      () => client.score("Urgency?", "text"),
      () => client.rankStream("Urgency?", ["text"]).next(),
      () => client.recognize(undefined, "text"),
      () => client.relate(undefined, "text"),
    ]) await assert.rejects(async () => call(), usage);
    for (const call of [
      () => client.tag("Topics?", "text", { labels: ["billing"], threshold: band }),
      () => client.find("Who?", "text", { threshold: band }),
      () => client.recognize(["person"], "text", { threshold: band }),
      () => client.relate([{ name: "knows", source: "person", target: "person" }], "text", { threshold: band }),
      () => client.decide("Ready?", "text", { threshold: "0.2:0.8" }),
    ]) await assert.rejects(async () => call(), error => usage(error) && error.code === "usage.threshold");
    for (const options of [{ adapter: "stub" }, { mode: "embedded" }, { keySource: "SECRET" }]) {
      assert.throws(() => new sdk.ThinkThen(options), usage);
    }
    const row = await client.decide("Ready?", { "a/b~c": { "0": "same evidence" } }, { evidence: ["a/b~c", 0], threshold: band });
    const pointer = await client.decide("Ready?", { "a/b~c": { "0": "same evidence" } }, { evidence: "/a~1b~0c/0", threshold: band });
    assert.deepEqual(row, pointer);
  } finally { await client.close(); }
});


// Feature: conversion failures release client ownership on every terminal path.
test("conversion failure streams leave the client on consumption or disposal", async () => {
  const client = new sdk.ThinkThen({ backend: "stub" });
  let retained = 0;
  try {
    for (const terminal of ["next", "return", "toArray", Symbol.asyncDispose]) {
      const rows = client.decideStream(() => {}, ["body"]);
      if (terminal === "next" || terminal === "toArray") {
        await assert.rejects(rows[terminal](), sdk.UsageError);
      } else await rows[terminal]();
      rows.return = async () => { retained++; return { done: true }; };
    }
  } finally { await client.close(); }
  assert.equal(retained, 0);
});
