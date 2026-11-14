import test from "node:test";
import assert from "node:assert/strict";
import { sdk, backend, caseFor } from "./support.js";

test("await returns one result and pending state reads require await", async () => {
  const server = await backend(caseFor({}, [], [{ probability: 0.1 }]));
  const client = new sdk.Client({url:server.url});
  try {
    const call = client.decide("Ready?", "evidence");
    for (const key of ["answer", "answered", "failed", "unsure", "status", "probability", "error"]) {
      assert.throws(() => call[key], error => error instanceof TypeError && /await/.test(error.message));
    }
    assert.equal(call.row, undefined);
    const row = await call;
    assert.equal(row.answered, true);
    assert.equal(row.answer, false);
    assert.equal(String(row), "false");
    assert.equal(row.throwIfFailed(), row);
    assert.equal(await server.count(), 1);
  } finally {await client.close(); await server.close();}
});

test("missing answers are loud while printing state and saving remain safe", async () => {
  const server = await backend(caseFor({}, [], [{ probability: 0.5 }]));
  const client = new sdk.Client({url:server.url});
  try {
    const unsure = await client.decide("Ready?", "evidence", {threshold:{low:0.2,high:0.8}});
    assert.throws(() => unsure.answer, error => error instanceof sdk.UnsureError && error instanceof sdk.NoAnswerError
      && !(error instanceof sdk.ThinkThenError) && error.row === unsure && error.probability === 0.5);
    assert.equal(unsure.throwIfFailed(), unsure);
    assert.equal(String(unsure), "unsure");
    for (const blank of ["", "   ", {}, null, undefined]) {
      const failure = await client.decide("Ready?", blank).catch(error => error);
      assert(failure instanceof sdk.FailedError);
      const row = failure.row;
      assert.equal(failure.rows[0], row);
      assert(row.failed);
      assert.equal(row.item, blank);
      assert.throws(() => row.answer, error => error instanceof sdk.FailedError && error instanceof sdk.NoAnswerError
        && error instanceof sdk.ThinkThenError && !(error instanceof sdk.UnsureError) && error.rows[0] === row);
      assert.throws(() => row.throwIfFailed(), sdk.FailedError);
      assert.equal(JSON.parse(JSON.stringify(row)).status, "failed");
      assert.match(String(row), /^failed: record\./);
      assert.doesNotThrow(() => new Set([row]).has(row));
    }
    assert.equal(await server.count(), 1);
  } finally {await client.close(); await server.close();}
});

test("kept filter records precede one error holding every failed row", async () => {
  const server = await backend(caseFor({}, [], [
    {contains:'"drop"', answer:{type:"noul",noul:0.1}},
    {answer:{type:"noul",noul:0.9}},
  ]));
  const client = new sdk.Client({url:server.url,batch:1});
  const kept = {body:"keep", extra:17};
  const received = [];
  try {
    await assert.rejects(async () => {
      for await (const record of client.filter("Ready?", [null, kept, "drop", {id:false}], {field:"/body"})) received.push(record);
    }, error => error instanceof sdk.FailedError && error.code === "record.missing" && error.rows.length === 2);
    assert.deepEqual(received, [kept]);
    assert.equal(received[0], kept);
    assert.equal(await server.count(), 2);
  } finally {await client.close(); await server.close();}
});

test("one-record arrays and stream strings receive shared usage errors", async () => {
  const server = await backend(caseFor({}, [], []));
  const client = new sdk.Client({url:server.url});
  const kindError = error => error instanceof TypeError && /decideStream/.test(error.message);
  try {
    for (const collection of [["one", "two"], new Set(["one"]), (async function* () { yield "one"; })()]) {
      await assert.rejects(client.decide("Ready?", collection), kindError);
    }
    await assert.rejects(client.decideStream("Ready?", "characters").toArray(), error => error instanceof sdk.UsageError && error.code === "usage.record-kind");
    assert.equal(await server.count(), 0);
  } finally {await client.close(); await server.close();}
});

test("undefined is missing in single and stream calls", async () => {
  const client = new sdk.Client({backend:"stub"});
  try {
    await assert.rejects(client.decide("Ready?", undefined), error => error instanceof sdk.FailedError && error.code === "record.missing");
    const rows = await client.decideStream("Ready?", [undefined, "real"]).toArray();
    assert.equal(rows.length, 2);
    assert.equal(rows[0].error.code, "record.missing");
    assert.equal(rows[1].status, "answered");
  } finally {await client.close();}
});

test("module functions return results through their shared default client", async () => {
  const previous = process.env.THINKTHEN_BACKEND;
  process.env.THINKTHEN_BACKEND = "stub";
  try {
    assert.equal((await sdk.decide("Ready?", "evidence")).status, "answered");
    assert.equal((await sdk.decide("Ready?", "next")).status, "answered");
    assert.equal((await sdk.decideStream("Ready?", ["next"]).toArray()).length, 1);
  } finally {
    if (previous === undefined) delete process.env.THINKTHEN_BACKEND;
    else process.env.THINKTHEN_BACKEND = previous;
  }
});

test("every single function preserves the row answer domain", async () => {
  const client = new sdk.Client({backend:"stub"});
  const calls = [
    () => client.choose("Team?", "evidence", {choices:["billing", "shipping"]}),
    () => client.tag("Topics?", "evidence", {labels:["billing", "shipping"]}),
    () => client.score("Priority?", "evidence", {levels:["low", "high"]}),
    () => client.find("Name?", "Ada works here"),
    () => client.recognize("Ada works here", {kinds: ["person"]}),
    () => client.relate("Ada knows Grace", {rules: [{relation:"knows", source:"person", target:"person"}]}),
    () => client.annotate({ready:{function:"decide",question:"Ready?"}}, "evidence"),
  ];
  try {
    for (const make of calls) {
      const call = make();
      const row = await call;
      assert.equal(await call, row);
      if (row.status === "answered") assert.doesNotThrow(() => row.answer);
      else if (row.status === "unsure") assert.throws(() => row.answer, sdk.UnsureError);
      else assert.fail("a fulfilled scalar call returned a failed row");
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
    const toolbox = await sdk.Toolbox.load(path);
    assert.deepEqual(toolbox.decide("ready"), definitions.ready);
    assert(Object.isFrozen(toolbox.decide("ready")));
    assert.throws(() => toolbox.choose("ready"), sdk.UsageError);
    assert.throws(() => toolbox.decide("absent"), sdk.UsageError);
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
  const client = new sdk.Client({url:server.url});
  try {
    const entries = (await client.annotate(fixture.request.question.questions, fixture.request.records[0], {field:"/body"})).answer;
    assert.equal(entries.decision.answer, true);
    assert.equal(entries.choice.answer, "z");
    assert.throws(() => entries.choice_unsure.answer, error => error instanceof sdk.UnsureError && error.row === entries.choice_unsure && error.probability === entries.choice_unsure.probability);
    assert.throws(() => entries.missing.answer, error => error instanceof sdk.FailedError && error.row === entries.missing && error.code === "record.missing");
  } finally {await client.close(); await server.close();}
});

test("environment backend applies only when the caller omits it", async () => {
  const previous = process.env.THINKTHEN_BACKEND;
  const server = await backend(caseFor({}, [], [{answer:{type:"noul",noul:0.9}}]));
  let inferred, explicit;
  try {
    process.env.THINKTHEN_BACKEND = "stub";
    inferred = new sdk.Client({url:server.url});
    assert.equal((await inferred.decide("Ready?", "evidence")).source.backend, "stub");
    assert.equal(await server.count(), 0);
    explicit = new sdk.Client({backend:"systemone",url:server.url});
    assert.equal((await explicit.decide("Ready?", "evidence")).source.backend, "systemone");
    assert.equal(await server.count(), 1);
    process.env.THINKTHEN_BACKEND = "not-a-backend";
    assert.throws(() => new sdk.Client(), error => error instanceof sdk.UsageError && error.code === "usage.backend");
  } finally {
    if (previous === undefined) delete process.env.THINKTHEN_BACKEND;
    else process.env.THINKTHEN_BACKEND = previous;
    await inferred?.close(); await explicit?.close(); await server.close();
  }
});

test("rank rows retain original input records including failures and top limits", async () => {
  const records = [{id:"one",text:"first",other:17}, null, {id:"three",text:"third",other:29}];
  const client = new sdk.Client({backend:"stub"});
  try {
    async function* input() {yield* records;}
    const rows = await client.rank("Priority?", input(), {levels:["low","high"],field:"/text"}).toArray();
    assert.equal(rows.length, records.length);
    for (const row of rows) {
      assert.deepEqual(row.item, records[row.position - 1]);
      if (row.item !== null && typeof row.item === "object") assert.equal(row.item, records[row.position - 1]);
    }
    const limited = await client.rank("Priority?", records, {levels:["low","high"],field:"/text",top:1}).toArray();
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
      client = new sdk.Client({backend:backendName,url:server.url,apiKey:"test-only"});
      const rows = await client.rank(loaded.priority, ["evidence"]).toArray();
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
  const client = new sdk.Client({backend:"stub"});
  try {
    const rows = await client.rank("Priority?", records, {levels:["low","high"],field:"/text"}).toArray();
    assert.equal(rows.length, records.length);
    for (const row of rows) {
      const source = records[row.position - 1];
      assert.equal(row.item, source);
      if (row.item !== null && typeof row.item === "object") {
        assert.equal(row.item, source);
        assert(!Object.isFrozen(row.item));
      }
    }
  } finally {await client.close();}
});

test("rank snapshots preserve caller fields that resemble engine metadata", async () => {
  const records = [
    {text:"first",function:"decide",source:{question_identity:"caller field"},error:{code:"record.missing",message:"caller data",http_status:418}},
    {text:"second",nested:[{function:"customer data",error:{code:"my.application.code",message:"literal"}}]},
  ];
  const client = new sdk.Client({backend:"stub"});
  try {
    const rows = await client.rank("Priority?", records, {levels:["low","high"],field:"/text"}).toArray();
    assert.equal(rows.length, records.length);
    for (const row of rows) {
      assert.deepEqual(row.item, records[row.position - 1]);
      assert(!Object.isFrozen(row.item));
    }
  } finally {await client.close();}
});

test("canonical saving keeps exact ids and character coordinates after close and caller mutation", async () => {
  const {readFile} = await import("node:fs/promises");
  const {fixtureJson} = await import("./canonical.js");
  const fixture = fixtureJson(await readFile(new URL("../../../conformance/typed/find-unicode.json", import.meta.url), "utf8"));
  for (const selected of ["systemone", "openai"]) {
    const server = await backend({replies:fixture.replies});
    let client;
    const original = {id:18446744073709551615n,text:fixture.request.records[0]};
    try {
      client = new sdk.Client({backend:selected,url:server.url,apiKey:"synthetic-loopback-key"});
      const row = await client.find("Which line?", original, {field:"/text"});
      assert.equal(row.item, original);
      assert.equal(row.id, original.id);
      assert.equal(original.text.slice(row.answer.start,row.answer.end), row.answer.text);
      assert.equal(row.answer.start, 7);
      assert.equal(row.answer.end, 12);
      const before = JSON.stringify(row);
      await client.close();
      original.text = "changed";
      original.id = 1n;
      assert.equal(JSON.stringify(row), before);
      const saved = fixtureJson(before);
      assert.equal(saved.id,18446744073709551615n);
      assert.deepEqual(saved.answer,fixture.expected[0].answer);
      assert(!Object.hasOwn(saved,"item"));
      assert.equal(new Map([[row,"stored"]]).get(row),"stored");
    } finally {await client?.close(); await server.close();}
  }
});

test("human result text folds leaf controls and refuses implicit answer conversion", async () => {
  const {inspect} = await import("node:util");
  const server = await backend({replies:[{distribution:{"first\nlabel\tword":1,"second":0}}]});
  const client = new sdk.Client({url:server.url});
  try {
    const row = await client.choose("Which?","evidence", {choices:["first\nlabel\tword","second"]});
    assert.equal(row.answer,"first\nlabel\tword");
    for (const text of [String(row), `${row}`, inspect(row)]) assert.equal(text,"first label word");
    assert.equal(JSON.parse(JSON.stringify(row)).answer,row.answer);
    for (const convert of [() => +row, () => row + "", () => row == row.answer, () => row > 0]) {
      assert.throws(convert,error => error instanceof TypeError && /\.answer/.test(error.message));
    }
  } finally {await client.close(); await server.close();}
});

test("form entry names cannot masquerade as result or span fields during printing", async () => {
  const server = await backend({replies:[{probability:0.9}]});
  const client = new sdk.Client({url:server.url});
  const names = ["function","item","kind","relation","source","target","text"];
  try {
    const form = await client.annotate(Object.fromEntries(names.map(name => [name,{function:"decide",question:"Ready?"}])), "evidence");
    assert.equal(String(form), names.map(name=>`${name}=true`).join("\t"));
    assert.equal(form.answer.function.answer,true);
    assert.equal(form.item,"evidence");
  } finally {await client.close(); await server.close();}
});

test("form failures retain one affected record and use engine question-name order", async () => {
  const client = new sdk.Client({backend:"stub"});
  const record = {id:9007199254740993n, empty:"", other:"evidence"};
  try {
    for (const names of [["10","2"],["\uE000","😀"]]) {
      const questions = Object.fromEntries([...names].reverse().map(name => [name, {
        function:"decide", question:"Ready?", field:name === names[0] ? "/empty" : "/missing",
      }]));
      const form = await client.annotate(questions, record);
      assert.equal(form.status,"answered");
      assert.throws(() => form.throwIfFailed(), error => {
        assert(error instanceof sdk.FailedError && error instanceof sdk.NoAnswerError);
        assert.equal(error.code,"record.empty");
        assert.equal(error.row,form.answer[names[0]]);
        assert.deepEqual(error.rows,[form.answer[names[0]]]);
        assert.equal(error.rows[0].item,record);
        assert.equal(error.rows[0].position,form.position);
        assert.equal(error.rows[0].id,record.id);
        assert.equal(error.rows[0].source,form.source);
        assert.deepEqual(error.row.toJSON(),JSON.parse(JSON.stringify(form)).answer[names[0]]);
        return true;
      });
      assert.equal(String(form),names.map(name=>`${name}=${String(form.answer[name])}`).join("\t"));
    }
    assert.equal(client.usage.attempts,0n);
  } finally {await client.close();}
});

// Feature: every scalar entry rejects its own failed parent before exposing a result.
test("scalar parent failure is distinct from one-item bulk and has no old error alias", async () => {
  assert.equal(sdk.RecordError, undefined);
  const client = new sdk.Client({backend:"stub"});
  const calls = [
    ["decide", record => client.decide("Ready?", record)],
    ["choose", record => client.choose("Topic?", record, {choices:["billing","shipping"]})],
    ["tag", record => client.tag("Topics?", record, {labels:["billing","shipping"]})],
    ["score", record => client.score("Urgency?", record, {levels:["low","high"]})],
    ["find", record => client.find("Which line?", record)],
    ["recognize", record => client.recognize(record, {kinds:["person"]})],
    ["relate", record => client.relate(record, {rules:[{source:"person",target:"person",relation:"knows"}]})],
    ["annotate", record => client.annotate({ready:{function:"decide",question:"Ready?"}}, record)],
  ];
  try {
    for (const [name, call] of calls) {
      await assert.rejects(call(""), error => {
        assert(error instanceof sdk.FailedError && error instanceof sdk.NoAnswerError && error instanceof sdk.ThinkThenError);
        assert(!(error instanceof sdk.UnsureError));
        assert.equal(error.code,"record.empty");
        assert.equal(error.row.function,name);
        assert.equal(error.row.item,"");
        assert.equal(error.row.position,1);
        assert.deepEqual(error.rows,[error.row]);
        return true;
      });
    }
    assert.equal(client.usage.attempts,0n);
    assert.equal(client.usage.activeCalls,0n);
    const [row] = await client.decideStream("Ready?", [""]).toArray();
    assert.equal(row.status,"failed");
    assert.equal(row.error.code,"record.empty");
    assert.throws(() => row.answer,sdk.FailedError);
    assert.equal((await client.decide("Ready?","still usable")).status,"answered");
  } finally { await client.close(); }
});
