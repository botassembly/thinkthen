import test from "node:test";
import assert from "node:assert/strict";
import { inspect } from "node:util";
import { sdk, backend, caseFor } from "./support.js";

function frozen(value) {
  if (value === null || typeof value !== "object") return;
  assert(Object.isFrozen(value));
  for (const child of Object.values(value)) frozen(child);
}

// Feature: immutable toolboxes preserve the caller's domain and evidence.
test("toolboxes copy and freeze mutable nested inputs without freezing their owner", async () => {
  const original = {
    topic: { function: "choose", question: "Team?", choices: ["billing", "shipping"], threshold: { low: 0.2, high: 0.8 }, evidence: ["body"] },
    links: { function: "relate", rules: [{ name: "knows", source: "person", target: "person", symmetric: true }] },
  };
  const snapshot = structuredClone(original);
  const toolbox = sdk.defineToolbox(original);
  assert.notEqual(toolbox, original);
  frozen(toolbox);
  assert(!Object.isFrozen(original));
  original.topic.choices[0] = "changed";
  original.topic.threshold.low = 0.9;
  original.topic.evidence[0] = "other";
  original.links.rules[0].name = "changed";
  assert.deepEqual(toolbox, snapshot);
  const client = new sdk.ThinkThen({ backend: "stub" });
  try {
    const fromToolbox = await client.choose(toolbox.topic, { body: "Refund" });
    const direct = await client.choose("Team?", { body: "Refund" }, {
      choices: snapshot.topic.choices, threshold: snapshot.topic.threshold, evidence: snapshot.topic.evidence,
    });
    assert.deepEqual(fromToolbox, direct);
  } finally { await client.close(); }
});

// Feature: lazy calls snapshot question aliases before their first pull.
test("stream creation snapshots options lists and evidence but leaves AbortSignal live", async () => {
  const client = new sdk.ThinkThen({ backend: "stub" });
  const controller = new AbortController();
  const options = { choices: ["billing", "shipping"], threshold: { low: 0.2, high: 0.8 }, evidence: ["body"], signal: controller.signal };
  const record = { body: "Refund", other: "Different" };
  const stream = client.chooseStream("Team?", [record], options);
  options.choices[0] = "changed";
  options.threshold.low = 0.9;
  options.evidence[0] = "other";
  try {
    const [actual] = await stream.toArray();
    const expected = await client.choose("Team?", record, { choices: ["billing", "shipping"], threshold: { low: 0.2, high: 0.8 }, evidence: ["body"] });
    assert.deepEqual(actual, expected);
    const second = client.chooseStream("Team?", [record], { choices: ["billing", "shipping"], signal: controller.signal });
    controller.abort("after creation");
    await assert.rejects(second.next(), error => error instanceof sdk.CancelledError && error.code === "local.cancelled");
  } finally { await stream.return(); await client.close(); }
});

// Feature: input evidence is encoded before pulling a reused mutable object again.
test("reused input objects keep their per-yield evidence and original filter references", async () => {
  const server = await backend(caseFor({}, [], [
    { contains: '"second"', answer: { type: "noul", noul: 0.1 } },
    { answer: { type: "noul", noul: 0.9 } },
  ]));
  const client = new sdk.ThinkThen({ backend: "systemone", url: server.url, batch: 1 });
  const record = { body: "first" };
  const bodies = ["first", "second", "third"];
  function* records() { for (const body of bodies) { record.body = body; yield record; } }
  try {
    const rows = await client.filterStream("Ready?", records(), { evidence: ["body"] }).toArray();
    assert.equal(rows.length, bodies.length);
    for (const [index, item] of rows.entries()) {
      assert.equal(item.item, record);
      assert.equal(item.row.status, "answered");
      assert.equal(item.row.answer, index !== 1);
      assert.equal(item.kept, index !== 1);
      assert.equal(item.row.position, index + 1);
    }
    assert(!Object.isFrozen(record));
    assert.equal(await server.count(), 3);
  } finally { await client.close(); await server.close(); }
});

// Feature: JSON conversion failures are record rows and never become backend calls.
test("missing invalid ids and malformed host values preserve positions and failed single rows", async () => {
  const server = await backend(caseFor({}, [], []));
  const client = new sdk.ThinkThen({ backend: "systemone", url: server.url });
  const invalid = [null, { id: null }, { id: true }, { id: 1.5 }, { id: [] }, { id: {} }, { body: NaN }, { body: undefined }, new Date(0)];
  try {
    const rows = await client.decideStream("Ready?", invalid).toArray();
    assert.equal(rows.length, invalid.length);
    for (const [index, row] of rows.entries()) {
      assert.equal(row.position, index + 1);
      assert.equal(row.function, "decide");
      assert.equal(row.status, "failed");
      assert.equal(row.error.code, index === 0 ? "record.missing" : "record.invalid");
      for (const field of ["answer", "probability", "detail"]) assert(!Object.hasOwn(row, field));
      frozen(row);
    }
    const one = await client.decide("Ready?", { id: false });
    assert.equal(one.status, "failed");
    assert.equal(one.error.code, "record.invalid");
    assert.equal(await server.count(), 0);
  } finally { await client.close(); await server.close(); }
});

// Feature: inspectable clients rows and errors cannot reveal explicit keys.
test("keys never appear in inspect JSON source or errors", async () => {
  const key = "test-secret-never-display-this";
  const client = new sdk.ThinkThen({ backend: "stub", apiKey: key });
  try {
    const row = await client.decide("Ready?", "evidence");
    const failure = await client.decide("Ready?", null);
    let caught;
    try { await client.decide("", "evidence"); } catch (error) { caught = error; }
    assert(caught instanceof sdk.UsageError);
    for (const value of [client, row, failure, caught, caught.cause]) {
      assert(!inspect(value, { depth: null, showHidden: true }).includes(key));
      assert(!(JSON.stringify(value) ?? "").includes(key));
    }
    frozen(row);
    frozen(failure);
  } finally { await client.close(); }
});

// Feature: status-specific rows expose precise typed detail and frozen owned data.
test("answered unsure and failed rows map probabilities detail and HTTP status without raw replies", async () => {
  const probability = 0.8731942765123456;
  const server = await backend(caseFor({}, [], [
    { contains: '"bad"', status: 400, body: {} },
    { answer: { type: "choice", probabilities: { billing: probability, shipping: 1 - probability } } },
  ]));
  const client = new sdk.ThinkThen({ backend: "systemone", url: server.url, batch: 1 });
  try {
    const answered = await client.choose("Team?", "good", { choices: ["billing", "shipping"], threshold: 0.5 });
    const unsure = await client.choose("Team?", "good", { choices: ["billing", "shipping"], threshold: 0.95 });
    const failed = await client.choose("Team?", "bad", { choices: ["billing", "shipping"] });
    assert.equal(answered.status, "answered");
    assert.equal(answered.answer, "billing");
    assert.equal(answered.probability, probability);
    assert.equal(answered.detail.labels.billing, probability);
    assert.equal(answered.detail.labels.shipping, 1 - probability);
    assert.equal(unsure.status, "unsure");
    assert.equal(unsure.probability, probability);
    assert(!Object.hasOwn(unsure, "answer"));
    assert.equal(failed.status, "failed");
    assert.equal(failed.error.code, "backend.status");
    assert.equal(failed.error.httpStatus, 400);
    assert(!Object.hasOwn(failed, "answer"));
    for (const row of [answered, unsure, failed]) {
      assert.equal(row.function, "choose");
      assert.equal(row.position, 1);
      assert.equal(row.source.backend, "systemone");
      assert.equal(typeof row.source.questionIdentity, "string");
      assert(!Object.hasOwn(row, "p"));
      assert(!Object.hasOwn(row, "raw"));
      frozen(row);
    }
    assert.throws(() => { answered.detail.labels.billing = 0; }, TypeError);
    await client.close();
    assert.equal(answered.detail.labels.billing, probability);
    assert.equal(await server.count(), 3);
  } finally { await client.close(); await server.close(); }
});

// Feature: host ids retain string or integer type while position stays independent.
test("original ids preserve their types and records without ids do not invent one", async () => {
  const client = new sdk.ThinkThen({ backend: "stub" });
  const ids = ["17", 17, -17, Number.MAX_SAFE_INTEGER];
  try {
    const rows = await client.decideStream("Ready?", [...ids.map(id => ({ id, body: "evidence" })), { body: "no id" }]).toArray();
    assert.equal(rows.length, ids.length + 1);
    for (const [index, id] of ids.entries()) {
      assert.equal(rows[index].id, id);
      assert.equal(typeof rows[index].id, typeof id);
      assert.equal(rows[index].position, index + 1);
    }
    assert(!Object.hasOwn(rows.at(-1), "id"));
    assert.equal(rows.at(-1).position, ids.length + 1);
  } finally { await client.close(); }
});

// Feature: exported codes and row-code contexts derive from the engine registry.
test("installed error code values equal the engine registry", async () => {
  const { readFile } = await import("node:fs/promises");
  const registry = JSON.parse(await readFile(new URL("../../../schemas/contract.json", import.meta.url), "utf8"));
  assert.deepEqual(sdk.ERROR_CODES, registry.codes.map(entry => entry.name));
  assert(Object.isFrozen(sdk.ERROR_CODES));
});

// Feature: raw strings preserve empty, NUL and Unicode evidence.
test("raw and structured evidence retain the same bytes", async () => {
  const client = new sdk.ThinkThen({backend:"stub"});
  try {
    for (const text of ["", "a\0b\0", 'quoted "text"\n😀']) {
      assert.deepEqual(await client.decide("Ready?", text, {evidence:"/body"}), await client.decide("Ready?", {body:text}, {evidence:"/body"}));
    }
  } finally { await client.close(); }
});

// Feature: native ID descriptors retain exact integers while rejecting lossy host values.
test("bigint is accepted only in a root id and stays exact through native evidence selection", async () => {
  const client = new sdk.ThinkThen({backend:"stub"});
  try {
    for (const [id, expected] of [[0n,0], [9007199254740991n,9007199254740991], [9007199254740992n,9007199254740992n], [-9223372036854775808n,-9223372036854775808n], [18446744073709551615n,18446744073709551615n], ["", ""]]) {
      const row = await client.decide("Ready?", {id,body:"same"});
      assert.equal(row.status,"answered");
      assert.equal(row.id,expected);
    }
    for (const record of [{id:9007199254740992},{id:1.5},{id:18446744073709551616n},{id:-9223372036854775809n},{body:{id:1n}}]) {
      const row = await client.decide("Ready?",record);
      assert.equal(row.status,"failed");
      assert.equal(row.error.code,"record.invalid");
    }
    const numeric = await client.decide("Ready?",{id:18446744073709551615n},{evidence:"/id"});
    const text = await client.decide("Ready?","18446744073709551615",{evidence:"/id"});
    assert.equal(numeric.probability,text.probability);
    assert.equal(numeric.answer,text.answer);
  } finally { await client.close(); }
});

// Feature: execution names remain separate from typed toolbox selection.
test("named calls carry source names while selecting a question never supplies one", async () => {
  const client = new sdk.ThinkThen({backend:"stub"});
  const toolbox = sdk.defineToolbox({topic:{function:"choose",question:"Team?",choices:["billing","shipping"]}});
  try {
    const selected = await client.choose(toolbox.topic,"Refund");
    assert(!Object.hasOwn(selected.source,"name"));
    const named = await client.choose(toolbox.topic,"Refund",{name:"triage"});
    assert.equal(named.source.name,"triage");
    assert.notEqual(named.source.questionIdentity,selected.source.questionIdentity);
    const direct = await client.choose("Team?","Refund",{choices:["billing","shipping"],name:"triage"});
    assert.deepEqual(named,direct);
    const rows = await client.chooseStream(toolbox.topic,["Refund"],{name:"batch-triage"}).toArray();
    assert.equal(rows.length,1);
    assert.equal(rows[0].source.name,"batch-triage");
    await client.close();
    await assert.rejects(client.decide("Ready?","closed"), error => error instanceof sdk.UsageError && error.code === "usage.abi");
  } finally { await client.close(); }
});

// Feature: host field translation never changes caller-defined labels or toolbox names.
test("labels and annotation entries keep names that match native field names", async () => {
  const client = new sdk.ThinkThen({backend:"stub"});
  const labels = ["function","question_identity","http_status","either"];
  try {
    const chosen = await client.choose("Which?","evidence",{choices:labels});
    assert.equal(chosen.status,"answered");
    assert.deepEqual(Object.keys(chosen.detail.labels).sort(),[...labels].sort());
    const toolbox = sdk.defineToolbox(Object.fromEntries(labels.map(name => [name,{function:"choose",question:"Which?",choices:labels}])));
    const annotated = await client.annotate(toolbox,"evidence");
    assert.equal(annotated.status,"answered");
    assert.deepEqual(Object.keys(annotated.answer).sort(),[...labels].sort());
    for (const name of labels) assert.deepEqual(Object.keys(annotated.answer[name].detail.labels).sort(),[...labels].sort());
  } finally { await client.close(); }
});

// Feature: record encoding captures each property once before native evidence selection.
test("record getters cannot change evidence between validation and encoding", async () => {
  const client = new sdk.ThinkThen({backend:"stub"});
  let idReads = 0;
  let bodyReads = 0;
  const record = {
    get id() { idReads++; return idReads === 1 ? 17 : "changed"; },
    get body() { bodyReads++; return bodyReads === 1 ? "first" : NaN; },
  };
  try {
    const actual = await client.decide("Ready?",record,{evidence:"/body"});
    const expected = await client.decide("Ready?",{id:17,body:"first"},{evidence:"/body"});
    assert.deepEqual(actual,expected);
    assert.equal(idReads,1);
    assert.equal(bodyReads,1);
    const date = new Date(0);
    date.id = 17n;
    const invalid = await client.decide("Ready?",date);
    assert.equal(invalid.status,"failed");
    assert.equal(invalid.error.code,"record.invalid");
  } finally { await client.close(); }
});

// Feature: conversion errors preserve settings/question/record contexts without retaining caller secrets.
test("unsupported host values produce coded secret-free errors before requests or input pulls", async () => {
  const server = await backend(caseFor({},[],[]));
  const client = new sdk.ThinkThen({url:server.url});
  const secret = "private-conversion-failure-do-not-retain";
  const cycle = {}; cycle.self = cycle;
  const throwing = {toJSON(){throw new Error(secret);}};
  const failures = [cycle,1n,throwing,undefined,()=>{},Symbol("unsupported"),{[Symbol("unsupported")]:true}];
  const usage = code => error => {
    assert(error instanceof sdk.UsageError);
    assert.equal(error.code,code);
    assert(!inspect(error,{depth:null,showHidden:true}).includes(secret));
    return true;
  };
  try {
    for (const value of failures) {
      assert.throws(()=>new sdk.ThinkThen({url:server.url,apiKey:secret,batch:value}),usage("usage.settings"));
      await assert.rejects(client.decide("Ready?","evidence",{threshold:value}),usage("usage.question"));
      await assert.rejects(client.annotate({ready:{function:"decide",question:"Ready?",threshold:value}},"evidence"),usage("usage.question"));
      await assert.rejects(client.annotate({ready:value},"evidence"),usage("usage.question"));
      let pulled = 0;
      const source = {[Symbol.iterator](){return this;},next(){pulled++;return {done:true};}};
      await assert.rejects(client.decideStream("Ready?",source,{threshold:value}).next(),usage("usage.question"));
      assert.equal(pulled,0);
      const row = await client.decide("Ready?",{body:value});
      assert.equal(row.status,"failed");
      assert.equal(row.error.code,"record.invalid");
      assert(!inspect(row,{depth:null,showHidden:true}).includes(secret));
    }
    const toolbox = {ready:{function:"decide",question:"Ready?"},[Symbol("unsupported")]:{function:"decide",question:"Other?"}};
    await assert.rejects(client.annotate(toolbox,"evidence"),usage("usage.question"));
    assert.equal(await server.count(),0);
  } finally {await client.close();await server.close();}
});

// Feature: unordered relation rules translate direction in both input and output.
test("symmetric relation rules retain the native direction flag", async () => {
  const {readFile} = await import("node:fs/promises");
  const fixture = JSON.parse(await readFile(new URL("../../../conformance/typed/relate-rounds.json",import.meta.url),"utf8"));
  const server = await backend({replies:fixture.replies});
  const client = new sdk.ThinkThen({url:server.url});
  try {
    const rules = fixture.request.question.rules.map(({relation,reads,either,...rule})=>({...rule,name:relation,symmetric:true}));
    const text = fixture.request.records[0];
    const row = await client.relate(rules,text);
    assert.equal(row.status,"answered");
    assert.equal(row.answer.length,1);
    assert.equal(row.answer[0].symmetric,true);
    assert.equal(text.slice(row.answer[0].source.start,row.answer[0].source.end),row.answer[0].source.text);
  } finally {await client.close();await server.close();}
});
