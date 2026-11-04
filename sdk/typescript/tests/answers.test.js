import test from "node:test";
import assert from "node:assert/strict";
import { sdk, backend, caseFor } from "./support.js";

test("single await returns its answer and row reads the same execution", async () => {
  const server = await backend(caseFor({}, [], [{ answer: {type:"noul", noul:0.1} }]));
  const client = new sdk.ThinkThen({url:server.url});
  try {
    const call = client.decide("Ready?", "evidence");
    assert.equal(typeof call.row, "function");
    assert.equal(await call, false);
    const row = await call.row();
    assert.equal(row.status, "answered");
    assert.equal(row.answer, false);
    assert.equal(sdk.answer(row), false);
    assert.equal(await server.count(), 1);
  } finally {await client.close(); await server.close();}
});

test("unsure and failed answers reject while their rows stay available", async () => {
  const server = await backend(caseFor({}, [], [{ answer: {type:"noul", noul:0.5} }]));
  const client = new sdk.ThinkThen({url:server.url});
  try {
    const call = client.decide("Ready?", "evidence", {threshold:{low:0.2,high:0.8}});
    await assert.rejects(Promise.resolve(call), error => error instanceof sdk.UnsureError && error.probability === 0.5);
    assert.equal((await call.row()).status, "unsure");
    const missing = client.decide("Ready?", null);
    await assert.rejects(Promise.resolve(missing), error => error instanceof sdk.RecordError && error.code === "record.missing" && error.rows.length === 1);
    assert.equal((await missing.row()).error.code, "record.missing");
    assert.equal(await server.count(), 1);
  } finally {await client.close(); await server.close();}
});

test("kept filter records precede one error holding every failed row", async () => {
  const server = await backend(caseFor({}, [], [
    {contains:'"drop"', answer:{type:"noul",noul:0.1}},
    {answer:{type:"noul",noul:0.9}},
  ]));
  const client = new sdk.ThinkThen({url:server.url,batch:1});
  const kept = {body:"keep", extra:17};
  const received = [];
  try {
    await assert.rejects(async () => {
      for await (const record of client.filter("Ready?", [null, kept, "drop", {id:false}], {field:"/body"})) received.push(record);
    }, error => error instanceof sdk.RecordError && error.code === "record.missing" && error.rows.length === 2);
    assert.deepEqual(received, [kept]);
    assert.equal(received[0], kept);
    assert.equal(await server.count(), 2);
  } finally {await client.close(); await server.close();}
});

test("one-record arrays and stream strings receive shared usage errors", async () => {
  const server = await backend(caseFor({}, [], []));
  const client = new sdk.ThinkThen({url:server.url});
  const kindError = error => error instanceof sdk.UsageError && error.code === "usage.record-kind";
  try {
    for (const collection of [["one", "two"], new Set(["one"]), (async function* () { yield "one"; })()]) {
      await assert.rejects(client.decide("Ready?", collection), kindError);
    }
    await assert.rejects(client.decideStream("Ready?", "characters").toArray(), kindError);
    assert.equal(await server.count(), 0);
  } finally {await client.close(); await server.close();}
});

test("undefined is missing in single and stream calls", async () => {
  const client = new sdk.ThinkThen({backend:"stub"});
  try {
    await assert.rejects(client.decide("Ready?", undefined), error => error instanceof sdk.RecordError && error.code === "record.missing");
    const rows = await client.decideStream("Ready?", [undefined, "real"]).toArray();
    assert.equal(rows.length, 2);
    assert.equal(rows[0].error.code, "record.missing");
    assert.equal(rows[1].status, "answered");
  } finally {await client.close();}
});

test("module functions reuse but never close the caller's default client", async () => {
  const client = new sdk.ThinkThen({backend:"stub"});
  sdk.setDefaultClient(client);
  try {
    const call = sdk.decide("Ready?", "evidence");
    assert.equal(typeof await call, "boolean");
    assert.equal((await call.row()).status, "answered");
    assert.equal((await sdk.decideStream("Ready?", ["next"]).toArray()).length, 1);
    assert.equal((await client.decide("Ready?", "still open").row()).status, "answered");
  } finally {sdk.setDefaultClient(undefined); await client.close();}
});

test("every single function preserves the row answer domain", async () => {
  const client = new sdk.ThinkThen({backend:"stub"});
  const calls = [
    () => client.choose("Team?", "evidence", {choices:["billing", "shipping"]}),
    () => client.tag("Topics?", "evidence", {labels:["billing", "shipping"]}),
    () => client.score("Priority?", "evidence", {levels:["low", "high"]}),
    () => client.find("Name?", "Ada works here"),
    () => client.recognize(["person"], "Ada works here"),
    () => client.relate([{name:"knows", source:"person", target:"person"}], "Ada knows Grace"),
    () => client.annotate({ready:{function:"decide",question:"Ready?"}}, "evidence"),
  ];
  try {
    for (const make of calls) {
      const call = make();
      const row = await call.row();
      if (row.status === "answered") assert.deepEqual(await call, row.answer);
      else if (row.status === "unsure") await assert.rejects(call, sdk.UnsureError);
      else await assert.rejects(call, sdk.RecordError);
    }
  } finally {await client.close();}
});

test("loadToolbox reads saved definitions through shared admission", async () => {
  const {mkdtemp, writeFile, rm} = await import("node:fs/promises");
  const {tmpdir} = await import("node:os");
  const {join} = await import("node:path");
  const directory = await mkdtemp(join(tmpdir(), "thinkthen-toolbox-"));
  const path = join(directory, "questions.json");
  const definitions = {ready:{function:"decide",question:"Ready?",threshold:0.5}};
  try {
    await writeFile(path, sdk.serializeToolbox(definitions));
    assert.deepEqual(await sdk.loadToolbox(path), definitions);
    assert(Object.isFrozen(await sdk.loadToolbox(path)));
  } finally {await rm(directory, {recursive:true});}
});

test("shared descriptions round trip through the host question names", async () => {
  const definitions = {
    ready:{function:"decide",question:"Ready?",threshold:0.5,yesMeans:"Prepared",noMeans:"Not prepared"},
    team:{function:"choose",question:"Team?",choices:["billing","shipping"],threshold:0.5,descriptions:{billing:"Payment issues"}},
    priority:{function:"score",question:"Priority?",levels:["low","high"],descriptions:{high:"Urgent"}},
  };
  assert.deepEqual(sdk.parseToolbox(sdk.serializeToolbox(definitions)), definitions);
  assert.throws(() => sdk.serializeToolbox({team:{...definitions.team,descriptions:{unknown:"Not a choice"}}}), sdk.UsageError);
});

test("annotation entries extract their answers or throw typed errors in one step", async () => {
  const {readFile} = await import("node:fs/promises");
  const {fixtureJson} = await import("./canonical.js");
  const fixture = fixtureJson(await readFile(new URL("../../../conformance/typed/annotate-isolation.json", import.meta.url), "utf8"));
  const server = await backend({replies:fixture.replies});
  const client = new sdk.ThinkThen({url:server.url});
  try {
    const entries = await client.annotate(fixture.request.question.questions, fixture.request.records[0], {field:"/body"});
    assert.equal(sdk.answer(entries.decision), true);
    assert.equal(sdk.answer(entries.choice), "z");
    assert.throws(() => sdk.answer(entries.choice_unsure), error => error instanceof sdk.UnsureError && error.row === entries.choice_unsure && error.probability === entries.choice_unsure.probability);
    assert.throws(() => sdk.answer(entries.missing), error => error instanceof sdk.RecordError && error.row === entries.missing && error.code === "record.missing");
  } finally {await client.close(); await server.close();}
});

test("environment backend applies only when the caller omits it", async () => {
  const previous = process.env.THINKTHEN_BACKEND;
  const server = await backend(caseFor({}, [], [{answer:{type:"noul",noul:0.9}}]));
  let inferred, explicit;
  try {
    process.env.THINKTHEN_BACKEND = "stub";
    inferred = new sdk.ThinkThen({url:server.url});
    assert.equal((await inferred.decide("Ready?", "evidence").row()).source.backend, "stub");
    assert.equal(await server.count(), 0);
    explicit = new sdk.ThinkThen({backend:"systemone",url:server.url});
    assert.equal((await explicit.decide("Ready?", "evidence").row()).source.backend, "systemone");
    assert.equal(await server.count(), 1);
    process.env.THINKTHEN_BACKEND = "not-a-backend";
    assert.throws(() => new sdk.ThinkThen(), error => error instanceof sdk.UsageError && error.code === "usage.backend");
  } finally {
    if (previous === undefined) delete process.env.THINKTHEN_BACKEND;
    else process.env.THINKTHEN_BACKEND = previous;
    await inferred?.close(); await explicit?.close(); await server.close();
  }
});

test("rank rows retain original input records including failures and top limits", async () => {
  const records = [{id:"one",text:"first",other:17}, null, {id:"three",text:"third",other:29}];
  const client = new sdk.ThinkThen({backend:"stub"});
  try {
    async function* input() {yield* records;}
    const rows = await client.rankStream("Priority?", input(), {levels:["low","high"],field:"/text"}).toArray();
    assert.equal(rows.length, records.length);
    for (const row of rows) {
      assert.deepEqual(row.item, records[row.position - 1]);
      if (row.item !== null && typeof row.item === "object") assert.notEqual(row.item, records[row.position - 1]);
    }
    const limited = await client.rankStream("Priority?", records, {levels:["low","high"],field:"/text",top:1}).toArray();
    assert.equal(limited.length, 2);
    for (const row of limited) assert.deepEqual(row.item, records[row.position - 1]);
    assert.equal(limited.filter(row => row.status === "failed").length, 1);
    assert(!Object.isFrozen(records[0]));
  } finally {await client.close();}
});


test("description names that match object properties survive toolbox round trips", () => {
  const descriptions = Object.fromEntries([["__proto__", "First"], ["constructor", "Second"], ["toString", "Third"]]);
  const names = Object.keys(descriptions);
  const definitions = {
    choice:{function:"choose",question:"Which?",choices:names,threshold:0.5,descriptions},
    tags:{function:"tag",question:"Which?",labels:names,threshold:0.5,descriptions},
    score:{function:"score",question:"Which?",levels:names,descriptions},
  };
  assert.deepEqual(sdk.parseToolbox(sdk.serializeToolbox(definitions)), definitions);
});

test("rank descriptions survive saved toolboxes and reach both backend requests", async () => {
  const definition = {function:"rank",question:"Rank priority?",levels:["low","high"],descriptions:{high:"Urgent"},top:1};
  const saved = sdk.serializeToolbox({priority:definition});
  const loaded = sdk.parseToolbox(saved);
  assert.deepEqual(loaded.priority, definition);
  for (const backendName of ["systemone", "openai"]) {
    const server = await backend({replies:[
      {contains:"Urgent",distribution:{low:0.1,high:0.9}},
      {distribution:{low:0.9,high:0.1}},
    ]});
    let client;
    try {
      client = new sdk.ThinkThen({backend:backendName,url:server.url,apiKey:"test-only"});
      const rows = await client.rankStream(loaded.priority, ["evidence"]).toArray();
      assert.equal(rows.length, 1);
      assert.equal(rows[0].answer, 1);
      assert.deepEqual(rows[0].detail.levels, [0.1,0.9]);
      assert.equal(await server.count(), 1);
    } finally {await client?.close(); await server.close();}
  }
});

test("filter answer meanings survive the saved host question", async () => {
  const definition = {function:"filter",question:"Ready?",threshold:0.5,yesMeans:"Prepared",noMeans:"Needs work"};
  assert.deepEqual(sdk.parseToolbox(sdk.serializeToolbox({ready:definition})).ready, definition);
});

test("rank snapshots preserve bigint ids, missing values and representable invalid records", async () => {
  const records = [{id:17n,text:"safe bigint"}, {id:18446744073709551615n,text:"wide bigint"},
    {id:17,text:"number"}, {id:"17",text:"string"}, null, undefined, {id:true,text:"bad id"}, new Date(0)];
  const client = new sdk.ThinkThen({backend:"stub"});
  try {
    const rows = await client.rankStream("Priority?", records, {levels:["low","high"],field:"/text"}).toArray();
    assert.equal(rows.length, records.length);
    for (const row of rows) {
      const source = records[row.position - 1];
      assert.deepEqual(row.item, source instanceof Date ? undefined : source);
      if (row.item !== null && typeof row.item === "object") {
        assert.notEqual(row.item, source);
        assert(Object.isFrozen(row.item));
      }
    }
  } finally {await client.close();}
});

test("rank snapshots preserve caller fields that resemble engine metadata", async () => {
  const records = [
    {text:"first",function:"decide",source:{question_identity:"caller field"},error:{code:"record.missing",message:"caller data",http_status:418}},
    {text:"second",nested:[{function:"customer data",error:{code:"my.application.code",message:"literal"}}]},
  ];
  const client = new sdk.ThinkThen({backend:"stub"});
  try {
    const rows = await client.rankStream("Priority?", records, {levels:["low","high"],field:"/text"}).toArray();
    assert.equal(rows.length, records.length);
    for (const row of rows) {
      assert.deepEqual(row.item, records[row.position - 1]);
      assert(Object.isFrozen(row.item));
    }
  } finally {await client.close();}
});
