# 0584: Test the answer schema against later uses

Ticket: 0584
Date: 2026-10-14

Final review: accept. The fresh reviewer reproduced all 39 prior SQLite queries and nine targeted checks, checked both engines against independent literal expectations, verified 45 prior artifact hashes, and confirmed the missing-second-model, missing-label and missing-answer cases. The record preserves the evidence needed after scratch cleanup. The two earlier findings and their repairs appear below.

ThinkThen revision: `0e46dc2daf53bba7ad40165e67068948607f7bec`. Marketing source repository HEAD: `e1bb1955cb83aa072d49e88c9cd4dc14ae5d538a`. Earlier experiment source repository HEAD: `53d08b270070dda604760ee0ddb2562862afd820`. The manifest pins the actual source bytes separately because repository HEAD does not establish whether a design document has local edits. This experiment changed no repository file, product code, service or cache. It used Beelink, invented examples and zero model calls. The named risk was a schema that cannot answer the eleven eval uses or notebook reads planned beyond 0.3.

The extended sketch answers 39 independently expected queries on SQLite and PostgreSQL after the targeted review repair recorded below. The initial 26-query coverage excluded queue construction and direct loose-reuse lookup. SQLite uses STRICT tables. The unextended sketch lacks information needed for several uses. The database ticket should specify the relationships below before it fixes its migration and published views. This finding concerns the relational shape, not production readiness.

## What was tested

`baseline.sql` preserves an executable interpretation of PRD 01's table list and PRD 02's detailed columns. Neither PRD supplies executable DDL. The baseline is this experiment's interpretation, including scalar observation and replacement references; those particular choices are not claims that the PRDs mandate them. Actor is named `actor` internally to avoid SQL's `user` name. The public view remains `users`.

`changes.sql` preserves each extension separately. `proposed.sqlite.sql` and `proposed.postgres.sql` are usable complete scripts. The former maps signed counts to INTEGER, probabilities to REAL, JSON to TEXT and creates STRICT tables; the latter uses BIGINT, DOUBLE PRECISION and JSON. The logical shape and all 26 query texts are shared. `fixture.sql` holds every invented row. `queries.json` holds literal expected rows, independent from SQL execution. `queries.sql` holds the readable SQL. `results.json` has both engines' actual rows, expected rows, versions and pass counts. `run.log` has the final run's exit summary. `results-attempt1.json` preserves the first genuine failures; attempts 2 and 3 preserve intermediate passes before the final public-view and STRICT checks.

Expected examples were computed directly from the fixture: truths yes/no/yes; model A probabilities .9/.2/.4; B .8/.3/.7. At cuts .3/.5/.8, A gets 3/2/2 correct. A and B disagree only on the third item. Five models get 2/3/2/1/2 correct, respectively. Wording 2 gets all three. A frozen version still pins the old yes after its replacement says no. Three retrieval grades 1/3/0 yield the saved order d2,d1,d3 and perfect top-1, reciprocal rank, R-precision and normalized discounted gain for this illustrative ordering. A second query over the same members has different grades 0/0/3. A pair and its reversed presentation both join one truth row that names d2.

## Relationships and fields the uses force

| Use that exposes the gap | Change in the extended sketch | Why it matters |
| --- | --- | --- |
| One rank answer depends on three raw observations | `answer_observation(answer_id, position, observation_id)` | A scalar pointer loses observations. The PRD already says plural observations; it needs the joining relation specified. |
| Several calls return the same deterministic answer ID | `call_answer(call_id, position, answer_id)` | One answer-to-call pointer cannot represent both calls. This relation is needed only if call history promises to keep repeats; see the policy conflict below. |
| One settle resolves two conflicting actors | `right_answer_replaces(right_answer_id, replaced_id)` | One `replaces` value cannot name both live rows. The old rows stay present. |
| A retrieval query changes while the member stays the same | `right_answer.query_digest` | PRD 02 Q2 explicitly requires query inputs as well as question and member. The short truth-key rule omits them. This field is a proposed representation of that existing requirement. |
| A pair is presented in both orders | `evidence.truth_digest` | Exact reuse keeps the ordered evidence identity. A right answer can use the canonical pair subject and chosen member ID. PRD 01 Q8 already requires this distinction. |
| A notebook reads a frozen benchmark or sends picks to the CLI | Public `evals`, `eval_answers`, `picks`, `evidence_members`, `evidence_items`, `contexts`, `answer_observations`, `call_answers` and replacement views | The original six view names do not state how notebooks recover frozen setup membership, queue handles, ordered members or provenance. These are one explicit contract option; adding equivalent fields to the original views could serve the same uses. |
| Plain SQL reproduces newest/no-store lookup | Public `observations`, including markers | An answers view that joins only returned answer IDs hides markers. No-store has no probabilities and must still block older rows. |
| A historical benchmark reports money, after prices change | Nullable `observation.cost_microusd` | Tokens measure usage. They do not establish a historical price. Store reported cost or a price snapshot; unknown must remain unknown. The tested integer is an invented money amount, not a provider charge. |

The answer view has one row per answer-observation link, not one row per caller answer. Its grain must be documented. Counts of answers and money need distinct answer or observation IDs when joins fan out. Benchmark fixtures use one observation per item, so the benchmark sums here do not establish the general accounting query for composed answers. Batch wall time may also belong to one request rather than every observation; this experiment does not settle latency allocation.

## Every eval use

| evals.md use | Query numbers | Observed result |
| --- | --- | --- |
| Save money | 01–03, 32–37 | Latest answering time wins; insertion order breaks a tie; late import stays old; no-store marker blocks; loose reuse retains the answering model. |
| Pick a threshold | 04 | Correct counts 3, 2, 2 at cuts .3, .5, .8. |
| Compare runs | 05, 26 | Third item changes no to yes; later refreshes do not change the frozen comparison. |
| Benchmark | 06 | Five setup scores, tokens, invented cost and latency match the literal expected rows. |
| Curate disagreements | 07, 18–23, 27–31, 38–39 | Probability gap .3; replacement resolves two actors; unresolved values remain visible; cannot-tell stays separate; agent audit input pairs and random stratum are preserved. |
| Optimize wording | 08, 24 | Versions 1 and 2 score 2 and 3 against one key; held-read count remains available. |
| Order quality | 09, 10, 25 | The stored permutation joins query-scoped grades; top-1, reciprocal rank, R-precision and normalized gain compute in shared SQL. |
| Pairwise and tournament ranking | 11 | Both presentation orders join one winning-member truth. The schema stores pair edges; no tournament algorithm was tested or endorsed. |
| Watch over time | 12 | Four A-v1 observations remain, ordered at 50, 100, 200, 200. |
| Real outcomes | 13 | Outcome no joins the original acted answer yes without rewriting it. |
| Fine-tune | 14, 15, 22 | Tune export preserves pinned truth, source text, basis and stratum; a second key version does not alter the first. |

Extra checks preserve all three observation links (16), two calls returning one answer ID (17), stable stored part labels across key versions (15), and notebook-to-CLI pick handles (23). The SQL corpus reads published views only after the schema extension.

## Policy details the experiment cannot decide

PRD 01 Q16 proposes writing no call for a pure repeat. Full repeat-call history conflicts with that choice. The `call_answer` relation demonstrates the information needed if that history is required; it is not a recommendation to silently change the no-write rule. A database ticket must state which call history is promised. Frozen eval membership works without recording every repeat call.

The short truth key excludes context, but retrieval Q2 includes caller query inputs. Query-specific truth must include those inputs. Authored help context and model context recipes should still stay out of truth identity. `query_digest` demonstrates that separation; exact field naming and derivation remain design work.

The schema permits pair truth subjects and grade rows. The ranking experiment owns whether pairwise judgments and tournament methods work. No new ranking behavior is admitted here.

A public SQL contract needs stable field names, types, null behavior and row grain. The proposed extra views are one simple option, not an approved final API. Empty key versions, zero-member evals and input-source cluster metadata deserve explicit treatment in the final view contract. A member-only key view omits empty versions.

## Portability findings

The first shared SQL run passed 23/24 on SQLite and 20/24 on PostgreSQL. One threshold query forgot to join its cuts relation; fixing the query corrected both engines. PostgreSQL rejected two-argument ROUND on a floating-point value, so the shared query now casts to NUMERIC before ROUND.

More significantly, PostgreSQL freezes a `SELECT *` view's columns when the view is created. SQLite expands them later. Adding query_digest therefore made SQLite's right_answers expose it while PostgreSQL's right_answers did not. The migration explicitly recreates the view after the column addition. The same rule applies to adding cost fields. Final product migrations should list public columns explicitly.

JSON `->>` also returns different types for booleans: SQLite returned integer 0 for false, PostgreSQL text false. Query 21 normalizes explicitly. Numeric probabilities are explicitly cast before arithmetic. Both engines support the tested integer subscripts into saved JSON order arrays. Shared SQL still needs deliberate type normalization; saying both support `->>` is insufficient.

Final result: 26/26 on each engine. SQLite integrity_check returned ok and foreign_key_check returned no rows. PostgreSQL enforced declared foreign keys during loading. Four structural probes against the baseline failed as expected for missing public eval membership, observation-link view, query identity and canonical pair truth.

## Earlier evidence and limitations

The experiment read prior experiment 0014's README and core/curation schema. Its fixes already demonstrate that raw answers must preserve differing observations, decision rows need wording links, evidence-provided candidates matter, a true question identity cannot be an authored display name, and joins need honest grain. This experiment reuses those lessons instead of repeating its scale runs. PRD 01's earlier million-row size script was deleted; its printed size is design context, not reproduced evidence here.

The input manifest names and hashes the design sources and old schema files. Only invented evidence and values were loaded. No paid calls, external publishing, production data, licensed datasets or 0.2 work artifacts were used. Docker initially failed inside the sandbox; scoped escalation launched only the labelled, isolated PostgreSQL 16 container. The container has no network or persistent volume, a half-CPU cap and a 256 MB memory cap. Existing services remain unchanged.

These are tiny relational counterexamples. They do not prove throughput, concurrency, ten-million-row lookup performance, complete reuse rules, provider no-store behavior, real migration/import correctness, UUID/digest generation, cryptographic IDs, write authorization, frozen-set enforcement, automatic trust/dispute policy, deterministic hash splitting, every typed function's decoding, statistical coverage, training sampling, GEPA efficacy or the ranking algorithm. The SQL keeps stored split labels and held-read counts; it does not test the application code that creates or increments them. Three agent audit pairs are inputs for an agreement report, not a valid 30-case audit or an acceptance of agent-only keys. Views here grant readability by convention; database permissions were not qualified.

The experiment resolves the named relational risk by identifying required relationships and demonstrating shared queries. Recommendation: carry these requirements into the database ticket and define the public read contract before implementation. Keep the initial sketch and query counterexamples as evidence. Do not treat this script as a production migration or adopt its synthetic IDs.

## Related source finding and resource release

The coordinator supplied experiment 0586’s source-level finding: authored wording, evidence and context digests do not replace the exact rendered wire identity. Recognition may depend on earlier spans and relation questions on earlier entity IDs and candidate menus; the renderer is part of what produced the bytes. This experiment retained the exact_key field, but did not exercise those rendering dependencies or imported exact-only fallback. The database ticket should preserve exact wire identity and allow missing semantic parts to remain exact-only; 0586 owns the evidence for that requirement. No additional schema redesign was attempted here.

The owned PostgreSQL container was stopped after postgres.dump.sql preserved its complete schema and rows. Its --rm lifecycle removed the container and temporary in-memory database. No experiment evidence files were deleted. Four SQLite attempt databases, SQL scripts, raw results and the PostgreSQL dump remain for coordinator review. They are small and owned by the open ticket. No repository build or cache was created.

## Evidence retention

The experiment folder is `experiments/480-thinkthen-schema-0584`. The coordinator reviewed its raw results before recording this finding. After acceptance and landing, owned generated inputs, databases, packages and scratch can be removed. The observed results and stated limits remain in this record.

## Executable schema and query counterexamples

These are experiment evidence, not product migrations. Synthetic identifiers and fixture policy have the limits stated above.

### baseline.sql

```sql
-- Executable interpretation of draft PRD 01 plus PRD 02 working-file columns.
-- JSON_TYPE is rendered TEXT for SQLite and JSON for PostgreSQL.
-- IDs are readable synthetic stand-ins; this checks relational semantics, not UUID/hash generation.
CREATE TABLE question(id TEXT PRIMARY KEY, function TEXT NOT NULL, contract JSON_TYPE NOT NULL);
CREATE TABLE wording(digest TEXT PRIMARY KEY, question_id TEXT REFERENCES question(id), number INTEGER, text TEXT NOT NULL);
CREATE TABLE evidence(digest TEXT PRIMARY KEY, kind TEXT NOT NULL, text TEXT);
CREATE TABLE evidence_member(evidence_digest TEXT REFERENCES evidence(digest), position INTEGER, member_digest TEXT REFERENCES evidence(digest), PRIMARY KEY(evidence_digest,position));
CREATE TABLE context(digest TEXT PRIMARY KEY, text TEXT);
CREATE TABLE actor(id TEXT PRIMARY KEY, name TEXT, kind TEXT);
CREATE TABLE call(id TEXT PRIMARY KEY, function TEXT, surface TEXT, tool TEXT, step TEXT, user_id TEXT REFERENCES actor(id), agent_id TEXT REFERENCES actor(id), proxy_rule_version TEXT);
CREATE TABLE observation(id TEXT PRIMARY KEY, exact_key TEXT, adapter TEXT, address TEXT, model TEXT, answering_model TEXT, wording_digest TEXT REFERENCES wording(digest), evidence_digest TEXT REFERENCES evidence(digest), context_digest TEXT REFERENCES context(digest), probabilities JSON_TYPE, input_tokens BIGINT, output_tokens BIGINT, latency_ms BIGINT, batch_size INTEGER, answered_at BIGINT, insertion_order BIGINT UNIQUE, origin TEXT, call_id TEXT REFERENCES call(id), no_store INTEGER NOT NULL DEFAULT 0);
CREATE INDEX observation_exact ON observation(exact_key,answered_at,insertion_order);
CREATE INDEX observation_parts ON observation(evidence_digest,wording_digest,context_digest);
CREATE TABLE answer(id TEXT PRIMARY KEY, reading JSON_TYPE, threshold DOUBLE PRECISION, observation_id TEXT REFERENCES observation(id), call_id TEXT REFERENCES call(id), reuse_rule TEXT);
CREATE TABLE pick(id TEXT PRIMARY KEY, question_id TEXT REFERENCES question(id), evidence_digest TEXT REFERENCES evidence(digest), rule TEXT, stratum TEXT, setups JSON_TYPE, user_id TEXT REFERENCES actor(id), agent_id TEXT REFERENCES actor(id), picked_at BIGINT);
CREATE TABLE right_answer(id TEXT PRIMARY KEY, question_id TEXT REFERENCES question(id), evidence_digest TEXT REFERENCES evidence(digest), value TEXT, cannot_tell TEXT, basis TEXT, user_id TEXT REFERENCES actor(id), agent_id TEXT REFERENCES actor(id), pick_id TEXT REFERENCES pick(id), stratum TEXT, seen JSON_TYPE, reason TEXT, quote TEXT, replaces TEXT REFERENCES right_answer(id), settled_at BIGINT);
CREATE TABLE answer_key(id TEXT PRIMARY KEY, name TEXT UNIQUE);
CREATE TABLE answer_key_version(id TEXT PRIMARY KEY, key_id TEXT REFERENCES answer_key(id), number INTEGER, question_id TEXT REFERENCES question(id), contract_digest TEXT, trust_rule TEXT, held_share DOUBLE PRECISION, user_id TEXT REFERENCES actor(id), created_at BIGINT, UNIQUE(key_id,number));
CREATE TABLE answer_key_member(id TEXT PRIMARY KEY, version_id TEXT REFERENCES answer_key_version(id), right_answer_id TEXT REFERENCES right_answer(id), part TEXT, stratum TEXT);
CREATE TABLE outcome(id TEXT PRIMARY KEY, answer_id TEXT REFERENCES answer(id), label TEXT, value TEXT, note TEXT, user_id TEXT REFERENCES actor(id), agent_id TEXT REFERENCES actor(id), occurred_at BIGINT);
CREATE TABLE eval(id TEXT PRIMARY KEY, name TEXT, version_id TEXT REFERENCES answer_key_version(id), setups JSON_TYPE, reuse_count INTEGER, held_reads INTEGER, user_id TEXT REFERENCES actor(id), agent_id TEXT REFERENCES actor(id), created_at BIGINT);
CREATE TABLE eval_answer(eval_id TEXT REFERENCES eval(id), member_id TEXT REFERENCES answer_key_member(id), setup TEXT, answer_id TEXT REFERENCES answer(id), PRIMARY KEY(eval_id,member_id,setup));
CREATE VIEW answers AS SELECT a.id AS answer_id,a.reading,a.threshold,a.reuse_rule,o.* ,w.question_id,w.number AS wording_number,w.text AS wording_text,e.text AS evidence_text,c.text AS context_text FROM answer a JOIN observation o ON o.id=a.observation_id LEFT JOIN wording w ON w.digest=o.wording_digest LEFT JOIN evidence e ON e.digest=o.evidence_digest LEFT JOIN context c ON c.digest=o.context_digest;
CREATE VIEW right_answers AS SELECT * FROM right_answer;
CREATE VIEW answer_keys AS SELECT m.*,v.key_id,v.number,v.question_id,v.contract_digest,v.trust_rule,k.name FROM answer_key_member m JOIN answer_key_version v ON v.id=m.version_id JOIN answer_key k ON k.id=v.key_id;
CREATE VIEW calls AS SELECT * FROM call;
CREATE VIEW outcomes AS SELECT * FROM outcome;
CREATE VIEW users AS SELECT * FROM actor;

```

### changes.sql

```sql
-- Query-forced extensions to the literal baseline. See REPORT.md for source/conflicts.
-- An answer can combine N observations; scalar observation_id remains only for baseline comparison.
CREATE TABLE answer_observation(answer_id TEXT REFERENCES answer(id), position INTEGER, observation_id TEXT REFERENCES observation(id), PRIMARY KEY(answer_id,position));
-- One deterministic answer ID may be returned in multiple calls. Call provenance is a relation.
CREATE TABLE call_answer(call_id TEXT REFERENCES call(id), position INTEGER, answer_id TEXT REFERENCES answer(id), PRIMARY KEY(call_id,position));
-- A dispute resolution can replace multiple live judgments.
CREATE TABLE right_answer_replaces(right_answer_id TEXT REFERENCES right_answer(id), replaced_id TEXT REFERENCES right_answer(id), PRIMARY KEY(right_answer_id,replaced_id));
-- Retrieval grades depend on caller query inputs; fixed context is not the judged member.
ALTER TABLE right_answer ADD COLUMN query_digest TEXT REFERENCES context(digest);
-- Opposite pair presentation orders share one semantic truth subject without sharing cache identity.
ALTER TABLE evidence ADD COLUMN truth_digest TEXT REFERENCES evidence(digest);
-- Expose frozen membership, queue provenance and composite-member structure to notebook SQL.
CREATE VIEW evals AS SELECT * FROM eval;
CREATE VIEW eval_answers AS SELECT * FROM eval_answer;
CREATE VIEW picks AS SELECT * FROM pick;
CREATE VIEW evidence_members AS SELECT * FROM evidence_member;
CREATE VIEW answer_observations AS SELECT * FROM answer_observation;
CREATE VIEW call_answers AS SELECT * FROM call_answer;
CREATE VIEW right_answer_replacements AS SELECT * FROM right_answer_replaces;
CREATE VIEW evidence_items AS SELECT * FROM evidence;
CREATE VIEW contexts AS SELECT * FROM context;
DROP VIEW answers;
CREATE VIEW answers AS SELECT a.id AS answer_id,a.reading,a.threshold,a.reuse_rule,ao.position AS observation_position,o.*,w.question_id,w.number AS wording_number,w.text AS wording_text,e.text AS evidence_text,e.truth_digest,c.text AS context_text FROM answer a JOIN answer_observation ao ON ao.answer_id=a.id JOIN observation o ON o.id=ao.observation_id LEFT JOIN wording w ON w.digest=o.wording_digest LEFT JOIN evidence e ON e.digest=o.evidence_digest LEFT JOIN context c ON c.digest=o.context_digest;
CREATE VIEW live_right_answers AS SELECT r.* FROM right_answer r WHERE NOT EXISTS (SELECT 1 FROM right_answer_replaces x WHERE x.replaced_id=r.id);
-- Tokens alone cannot reconstruct historic money after a provider price changes.
ALTER TABLE observation ADD COLUMN cost_microusd BIGINT;
-- Ordering SQL needs the saved model permutation, not the evidence presentation order.
-- It stays inside reading JSON; integer ->> subscripts work on both tested engines.

-- PostgreSQL freezes SELECT * when a view is created; SQLite expands dynamically.
-- Recreate after adding query_digest so both public contracts expose it.
DROP VIEW right_answers;
CREATE VIEW right_answers AS SELECT * FROM right_answer;

CREATE VIEW observations AS SELECT * FROM observation;
-- Refresh flattened observations after adding cost_microusd.
DROP VIEW answers;
CREATE VIEW answers AS SELECT a.id AS answer_id,a.reading,a.threshold,a.reuse_rule,ao.position AS observation_position,o.*,w.question_id,w.number AS wording_number,w.text AS wording_text,e.text AS evidence_text,e.truth_digest,c.text AS context_text FROM answer a JOIN answer_observation ao ON ao.answer_id=a.id JOIN observation o ON o.id=ao.observation_id LEFT JOIN wording w ON w.digest=o.wording_digest LEFT JOIN evidence e ON e.digest=o.evidence_digest LEFT JOIN context c ON c.digest=o.context_digest;

```

### fixture.sql

```sql
INSERT INTO actor (id,name,kind) VALUES ('person','person','person');
INSERT INTO actor (id,name,kind) VALUES ('other','other','person');
INSERT INTO actor (id,name,kind) VALUES ('agent','agent','agent');
INSERT INTO question (id,function,contract) VALUES ('q','decide','{"labels":["yes","no"]}');
INSERT INTO question (id,function,contract) VALUES ('retrieve','find','{"grade":[0,3]}');
INSERT INTO question (id,function,contract) VALUES ('pairq','find','{"grade":[0,3]}');
INSERT INTO wording (digest,question_id,number,text) VALUES ('w1','q',1,'Is a refund requested?');
INSERT INTO wording (digest,question_id,number,text) VALUES ('w2','q',2,'Does this ask for money back?');
INSERT INTO wording (digest,question_id,number,text) VALUES ('wr','retrieve',1,'Which member fits the query?');
INSERT INTO wording (digest,question_id,number,text) VALUES ('wp','pairq',1,'Which candidate fits?');
INSERT INTO evidence (digest,kind,text,truth_digest) VALUES ('e1','single','Refund please','e1');
INSERT INTO evidence (digest,kind,text,truth_digest) VALUES ('e2','single','Hello','e2');
INSERT INTO evidence (digest,kind,text,truth_digest) VALUES ('e3','single','Refund status?','e3');
INSERT INTO evidence (digest,kind,text,truth_digest) VALUES ('e4','single','Unclear','e4');
INSERT INTO evidence (digest,kind,text,truth_digest) VALUES ('e5','single','Duplicate?','e5');
INSERT INTO evidence (digest,kind,text,truth_digest) VALUES ('d1','single','document one','d1');
INSERT INTO evidence (digest,kind,text,truth_digest) VALUES ('d2','single','document two','d2');
INSERT INTO evidence (digest,kind,text,truth_digest) VALUES ('d3','single','document three','d3');
INSERT INTO evidence (digest,kind,text,truth_digest) VALUES ('set','set',NULL,'set');
INSERT INTO evidence (digest,kind,text,truth_digest) VALUES ('pair12','pair',NULL,'pair12');
INSERT INTO evidence (digest,kind,text,truth_digest) VALUES ('pair21','pair',NULL,'pair12');
INSERT INTO evidence_member (evidence_digest,position,member_digest) VALUES ('set',1,'d1');
INSERT INTO evidence_member (evidence_digest,position,member_digest) VALUES ('set',2,'d2');
INSERT INTO evidence_member (evidence_digest,position,member_digest) VALUES ('set',3,'d3');
INSERT INTO evidence_member (evidence_digest,position,member_digest) VALUES ('pair12',1,'d1');
INSERT INTO evidence_member (evidence_digest,position,member_digest) VALUES ('pair12',2,'d2');
INSERT INTO evidence_member (evidence_digest,position,member_digest) VALUES ('pair21',1,'d2');
INSERT INTO evidence_member (evidence_digest,position,member_digest) VALUES ('pair21',2,'d1');
INSERT INTO context (digest,text) VALUES ('c','No added context');
INSERT INTO context (digest,text) VALUES ('queryA','Find a refund policy');
INSERT INTO context (digest,text) VALUES ('queryB','Find a greetings policy');
INSERT INTO call (id,function,surface,tool,step,user_id,agent_id,proxy_rule_version) VALUES ('c1','decide','cli','refund','step1','person',NULL,NULL);
INSERT INTO call (id,function,surface,tool,step,user_id,agent_id,proxy_rule_version) VALUES ('c2','decide','cli','refund','step1','person',NULL,NULL);
INSERT INTO call (id,function,surface,tool,step,user_id,agent_id,proxy_rule_version) VALUES ('c3','decide','cli','refund','step1','person',NULL,NULL);
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,input_tokens,output_tokens,latency_ms,batch_size,answered_at,insertion_order,origin,call_id,cost_microusd) VALUES ('oA1','key-A-w1-e1','fixture','https://example.invalid','A','A-v1','w1','e1','c','{"yes":0.9}',10,1,100,1,100,1,'synthetic','c1',10);
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('aA1','{"value":"yes"}',0.5,'oA1','c1','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('aA1',1,'oA1');
INSERT INTO call_answer (call_id,position,answer_id) VALUES ('c1',1,'aA1');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,input_tokens,output_tokens,latency_ms,batch_size,answered_at,insertion_order,origin,call_id,cost_microusd) VALUES ('oA2','key-A-w1-e2','fixture','https://example.invalid','A','A-v1','w1','e2','c','{"yes":0.2}',10,1,100,1,100,2,'synthetic','c1',10);
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('aA2','{"value":"no"}',0.5,'oA2','c1','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('aA2',1,'oA2');
INSERT INTO call_answer (call_id,position,answer_id) VALUES ('c1',2,'aA2');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,input_tokens,output_tokens,latency_ms,batch_size,answered_at,insertion_order,origin,call_id,cost_microusd) VALUES ('oA3','key-A-w1-e3','fixture','https://example.invalid','A','A-v1','w1','e3','c','{"yes":0.4}',10,1,100,1,100,3,'synthetic','c1',10);
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('aA3','{"value":"no"}',0.5,'oA3','c1','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('aA3',1,'oA3');
INSERT INTO call_answer (call_id,position,answer_id) VALUES ('c1',3,'aA3');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,input_tokens,output_tokens,latency_ms,batch_size,answered_at,insertion_order,origin,call_id,cost_microusd) VALUES ('oB1','key-B-w1-e1','fixture','https://example.invalid','B','B-v1','w1','e1','c','{"yes":0.8}',10,1,100,1,100,4,'synthetic','c1',10);
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('aB1','{"value":"yes"}',0.5,'oB1','c1','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('aB1',1,'oB1');
INSERT INTO call_answer (call_id,position,answer_id) VALUES ('c1',4,'aB1');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,input_tokens,output_tokens,latency_ms,batch_size,answered_at,insertion_order,origin,call_id,cost_microusd) VALUES ('oB2','key-B-w1-e2','fixture','https://example.invalid','B','B-v1','w1','e2','c','{"yes":0.3}',10,1,100,1,100,5,'synthetic','c1',10);
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('aB2','{"value":"no"}',0.5,'oB2','c1','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('aB2',1,'oB2');
INSERT INTO call_answer (call_id,position,answer_id) VALUES ('c1',5,'aB2');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,input_tokens,output_tokens,latency_ms,batch_size,answered_at,insertion_order,origin,call_id,cost_microusd) VALUES ('oB3','key-B-w1-e3','fixture','https://example.invalid','B','B-v1','w1','e3','c','{"yes":0.7}',10,1,100,1,100,6,'synthetic','c1',10);
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('aB3','{"value":"yes"}',0.5,'oB3','c1','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('aB3',1,'oB3');
INSERT INTO call_answer (call_id,position,answer_id) VALUES ('c1',6,'aB3');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,input_tokens,output_tokens,latency_ms,batch_size,answered_at,insertion_order,origin,call_id,cost_microusd) VALUES ('oC1','key-C-w1-e1','fixture','https://example.invalid','C','C-v1','w1','e1','c','{"yes":0.6}',10,1,100,1,100,7,'synthetic','c1',10);
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('aC1','{"value":"yes"}',0.5,'oC1','c1','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('aC1',1,'oC1');
INSERT INTO call_answer (call_id,position,answer_id) VALUES ('c1',7,'aC1');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,input_tokens,output_tokens,latency_ms,batch_size,answered_at,insertion_order,origin,call_id,cost_microusd) VALUES ('oC2','key-C-w1-e2','fixture','https://example.invalid','C','C-v1','w1','e2','c','{"yes":0.8}',10,1,100,1,100,8,'synthetic','c1',10);
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('aC2','{"value":"yes"}',0.5,'oC2','c1','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('aC2',1,'oC2');
INSERT INTO call_answer (call_id,position,answer_id) VALUES ('c1',8,'aC2');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,input_tokens,output_tokens,latency_ms,batch_size,answered_at,insertion_order,origin,call_id,cost_microusd) VALUES ('oC3','key-C-w1-e3','fixture','https://example.invalid','C','C-v1','w1','e3','c','{"yes":0.6}',10,1,100,1,100,9,'synthetic','c1',10);
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('aC3','{"value":"yes"}',0.5,'oC3','c1','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('aC3',1,'oC3');
INSERT INTO call_answer (call_id,position,answer_id) VALUES ('c1',9,'aC3');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,input_tokens,output_tokens,latency_ms,batch_size,answered_at,insertion_order,origin,call_id,cost_microusd) VALUES ('oD1','key-D-w1-e1','fixture','https://example.invalid','D','D-v1','w1','e1','c','{"yes":0.2}',10,1,100,1,100,10,'synthetic','c1',10);
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('aD1','{"value":"no"}',0.5,'oD1','c1','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('aD1',1,'oD1');
INSERT INTO call_answer (call_id,position,answer_id) VALUES ('c1',10,'aD1');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,input_tokens,output_tokens,latency_ms,batch_size,answered_at,insertion_order,origin,call_id,cost_microusd) VALUES ('oD2','key-D-w1-e2','fixture','https://example.invalid','D','D-v1','w1','e2','c','{"yes":0.1}',10,1,100,1,100,11,'synthetic','c1',10);
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('aD2','{"value":"no"}',0.5,'oD2','c1','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('aD2',1,'oD2');
INSERT INTO call_answer (call_id,position,answer_id) VALUES ('c1',11,'aD2');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,input_tokens,output_tokens,latency_ms,batch_size,answered_at,insertion_order,origin,call_id,cost_microusd) VALUES ('oD3','key-D-w1-e3','fixture','https://example.invalid','D','D-v1','w1','e3','c','{"yes":0.3}',10,1,100,1,100,12,'synthetic','c1',10);
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('aD3','{"value":"no"}',0.5,'oD3','c1','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('aD3',1,'oD3');
INSERT INTO call_answer (call_id,position,answer_id) VALUES ('c1',12,'aD3');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,input_tokens,output_tokens,latency_ms,batch_size,answered_at,insertion_order,origin,call_id,cost_microusd) VALUES ('oE1','key-E-w1-e1','fixture','https://example.invalid','E','E-v1','w1','e1','c','{"yes":0.7}',10,1,100,1,100,13,'synthetic','c1',10);
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('aE1','{"value":"yes"}',0.5,'oE1','c1','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('aE1',1,'oE1');
INSERT INTO call_answer (call_id,position,answer_id) VALUES ('c1',13,'aE1');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,input_tokens,output_tokens,latency_ms,batch_size,answered_at,insertion_order,origin,call_id,cost_microusd) VALUES ('oE2','key-E-w1-e2','fixture','https://example.invalid','E','E-v1','w1','e2','c','{"yes":0.6}',10,1,100,1,100,14,'synthetic','c1',10);
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('aE2','{"value":"yes"}',0.5,'oE2','c1','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('aE2',1,'oE2');
INSERT INTO call_answer (call_id,position,answer_id) VALUES ('c1',14,'aE2');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,input_tokens,output_tokens,latency_ms,batch_size,answered_at,insertion_order,origin,call_id,cost_microusd) VALUES ('oE3','key-E-w1-e3','fixture','https://example.invalid','E','E-v1','w1','e3','c','{"yes":0.8}',10,1,100,1,100,15,'synthetic','c1',10);
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('aE3','{"value":"yes"}',0.5,'oE3','c1','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('aE3',1,'oE3');
INSERT INTO call_answer (call_id,position,answer_id) VALUES ('c1',15,'aE3');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,input_tokens,output_tokens,latency_ms,batch_size,answered_at,insertion_order,origin,call_id,cost_microusd) VALUES ('ow21','key-A-w2-e1','fixture','https://example.invalid','A','A-v1','w2','e1','c','{"yes":0.9}',10,1,100,1,100,16,'synthetic','c1',10);
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('aw21','{"value":"yes"}',0.5,'ow21','c1','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('aw21',1,'ow21');
INSERT INTO call_answer (call_id,position,answer_id) VALUES ('c1',16,'aw21');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,input_tokens,output_tokens,latency_ms,batch_size,answered_at,insertion_order,origin,call_id,cost_microusd) VALUES ('ow22','key-A-w2-e2','fixture','https://example.invalid','A','A-v1','w2','e2','c','{"yes":0.2}',10,1,100,1,100,17,'synthetic','c1',10);
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('aw22','{"value":"no"}',0.5,'ow22','c1','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('aw22',1,'ow22');
INSERT INTO call_answer (call_id,position,answer_id) VALUES ('c1',17,'aw22');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,input_tokens,output_tokens,latency_ms,batch_size,answered_at,insertion_order,origin,call_id,cost_microusd) VALUES ('ow23','key-A-w2-e3','fixture','https://example.invalid','A','A-v1','w2','e3','c','{"yes":0.8}',10,1,100,1,100,18,'synthetic','c1',10);
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('aw23','{"value":"yes"}',0.5,'ow23','c1','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('aw23',1,'ow23');
INSERT INTO call_answer (call_id,position,answer_id) VALUES ('c1',18,'aw23');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,input_tokens,output_tokens,latency_ms,batch_size,answered_at,insertion_order,origin,call_id,cost_microusd) VALUES ('orefresh','key-A-w1-e1','fixture','https://example.invalid','A','A-v1','w1','e1','c','{"yes":0.1}',10,1,100,1,200,19,'synthetic','c1',10);
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('aorefresh','{"value":"no"}',0.5,'orefresh','c1','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('aorefresh',1,'orefresh');
INSERT INTO call_answer (call_id,position,answer_id) VALUES ('c1',19,'aorefresh');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,input_tokens,output_tokens,latency_ms,batch_size,answered_at,insertion_order,origin,call_id,cost_microusd) VALUES ('oimport','key-A-w1-e1','fixture','https://example.invalid','A','A-v1','w1','e1','c','{"yes":0.95}',10,1,100,1,50,20,'synthetic','c1',10);
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('aoimport','{"value":"yes"}',0.5,'oimport','c1','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('aoimport',1,'oimport');
INSERT INTO call_answer (call_id,position,answer_id) VALUES ('c1',20,'aoimport');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,input_tokens,output_tokens,latency_ms,batch_size,answered_at,insertion_order,origin,call_id,cost_microusd) VALUES ('otie','key-A-w1-e1','fixture','https://example.invalid','A','A-v1','w1','e1','c','{"yes":0.85}',10,1,100,1,200,21,'synthetic','c1',10);
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('aotie','{"value":"yes"}',0.5,'otie','c1','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('aotie',1,'otie');
INSERT INTO call_answer (call_id,position,answer_id) VALUES ('c1',21,'aotie');
INSERT INTO observation (id,exact_key,model,answering_model,wording_digest,evidence_digest,context_digest,answered_at,insertion_order,no_store,origin) VALUES ('marker','key-A-w1-e2','A','A-v1','w1','e2','c',300,99,1,'synthetic');
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('aloose','{"value":"yes"}',0.5,'oA1','c2','any-model');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('aloose',1,'oA1');
INSERT INTO call_answer (call_id,position,answer_id) VALUES ('c2',22,'aloose');
INSERT INTO call_answer (call_id,position,answer_id) VALUES ('c3',1,'aA1');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,input_tokens,output_tokens,latency_ms,batch_size,answered_at,insertion_order,origin,call_id,cost_microusd) VALUES ('od1','key-A-wr-d1','fixture','https://example.invalid','A','A-v1','wr','d1','queryA','{"yes":0.4}',10,1,100,1,100,22,'synthetic','c1',10);
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,input_tokens,output_tokens,latency_ms,batch_size,answered_at,insertion_order,origin,call_id,cost_microusd) VALUES ('od2','key-A-wr-d2','fixture','https://example.invalid','A','A-v1','wr','d2','queryA','{"yes":0.9}',10,1,100,1,100,23,'synthetic','c1',10);
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,input_tokens,output_tokens,latency_ms,batch_size,answered_at,insertion_order,origin,call_id,cost_microusd) VALUES ('od3','key-A-wr-d3','fixture','https://example.invalid','A','A-v1','wr','d3','queryA','{"yes":0.1}',10,1,100,1,100,24,'synthetic','c1',10);
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('rank','{"value":"d2","order":["d2","d1","d3"]}',0.5,'od1','c1','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('rank',1,'od1');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('rank',2,'od2');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('rank',3,'od3');
INSERT INTO call_answer (call_id,position,answer_id) VALUES ('c1',24,'rank');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,input_tokens,output_tokens,latency_ms,batch_size,answered_at,insertion_order,origin,call_id,cost_microusd) VALUES ('op12','key-A-wp-pair12','fixture','https://example.invalid','A','A-v1','wp','pair12','c','{"yes":0.8}',10,1,100,1,100,25,'synthetic','c1',10);
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('aop12','{"value":"d2"}',0.5,'op12','c1','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('aop12',1,'op12');
INSERT INTO call_answer (call_id,position,answer_id) VALUES ('c1',25,'aop12');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,input_tokens,output_tokens,latency_ms,batch_size,answered_at,insertion_order,origin,call_id,cost_microusd) VALUES ('op21','key-A-wp-pair21','fixture','https://example.invalid','A','A-v1','wp','pair21','c','{"yes":0.8}',10,1,100,1,100,26,'synthetic','c1',10);
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('aop21','{"value":"d2"}',0.5,'op21','c1','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('aop21',1,'op21');
INSERT INTO call_answer (call_id,position,answer_id) VALUES ('c1',26,'aop21');
INSERT INTO pick (id,question_id,evidence_digest,rule,stratum,setups,user_id,picked_at) VALUES ('p1','q','e1','random','random','["A","B"]','person',100);
INSERT INTO pick (id,question_id,evidence_digest,rule,stratum,setups,user_id,picked_at) VALUES ('p2','q','e2','unsure','unsure','["A","B"]','person',100);
INSERT INTO pick (id,question_id,evidence_digest,rule,stratum,setups,user_id,picked_at) VALUES ('p3','q','e3','random','random','["A","B"]','person',100);
INSERT INTO right_answer (id,question_id,evidence_digest,value,cannot_tell,basis,user_id,agent_id,pick_id,stratum,seen,settled_at,query_digest) VALUES ('r1','q','e1','yes',NULL,'person','person',NULL,'p1','random','{"answers":false}',100,NULL);
INSERT INTO right_answer (id,question_id,evidence_digest,value,cannot_tell,basis,user_id,agent_id,pick_id,stratum,seen,settled_at,query_digest) VALUES ('r2','q','e2','no',NULL,'person','person',NULL,'p2','unsure','{"answers":false}',100,NULL);
INSERT INTO right_answer (id,question_id,evidence_digest,value,cannot_tell,basis,user_id,agent_id,pick_id,stratum,seen,settled_at,query_digest) VALUES ('r3','q','e3','yes',NULL,'person','person',NULL,'p3','random','{"answers":false}',100,NULL);
INSERT INTO right_answer (id,question_id,evidence_digest,value,cannot_tell,basis,user_id,agent_id,pick_id,stratum,seen,settled_at,query_digest) VALUES ('cannot','q','e4',NULL,'insufficient evidence','person','person',NULL,NULL,'random','{"answers":false}',100,NULL);
INSERT INTO right_answer (id,question_id,evidence_digest,value,cannot_tell,basis,user_id,agent_id,pick_id,stratum,seen,settled_at,query_digest) VALUES ('r1new','q','e1','no',NULL,'person','person',NULL,NULL,'random','{"answers":false}',200,NULL);
INSERT INTO right_answer_replaces (right_answer_id,replaced_id) VALUES ('r1new','r1');
INSERT INTO right_answer (id,question_id,evidence_digest,value,cannot_tell,basis,user_id,agent_id,pick_id,stratum,seen,settled_at,query_digest) VALUES ('dispute1','q','e5','yes',NULL,'person','person',NULL,NULL,'random','{"answers":false}',100,NULL);
INSERT INTO right_answer (id,question_id,evidence_digest,value,cannot_tell,basis,user_id,agent_id,pick_id,stratum,seen,settled_at,query_digest) VALUES ('dispute2','q','e5','no',NULL,'person','other',NULL,NULL,'random','{"answers":false}',100,NULL);
INSERT INTO right_answer (id,question_id,evidence_digest,value,cannot_tell,basis,user_id,agent_id,pick_id,stratum,seen,settled_at,query_digest) VALUES ('resolution','q','e5','yes',NULL,'person','person',NULL,NULL,'random','{"answers":false}',300,NULL);
INSERT INTO right_answer_replaces (right_answer_id,replaced_id) VALUES ('resolution','dispute1');
INSERT INTO right_answer_replaces (right_answer_id,replaced_id) VALUES ('resolution','dispute2');
INSERT INTO right_answer (id,question_id,evidence_digest,value,cannot_tell,basis,user_id,agent_id,pick_id,stratum,seen,settled_at,query_digest) VALUES ('unresolved1','q','e4','yes',NULL,'person','person',NULL,NULL,'random','{"answers":false}',100,NULL);
INSERT INTO right_answer (id,question_id,evidence_digest,value,cannot_tell,basis,user_id,agent_id,pick_id,stratum,seen,settled_at,query_digest) VALUES ('unresolved2','q','e4','no',NULL,'person','other',NULL,NULL,'random','{"answers":false}',100,NULL);
INSERT INTO right_answer (id,question_id,evidence_digest,value,cannot_tell,basis,user_id,agent_id,pick_id,stratum,seen,settled_at,query_digest) VALUES ('agent1','q','e1','yes',NULL,'agent','person','agent',NULL,'random','{"answers":false}',100,NULL);
INSERT INTO right_answer (id,question_id,evidence_digest,value,cannot_tell,basis,user_id,agent_id,pick_id,stratum,seen,settled_at,query_digest) VALUES ('agent2','q','e2','yes',NULL,'agent','person','agent',NULL,'random','{"answers":false}',100,NULL);
INSERT INTO right_answer (id,question_id,evidence_digest,value,cannot_tell,basis,user_id,agent_id,pick_id,stratum,seen,settled_at,query_digest) VALUES ('agent3','q','e3','no',NULL,'agent','person','agent',NULL,'random','{"answers":false}',100,NULL);
INSERT INTO right_answer (id,question_id,evidence_digest,value,cannot_tell,basis,user_id,agent_id,pick_id,stratum,seen,settled_at,query_digest) VALUES ('queryA1','retrieve','d1','1',NULL,'person','person',NULL,NULL,'random','{"answers":false}',100,'queryA');
INSERT INTO right_answer (id,question_id,evidence_digest,value,cannot_tell,basis,user_id,agent_id,pick_id,stratum,seen,settled_at,query_digest) VALUES ('queryA2','retrieve','d2','3',NULL,'person','person',NULL,NULL,'random','{"answers":false}',100,'queryA');
INSERT INTO right_answer (id,question_id,evidence_digest,value,cannot_tell,basis,user_id,agent_id,pick_id,stratum,seen,settled_at,query_digest) VALUES ('queryA3','retrieve','d3','0',NULL,'person','person',NULL,NULL,'random','{"answers":false}',100,'queryA');
INSERT INTO right_answer (id,question_id,evidence_digest,value,cannot_tell,basis,user_id,agent_id,pick_id,stratum,seen,settled_at,query_digest) VALUES ('queryB1','retrieve','d1','0',NULL,'person','person',NULL,NULL,'random','{"answers":false}',100,'queryB');
INSERT INTO right_answer (id,question_id,evidence_digest,value,cannot_tell,basis,user_id,agent_id,pick_id,stratum,seen,settled_at,query_digest) VALUES ('queryB2','retrieve','d2','0',NULL,'person','person',NULL,NULL,'random','{"answers":false}',100,'queryB');
INSERT INTO right_answer (id,question_id,evidence_digest,value,cannot_tell,basis,user_id,agent_id,pick_id,stratum,seen,settled_at,query_digest) VALUES ('queryB3','retrieve','d3','3',NULL,'person','person',NULL,NULL,'random','{"answers":false}',100,'queryB');
INSERT INTO right_answer (id,question_id,evidence_digest,value,cannot_tell,basis,user_id,agent_id,pick_id,stratum,seen,settled_at,query_digest) VALUES ('pairtruth','pairq','pair12','d2',NULL,'person','person',NULL,NULL,'random','{"answers":false}',100,NULL);
INSERT INTO answer_key (id,name) VALUES ('k','refund');
INSERT INTO answer_key_version (id,key_id,number,question_id,contract_digest,trust_rule,held_share,user_id,created_at) VALUES ('v1','k',1,'q','contract-q','person',0.3,'person',100);
INSERT INTO answer_key_member (id,version_id,right_answer_id,part,stratum) VALUES ('m11','v1','r1','tune','random');
INSERT INTO answer_key_member (id,version_id,right_answer_id,part,stratum) VALUES ('m12','v1','r2','held','unsure');
INSERT INTO answer_key_member (id,version_id,right_answer_id,part,stratum) VALUES ('m13','v1','r3','held','random');
INSERT INTO answer_key_version (id,key_id,number,question_id,contract_digest,trust_rule,held_share,user_id,created_at) VALUES ('v2','k',2,'q','contract-q','person',0.3,'person',200);
INSERT INTO answer_key_member (id,version_id,right_answer_id,part,stratum) VALUES ('m21','v2','r1new','tune','random');
INSERT INTO answer_key_member (id,version_id,right_answer_id,part,stratum) VALUES ('m22','v2','r2','held','unsure');
INSERT INTO answer_key_member (id,version_id,right_answer_id,part,stratum) VALUES ('m23','v2','r3','held','random');
INSERT INTO eval (id,name,version_id,setups,reuse_count,held_reads,user_id,created_at) VALUES ('bench','five-model','v1','{"A":{"model":"A","answering_model":"A-v1","wording":"w1","context":"c","threshold":0.5,"batch_size":1,"address":"https://example.invalid"},"B":{"model":"B","answering_model":"B-v1","wording":"w1","context":"c","threshold":0.5,"batch_size":1,"address":"https://example.invalid"},"C":{"model":"C","answering_model":"C-v1","wording":"w1","context":"c","threshold":0.5,"batch_size":1,"address":"https://example.invalid"},"D":{"model":"D","answering_model":"D-v1","wording":"w1","context":"c","threshold":0.5,"batch_size":1,"address":"https://example.invalid"},"E":{"model":"E","answering_model":"E-v1","wording":"w1","context":"c","threshold":0.5,"batch_size":1,"address":"https://example.invalid"},"A2":{"model":"A","answering_model":"A-v1","wording":"w2","context":"c","threshold":0.5,"batch_size":1,"address":"https://example.invalid"}}',0,1,'person',100);
INSERT INTO eval_answer (eval_id,member_id,setup,answer_id) VALUES ('bench','m11','A','aA1');
INSERT INTO eval_answer (eval_id,member_id,setup,answer_id) VALUES ('bench','m12','A','aA2');
INSERT INTO eval_answer (eval_id,member_id,setup,answer_id) VALUES ('bench','m13','A','aA3');
INSERT INTO eval_answer (eval_id,member_id,setup,answer_id) VALUES ('bench','m11','B','aB1');
INSERT INTO eval_answer (eval_id,member_id,setup,answer_id) VALUES ('bench','m12','B','aB2');
INSERT INTO eval_answer (eval_id,member_id,setup,answer_id) VALUES ('bench','m13','B','aB3');
INSERT INTO eval_answer (eval_id,member_id,setup,answer_id) VALUES ('bench','m11','C','aC1');
INSERT INTO eval_answer (eval_id,member_id,setup,answer_id) VALUES ('bench','m12','C','aC2');
INSERT INTO eval_answer (eval_id,member_id,setup,answer_id) VALUES ('bench','m13','C','aC3');
INSERT INTO eval_answer (eval_id,member_id,setup,answer_id) VALUES ('bench','m11','D','aD1');
INSERT INTO eval_answer (eval_id,member_id,setup,answer_id) VALUES ('bench','m12','D','aD2');
INSERT INTO eval_answer (eval_id,member_id,setup,answer_id) VALUES ('bench','m13','D','aD3');
INSERT INTO eval_answer (eval_id,member_id,setup,answer_id) VALUES ('bench','m11','E','aE1');
INSERT INTO eval_answer (eval_id,member_id,setup,answer_id) VALUES ('bench','m12','E','aE2');
INSERT INTO eval_answer (eval_id,member_id,setup,answer_id) VALUES ('bench','m13','E','aE3');
INSERT INTO eval_answer (eval_id,member_id,setup,answer_id) VALUES ('bench','m11','A2','aw21');
INSERT INTO eval_answer (eval_id,member_id,setup,answer_id) VALUES ('bench','m12','A2','aw22');
INSERT INTO eval_answer (eval_id,member_id,setup,answer_id) VALUES ('bench','m13','A2','aw23');
INSERT INTO outcome (id,answer_id,label,value,note,user_id,occurred_at) VALUES ('out1','aA1','review-corrected','no','Synthetic downstream conflict','person',200);

```

### queries.json

```json
[
  {
    "name": "01_exact_newest_tie_and_late_import",
    "sql": "SELECT id FROM answers WHERE exact_key='key-A-w1-e1' AND reuse_rule='exact' ORDER BY answered_at DESC,insertion_order DESC LIMIT 1",
    "expected": [
      [
        "otie"
      ]
    ]
  },
  {
    "name": "02_no_store_blocks_older",
    "sql": "WITH ranked AS (SELECT id,no_store,ROW_NUMBER() OVER (ORDER BY answered_at DESC,insertion_order DESC) AS n FROM observations WHERE exact_key='key-A-w1-e2') SELECT id,no_store FROM ranked WHERE n=1",
    "expected": [
      [
        "marker",
        1
      ]
    ]
  },
  {
    "name": "03_loose_reuse_preserves_answering_model",
    "sql": "SELECT answer_id,model,answering_model FROM answers WHERE answer_id='aloose'",
    "expected": [
      [
        "aloose",
        "A",
        "A-v1"
      ]
    ]
  },
  {
    "name": "04_threshold_sweep",
    "sql": "WITH cuts(cut) AS (VALUES (0.3),(0.5),(0.8)) SELECT cut,SUM(CASE WHEN (CASE WHEN CAST(a.probabilities->>'yes' AS DOUBLE PRECISION)>=cut THEN 'yes' ELSE 'no' END)=r.value THEN 1 ELSE 0 END) FROM eval_answers ea JOIN answers a ON a.answer_id=ea.answer_id JOIN answer_keys k ON k.id=ea.member_id JOIN right_answers r ON r.id=k.right_answer_id CROSS JOIN cuts WHERE ea.eval_id='bench' AND ea.setup='A' GROUP BY cut ORDER BY cut",
    "expected": [
      [
        0.3,
        3
      ],
      [
        0.5,
        2
      ],
      [
        0.8,
        2
      ]
    ]
  },
  {
    "name": "05_compare_pinned_runs",
    "sql": "SELECT aa.evidence_digest,aa.reading->>'value',bb.reading->>'value',r.value FROM eval_answers x JOIN eval_answers y ON y.eval_id=x.eval_id AND y.member_id=x.member_id AND y.setup='B' JOIN answers aa ON aa.answer_id=x.answer_id JOIN answers bb ON bb.answer_id=y.answer_id JOIN answer_keys k ON k.id=x.member_id JOIN right_answers r ON r.id=k.right_answer_id WHERE x.setup='A' AND x.eval_id='bench' AND aa.reading->>'value'<>bb.reading->>'value' ORDER BY aa.evidence_digest",
    "expected": [
      [
        "e3",
        "no",
        "yes",
        "yes"
      ]
    ]
  },
  {
    "name": "06_five_models_accuracy_tokens_speed",
    "sql": "SELECT ea.setup,COUNT(*),SUM(CASE WHEN a.reading->>'value'=r.value THEN 1 ELSE 0 END),SUM(a.input_tokens+a.output_tokens),SUM(a.cost_microusd),AVG(a.latency_ms) FROM eval_answers ea JOIN answers a ON a.answer_id=ea.answer_id JOIN answer_keys k ON k.id=ea.member_id JOIN right_answers r ON r.id=k.right_answer_id WHERE ea.eval_id='bench' AND ea.setup<>'A2' GROUP BY ea.setup ORDER BY ea.setup",
    "expected": [
      [
        "A",
        3,
        2,
        33,
        30,
        100
      ],
      [
        "B",
        3,
        3,
        33,
        30,
        100
      ],
      [
        "C",
        3,
        2,
        33,
        30,
        100
      ],
      [
        "D",
        3,
        1,
        33,
        30,
        100
      ],
      [
        "E",
        3,
        2,
        33,
        30,
        100
      ]
    ]
  },
  {
    "name": "07_disagreement_gap",
    "sql": "SELECT a.evidence_digest,ROUND(CAST(ABS(CAST(a.probabilities->>'yes' AS DOUBLE PRECISION)-CAST(b.probabilities->>'yes' AS DOUBLE PRECISION)) AS NUMERIC),2) FROM answers a JOIN answers b ON a.evidence_digest=b.evidence_digest WHERE a.answer_id='aA3' AND b.answer_id='aB3'",
    "expected": [
      [
        "e3",
        0.3
      ]
    ]
  },
  {
    "name": "08_wording_comparison_same_key",
    "sql": "SELECT a.wording_number,SUM(CASE WHEN a.reading->>'value'=r.value THEN 1 ELSE 0 END) FROM eval_answers ea JOIN answers a ON a.answer_id=ea.answer_id JOIN answer_keys k ON k.id=ea.member_id JOIN right_answers r ON r.id=k.right_answer_id WHERE ea.eval_id='bench' AND ea.setup IN ('A','A2') GROUP BY a.wording_number ORDER BY a.wording_number",
    "expected": [
      [
        1,
        2
      ],
      [
        2,
        3
      ]
    ]
  },
  {
    "name": "09_order_metrics_inputs_query_scoped",
    "sql": "WITH perm AS (SELECT DISTINCT a.answer_id,m.position,a.reading->'order'->>(m.position-1) AS member FROM answers a JOIN evidence_members m ON m.evidence_digest='set' WHERE a.answer_id='rank') SELECT p.position,p.member,r.value FROM perm p JOIN right_answers r ON r.evidence_digest=p.member AND r.question_id='retrieve' AND r.query_digest='queryA' ORDER BY p.position",
    "expected": [
      [
        1,
        "d2",
        "3"
      ],
      [
        2,
        "d1",
        "1"
      ],
      [
        3,
        "d3",
        "0"
      ]
    ]
  },
  {
    "name": "10_query_B_different_truth",
    "sql": "SELECT evidence_digest,value FROM right_answers WHERE question_id='retrieve' AND query_digest='queryB' ORDER BY evidence_digest",
    "expected": [
      [
        "d1",
        "0"
      ],
      [
        "d2",
        "0"
      ],
      [
        "d3",
        "3"
      ]
    ]
  },
  {
    "name": "11_pair_presentation_independent_truth",
    "sql": "SELECT a.evidence_digest,a.reading->>'value',r.value FROM answers a JOIN right_answers r ON r.evidence_digest=a.truth_digest AND r.question_id=a.question_id WHERE a.answer_id IN ('aop12','aop21') ORDER BY a.evidence_digest",
    "expected": [
      [
        "pair12",
        "d2",
        "d2"
      ],
      [
        "pair21",
        "d2",
        "d2"
      ]
    ]
  },
  {
    "name": "12_history_kept_with_versions",
    "sql": "SELECT id,answered_at,answering_model,probabilities->>'yes' FROM answers WHERE exact_key='key-A-w1-e1' AND reuse_rule='exact' ORDER BY answered_at,insertion_order",
    "expected": [
      [
        "oimport",
        50,
        "A-v1",
        0.95
      ],
      [
        "oA1",
        100,
        "A-v1",
        0.9
      ],
      [
        "orefresh",
        200,
        "A-v1",
        0.1
      ],
      [
        "otie",
        200,
        "A-v1",
        0.85
      ]
    ]
  },
  {
    "name": "13_outcome_retains_acted_answer",
    "sql": "SELECT o.answer_id,a.reading->>'value',o.value FROM outcomes o JOIN answers a ON a.answer_id=o.answer_id",
    "expected": [
      [
        "aA1",
        "yes",
        "no"
      ]
    ]
  },
  {
    "name": "14_training_export_tune_frozen",
    "sql": "SELECT k.name,k.number,k.part,e.text,r.value,k.stratum,r.basis FROM answer_keys k JOIN right_answers r ON r.id=k.right_answer_id JOIN evidence_items e ON e.digest=r.evidence_digest WHERE k.version_id='v1' AND k.part='tune' ORDER BY e.digest",
    "expected": [
      [
        "refund",
        1,
        "tune",
        "Refund please",
        "yes",
        "random",
        "person"
      ]
    ]
  },
  {
    "name": "15_key_versions_pin_right_answer",
    "sql": "SELECT k.number,r.id,r.value,k.part FROM answer_keys k JOIN right_answers r ON r.id=k.right_answer_id WHERE r.evidence_digest='e1' ORDER BY k.number",
    "expected": [
      [
        1,
        "r1",
        "yes",
        "tune"
      ],
      [
        2,
        "r1new",
        "no",
        "tune"
      ]
    ]
  },
  {
    "name": "16_multi_observation_no_loss",
    "sql": "SELECT answer_id,observation_position,id FROM answers WHERE answer_id='rank' ORDER BY observation_position",
    "expected": [
      [
        "rank",
        1,
        "od1"
      ],
      [
        "rank",
        2,
        "od2"
      ],
      [
        "rank",
        3,
        "od3"
      ]
    ]
  },
  {
    "name": "17_repeat_call_same_answer",
    "sql": "SELECT call_id,answer_id FROM call_answers WHERE answer_id='aA1' ORDER BY call_id",
    "expected": [
      [
        "c1",
        "aA1"
      ],
      [
        "c3",
        "aA1"
      ]
    ]
  },
  {
    "name": "18_dispute_replaces_multiple",
    "sql": "SELECT id,value FROM live_right_answers WHERE evidence_digest='e5' ORDER BY id",
    "expected": [
      [
        "resolution",
        "yes"
      ]
    ]
  },
  {
    "name": "19_unresolved_dispute_not_latest_wins",
    "sql": "SELECT evidence_digest,COUNT(DISTINCT value) FROM live_right_answers WHERE question_id='q' AND evidence_digest='e4' AND cannot_tell IS NULL GROUP BY evidence_digest HAVING COUNT(DISTINCT value)>1",
    "expected": [
      [
        "e4",
        2
      ]
    ]
  },
  {
    "name": "20_cannot_tell_is_not_false",
    "sql": "SELECT id,value,cannot_tell FROM right_answers WHERE cannot_tell IS NOT NULL",
    "expected": [
      [
        "cannot",
        null,
        "insufficient evidence"
      ]
    ]
  },
  {
    "name": "21_agent_blind_audit_raw_pairs",
    "sql": "SELECT a.evidence_digest,a.value,p.value,CASE WHEN CAST(a.seen->>'answers' AS TEXT) IN ('false','0') THEN 0 ELSE 1 END FROM right_answers a JOIN right_answers p ON p.evidence_digest=a.evidence_digest AND p.question_id=a.question_id WHERE a.agent_id='agent' AND p.id IN ('r1','r2','r3') ORDER BY a.evidence_digest",
    "expected": [
      [
        "e1",
        "yes",
        "yes",
        0
      ],
      [
        "e2",
        "yes",
        "no",
        0
      ],
      [
        "e3",
        "no",
        "yes",
        0
      ]
    ]
  },
  {
    "name": "22_random_stratum_separate",
    "sql": "SELECT k.stratum,COUNT(*),SUM(CASE WHEN a.reading->>'value'=r.value THEN 1 ELSE 0 END) FROM eval_answers ea JOIN answers a ON a.answer_id=ea.answer_id JOIN answer_keys k ON k.id=ea.member_id JOIN right_answers r ON r.id=k.right_answer_id WHERE ea.eval_id='bench' AND ea.setup='A' GROUP BY k.stratum ORDER BY k.stratum",
    "expected": [
      [
        "random",
        2,
        1
      ],
      [
        "unsure",
        1,
        1
      ]
    ]
  },
  {
    "name": "23_notebook_picks_cli_handles",
    "sql": "SELECT p.id,p.stratum,a.answer_id FROM picks p JOIN answers a ON a.evidence_digest=p.evidence_digest WHERE a.answer_id='aA3' ORDER BY p.id",
    "expected": [
      [
        "p3",
        "random",
        "aA3"
      ]
    ]
  },
  {
    "name": "24_held_read_count",
    "sql": "SELECT name,held_reads FROM evals WHERE id='bench'",
    "expected": [
      [
        "five-model",
        1
      ]
    ]
  },
  {
    "name": "25_order_metrics_plain_SQL",
    "sql": "WITH perm AS (SELECT DISTINCT m.position,a.reading->'order'->>(m.position-1) AS member FROM answers a JOIN evidence_members m ON m.evidence_digest='set' WHERE a.answer_id='rank'), grades AS (SELECT p.position,CAST(r.value AS DOUBLE PRECISION) AS grade FROM perm p JOIN right_answers r ON r.evidence_digest=p.member AND r.question_id='retrieve' AND r.query_digest='queryA') SELECT SUM(CASE WHEN position<=1 AND grade>0 THEN 1 ELSE 0 END),1.0/MIN(CASE WHEN grade>0 THEN position END),SUM(CASE WHEN position<=2 AND grade>0 THEN 1 ELSE 0 END)/2.0,ROUND(CAST(SUM((POWER(2,grade)-1)/(LN(position+1)/LN(2)))/(7+1/(LN(3)/LN(2))) AS NUMERIC),4) FROM grades",
    "expected": [
      [
        1,
        1,
        1,
        1
      ]
    ]
  },
  {
    "name": "26_frozen_eval_unchanged_after_newest_changes",
    "sql": "SELECT ea.setup,SUM(CASE WHEN a.reading->>'value'=r.value THEN 1 ELSE 0 END) FROM eval_answers ea JOIN answers a ON a.answer_id=ea.answer_id JOIN answer_keys k ON k.id=ea.member_id JOIN right_answers r ON r.id=k.right_answer_id WHERE ea.eval_id='bench' AND ea.setup='A' GROUP BY ea.setup",
    "expected": [
      [
        "A",
        2
      ]
    ]
  }
]

```

## Initial review and targeted repair

The first reviewer rejected the initial coverage claim because the corpus did not construct the three curation queue orders with a random share and did not select observations under each loose-reuse rule. A fresh experimenter added independent cases on both engines. The following additive report supersedes those initial coverage claims; no initial evidence was overwritten.

### Repair finding

This additive evidence supersedes the initial REPORT.md's coverage claim for queue construction and loose-reuse lookup. All initial files and initial attempt outputs remain unchanged. The original 26 queries remain useful, but query 03 only inspected an existing loose answer, query 07 selected a known pair, and query 22 read stored strata. None established the missing lookup or queue behavior.

The repaired corpus contains 39 queries: the original 26 and 13 new queries. The final shared SQL passes 39/39 on SQLite 3.45.1 and PostgreSQL 16.15. SQLite integrity_check returns ok and foreign_key_check returns no rows. PostgreSQL enforces foreign keys during loading. repair-results.json holds every actual and independently specified expected row; repair-queries.json and repair-queries.sql freeze the 13 additions. The runner executes the original queries too.

The inputs remain the same invented fixture and local sources pinned in manifest.json. PRD 01, “Reuse rules” and “Newest,” supplies optional dimensions, mandatory wording text/evidence/context matches, and newest-time/insertion-order selection. PRD 02, the queue paragraph, supplies nearest-cut sorting, between-model and imported-label comparison, and a default 20% random share. specification/cache.md's identity and lookup sections and engine/store.rs and store/rows.rs establish the current exact-key contract. Loose rules are draft future behavior, not existing engine behavior. This repair adds no product behavior or policy approval.

The new fixture supplies five target items, two models, imported labels, and competing items from another tool and another question. Both competing calls share the same source step. Therefore the tool and question filters must do real work. Queries 38 and 39 remove one filter each and expose the corresponding competing item as the first queue entry. The target question's observations all share context and authored version. The queue binds a selected source step and setup, rather than silently mixing refresh history.

| Queries | Read | Handwritten result |
| --- | --- | --- |
| 27 | Nearest the 0.5 cut | u1, u2, u3, u4, u5; distances .01, .20, .40, .45, .49 |
| 28 | Descending between-model probability gap | u4, u2, u1, u3, u5; gaps .90, .60, .19, .10, 0 |
| 29 | Confidence queue with one random item out of five | random u5; priority u1, u2, u3, u4 |
| 30 | Model-disagreement queue with one random item out of five | random u5; priority u4, u2, u1, u3 |
| 31 | Imported-label disagreement with one random item out of five | random u5; priority u1, u2, u3, u4 |
| 32 | any-address lookup | lookup-address, model A, answering model A-v1 |
| 33 | any-model-version lookup | lookup-version, model A, answering model A-v2 |
| 34 | any-model lookup | lookup-model, model B, answering model B-v1 |
| 35 | any-wording-version lookup | lookup-wording-version, model A, answering model A-v1 |
| 36 | All four optional dimensions relaxed together | lookup-combined, model B, answering model B-v2 |
| 37 | All semantic dimensions constrained | lookup-base, model A, answering model A-v1 |
| 38–39 | Omit tool or question guard | wrong-tool or wrong-question leads |

Each queue derives priorities from probabilities or imported labels in SQL. The fixture has no preassigned queue strata. A supplied random draw selects u5 before priority selection, and the SQL excludes it from priority candidates. The output labels one random and four priority rows, so there is no duplicate item or changing denominator. Imported priority candidates require an actual label mismatch; the random item may agree. All candidate lists and expected orders are handwritten in repair.py and frozen as literal expectations in repair-queries.json. No expectation is calculated from query results.

The draft does not define disagreement strength, ties, random seeds or interleaving. Query 28's absolute probability gap, alphabetical ties in query 31, the supplied draw, and random-first display are explicit fixture choices. They establish that the stored data can support selection and retain strata; they do not choose the shipped queue algorithm or claim review efficiency or statistical validity. The deterministic draw is an input relation and carries no stratum assignment. The required 20% share is tested on a five-row result, with no claim about rounding other sizes.

Lookup queries select observation IDs directly from candidates, rather than looking up a pre-authored answer ID. Six legitimate candidates differ separately by address, answering-model version, requested model, authored wording version, and all dimensions together. Newer traps change wording bytes, evidence, context, adapter, or question. Every rule excludes those mandatory mismatches. Model relaxation also relaxes the reported model, so a B observation remains B/B-v1 in the result. Newest selection uses answered_at then insertion_order; original queries 01, 02 and 12 retain the existing tie, late-import and no-store evidence. Query 37 is a semantic-parts diagnostic, not a replacement for exact-key reuse.

One representation gap remains a concrete schema requirement. The original wording row keys solely on its content digest and holds one number. Two authored versions with identical wording bytes need the same content digest but different version metadata. The baseline attempt to insert version 2 of existing w1 fails on the primary key on both engines; repair-results.json retains the actual errors. A digest cannot truthfully become different solely to encode a version. repair-changes.sql proposes observation.wording_version so two observations can preserve their authored versions while sharing wording bytes. It also exposes a wordings view, allowing an observation lookup before any answer exists without reading the private wording table. All new query reads then use views. This is an experimental proposal for the database ticket, not an accepted migration.

There is also a draft contract conflict. PRD 01's default exact key and question 1 keep authored version numbers outside exact identity; the same PRD names any-wording-version as a relaxation. The current specification likewise excludes authored name/version metadata from answer identity. An exact-key hit therefore cannot enforce a version distinction that its key omits. The database ticket must define when the optional version filter applies and retain version provenance separately from content identity. This experiment proves that the proposed additional representation can perform either explicit semantic lookup. It does not silently change exact-key semantics. Wording bytes continue to match in every successful loose lookup; changing wording bytes is never treated as a version-only change.

Exact wire identity and incomplete imported semantic links remain under the related 0586 finding. These synthetic decomposed lookups do not establish that authored wording, evidence and context reconstruct dynamic recognition or relation wire questions. The exact key remains authoritative for exact reuse, and incomplete imports must remain exact-only until the required semantic links exist. No result here weakens that limitation.

repair.py, repair-fixture.sql, repair-changes.sql, query files, attempt databases, result files and logs preserve the targeted additions. The final PostgreSQL dump preserves schema and rows. repair-manifest.json hashes the final additions and confirms that all original artifacts still match manifest.json. Intermediate repair runs retain their own SQLite databases and result files. The first repair run passed 11 new queries on each engine; the second added scope controls and tightened imported mismatch selection; the third replayed all 39 queries and retained the actual digest collision. The final run uses the standalone wordings view so lookup reads stay on views.

The labelled PostgreSQL container used no network or persistent volume and had a half-CPU and 256 MB memory cap. It was removed after saving the final dump. No repository file, model call, build, commit, publication, pm write or 0.2 artifact was changed. No raw evidence was deleted. The experiment stops at these two review gaps.


### repair-changes.sql

```sql
ALTER TABLE observation ADD COLUMN wording_version INTEGER;
DROP VIEW observations;
CREATE VIEW observations AS SELECT * FROM observation;
CREATE VIEW wordings AS SELECT * FROM wording;

```

### repair-fixture.sql

```sql
INSERT INTO call (id,function,tool,step) VALUES ('queue-call','decide','refund','queue-source');
INSERT INTO call (id,function,tool,step) VALUES ('other-call','decide','other-tool','queue-source');
INSERT INTO question (id,function,contract) VALUES ('other-q','decide','{}');
INSERT INTO wording (digest,question_id,number,text) VALUES ('other-w','other-q',1,'Is a refund requested?');
INSERT INTO evidence (digest,kind,text,truth_digest) VALUES ('u1','single','u1','u1');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,answered_at,insertion_order,call_id,origin) VALUES ('queue-u1-A','queue-u1-A','fixture','https://example.invalid','A','A-v1','w1','u1','c','{"yes": 0.51}',100,1005,'queue-call','synthetic');
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('a-queue-u1-A','{"value": "yes"}',0.5,'queue-u1-A','queue-call','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('a-queue-u1-A',1,'queue-u1-A');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,answered_at,insertion_order,call_id,origin) VALUES ('queue-u1-B','queue-u1-B','fixture','https://example.invalid','B','B-v1','w1','u1','c','{"yes": 0.7}',100,1008,'queue-call','synthetic');
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('a-queue-u1-B','{"value": "yes"}',0.5,'queue-u1-B','queue-call','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('a-queue-u1-B',1,'queue-u1-B');
INSERT INTO right_answer (id,question_id,evidence_digest,value,basis,stratum,settled_at) VALUES ('label-u1','q','u1','no','person','imported',100);
INSERT INTO evidence (digest,kind,text,truth_digest) VALUES ('u2','single','u2','u2');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,answered_at,insertion_order,call_id,origin) VALUES ('queue-u2-A','queue-u2-A','fixture','https://example.invalid','A','A-v1','w1','u2','c','{"yes": 0.7}',100,1013,'queue-call','synthetic');
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('a-queue-u2-A','{"value": "yes"}',0.5,'queue-u2-A','queue-call','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('a-queue-u2-A',1,'queue-u2-A');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,answered_at,insertion_order,call_id,origin) VALUES ('queue-u2-B','queue-u2-B','fixture','https://example.invalid','B','B-v1','w1','u2','c','{"yes": 0.1}',100,1016,'queue-call','synthetic');
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('a-queue-u2-B','{"value": "no"}',0.5,'queue-u2-B','queue-call','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('a-queue-u2-B',1,'queue-u2-B');
INSERT INTO right_answer (id,question_id,evidence_digest,value,basis,stratum,settled_at) VALUES ('label-u2','q','u2','no','person','imported',100);
INSERT INTO evidence (digest,kind,text,truth_digest) VALUES ('u3','single','u3','u3');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,answered_at,insertion_order,call_id,origin) VALUES ('queue-u3-A','queue-u3-A','fixture','https://example.invalid','A','A-v1','w1','u3','c','{"yes": 0.1}',100,1021,'queue-call','synthetic');
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('a-queue-u3-A','{"value": "no"}',0.5,'queue-u3-A','queue-call','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('a-queue-u3-A',1,'queue-u3-A');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,answered_at,insertion_order,call_id,origin) VALUES ('queue-u3-B','queue-u3-B','fixture','https://example.invalid','B','B-v1','w1','u3','c','{"yes": 0.2}',100,1024,'queue-call','synthetic');
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('a-queue-u3-B','{"value": "no"}',0.5,'queue-u3-B','queue-call','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('a-queue-u3-B',1,'queue-u3-B');
INSERT INTO right_answer (id,question_id,evidence_digest,value,basis,stratum,settled_at) VALUES ('label-u3','q','u3','yes','person','imported',100);
INSERT INTO evidence (digest,kind,text,truth_digest) VALUES ('u4','single','u4','u4');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,answered_at,insertion_order,call_id,origin) VALUES ('queue-u4-A','queue-u4-A','fixture','https://example.invalid','A','A-v1','w1','u4','c','{"yes": 0.95}',100,1029,'queue-call','synthetic');
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('a-queue-u4-A','{"value": "yes"}',0.5,'queue-u4-A','queue-call','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('a-queue-u4-A',1,'queue-u4-A');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,answered_at,insertion_order,call_id,origin) VALUES ('queue-u4-B','queue-u4-B','fixture','https://example.invalid','B','B-v1','w1','u4','c','{"yes": 0.05}',100,1032,'queue-call','synthetic');
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('a-queue-u4-B','{"value": "no"}',0.5,'queue-u4-B','queue-call','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('a-queue-u4-B',1,'queue-u4-B');
INSERT INTO right_answer (id,question_id,evidence_digest,value,basis,stratum,settled_at) VALUES ('label-u4','q','u4','no','person','imported',100);
INSERT INTO evidence (digest,kind,text,truth_digest) VALUES ('u5','single','u5','u5');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,answered_at,insertion_order,call_id,origin) VALUES ('queue-u5-A','queue-u5-A','fixture','https://example.invalid','A','A-v1','w1','u5','c','{"yes": 0.99}',100,1037,'queue-call','synthetic');
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('a-queue-u5-A','{"value": "yes"}',0.5,'queue-u5-A','queue-call','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('a-queue-u5-A',1,'queue-u5-A');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,answered_at,insertion_order,call_id,origin) VALUES ('queue-u5-B','queue-u5-B','fixture','https://example.invalid','B','B-v1','w1','u5','c','{"yes": 0.99}',100,1040,'queue-call','synthetic');
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('a-queue-u5-B','{"value": "yes"}',0.5,'queue-u5-B','queue-call','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('a-queue-u5-B',1,'queue-u5-B');
INSERT INTO right_answer (id,question_id,evidence_digest,value,basis,stratum,settled_at) VALUES ('label-u5','q','u5','yes','person','imported',100);
INSERT INTO evidence (digest,kind,text,truth_digest) VALUES ('wrong-tool','single','wrong-tool','wrong-tool');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,answered_at,insertion_order,call_id,origin) VALUES ('queue-wrong-tool-A','queue-wrong-tool-A','fixture','https://example.invalid','A','A-v1','w1','wrong-tool','c','{"yes": 0.5001}',100,1045,'other-call','synthetic');
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('a-queue-wrong-tool-A','{"value": "yes"}',0.5,'queue-wrong-tool-A','other-call','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('a-queue-wrong-tool-A',1,'queue-wrong-tool-A');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,answered_at,insertion_order,call_id,origin) VALUES ('queue-wrong-tool-B','queue-wrong-tool-B','fixture','https://example.invalid','B','B-v1','w1','wrong-tool','c','{"yes": 0.01}',100,1048,'other-call','synthetic');
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('a-queue-wrong-tool-B','{"value": "no"}',0.5,'queue-wrong-tool-B','other-call','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('a-queue-wrong-tool-B',1,'queue-wrong-tool-B');
INSERT INTO right_answer (id,question_id,evidence_digest,value,basis,stratum,settled_at) VALUES ('label-wrong-tool','q','wrong-tool','no','person','imported',100);
INSERT INTO evidence (digest,kind,text,truth_digest) VALUES ('wrong-question','single','wrong-question','wrong-question');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,answered_at,insertion_order,call_id,origin) VALUES ('queue-wrong-question-A','queue-wrong-question-A','fixture','https://example.invalid','A','A-v1','other-w','wrong-question','c','{"yes": 0.50001}',100,1053,'queue-call','synthetic');
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('a-queue-wrong-question-A','{"value": "yes"}',0.5,'queue-wrong-question-A','queue-call','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('a-queue-wrong-question-A',1,'queue-wrong-question-A');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,answered_at,insertion_order,call_id,origin) VALUES ('queue-wrong-question-B','queue-wrong-question-B','fixture','https://example.invalid','B','B-v1','other-w','wrong-question','c','{"yes": 0.001}',100,1056,'queue-call','synthetic');
INSERT INTO answer (id,reading,threshold,observation_id,call_id,reuse_rule) VALUES ('a-queue-wrong-question-B','{"value": "no"}',0.5,'queue-wrong-question-B','queue-call','exact');
INSERT INTO answer_observation (answer_id,position,observation_id) VALUES ('a-queue-wrong-question-B',1,'queue-wrong-question-B');
INSERT INTO right_answer (id,question_id,evidence_digest,value,basis,stratum,settled_at) VALUES ('label-wrong-question','other-q','wrong-question','no','person','imported',100);
INSERT INTO evidence (digest,kind,text,truth_digest) VALUES ('lookup-e','single','lookup-e','lookup-e');
INSERT INTO evidence (digest,kind,text,truth_digest) VALUES ('lookup-other-e','single','lookup-other-e','lookup-other-e');
INSERT INTO context (digest,text) VALUES ('lookup-c','lookup context');
INSERT INTO context (digest,text) VALUES ('lookup-other-c','changed context');
INSERT INTO wording (digest,question_id,number,text) VALUES ('lookup-w','q',1,'Identical authored wording');
INSERT INTO wording (digest,question_id,number,text) VALUES ('lookup-other-q-w','other-q',1,'Identical authored wording');
INSERT INTO wording (digest,question_id,number,text) VALUES ('lookup-other-w','q',1,'Changed authored wording');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,wording_version,evidence_digest,context_digest,answered_at,insertion_order,origin) VALUES ('lookup-base','lookup-key-base','fixture','https://example.invalid','A','A-v1','lookup-w',1,'lookup-e','lookup-c',10,2010,'synthetic');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,wording_version,evidence_digest,context_digest,answered_at,insertion_order,origin) VALUES ('lookup-address','lookup-key-address','fixture','https://else.invalid','A','A-v1','lookup-w',1,'lookup-e','lookup-c',20,2020,'synthetic');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,wording_version,evidence_digest,context_digest,answered_at,insertion_order,origin) VALUES ('lookup-version','lookup-key-version','fixture','https://example.invalid','A','A-v2','lookup-w',1,'lookup-e','lookup-c',30,2030,'synthetic');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,wording_version,evidence_digest,context_digest,answered_at,insertion_order,origin) VALUES ('lookup-model','lookup-key-model','fixture','https://example.invalid','B','B-v1','lookup-w',1,'lookup-e','lookup-c',40,2040,'synthetic');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,wording_version,evidence_digest,context_digest,answered_at,insertion_order,origin) VALUES ('lookup-wording-version','lookup-key-wording-version','fixture','https://example.invalid','A','A-v1','lookup-w',2,'lookup-e','lookup-c',50,2050,'synthetic');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,wording_version,evidence_digest,context_digest,answered_at,insertion_order,origin) VALUES ('lookup-combined','lookup-key-combined','fixture','https://else.invalid','B','B-v2','lookup-w',2,'lookup-e','lookup-c',60,2060,'synthetic');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,wording_version,evidence_digest,context_digest,answered_at,insertion_order,origin) VALUES ('lookup-wording-trap','lookup-key-wording-trap','fixture','https://example.invalid','A','A-v1','lookup-other-w',1,'lookup-e','lookup-c',90,2090,'synthetic');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,wording_version,evidence_digest,context_digest,answered_at,insertion_order,origin) VALUES ('lookup-evidence-trap','lookup-key-evidence-trap','fixture','https://example.invalid','A','A-v1','lookup-w',1,'lookup-other-e','lookup-c',91,2091,'synthetic');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,wording_version,evidence_digest,context_digest,answered_at,insertion_order,origin) VALUES ('lookup-context-trap','lookup-key-context-trap','fixture','https://example.invalid','A','A-v1','lookup-w',1,'lookup-e','lookup-other-c',92,2092,'synthetic');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,wording_version,evidence_digest,context_digest,answered_at,insertion_order,origin) VALUES ('lookup-adapter-trap','lookup-key-adapter-trap','different','https://example.invalid','A','A-v1','lookup-w',1,'lookup-e','lookup-c',93,2093,'synthetic');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,wording_version,evidence_digest,context_digest,answered_at,insertion_order,origin) VALUES ('lookup-question-trap','lookup-key-question-trap','fixture','https://example.invalid','A','A-v1','lookup-other-q-w',1,'lookup-e','lookup-c',94,2094,'synthetic');

```

### repair-queries.json

```json
[
  {
    "name": "27_queue_lowest_confidence",
    "sql": "WITH scoped AS (SELECT a.*,CAST(a.probabilities->>'yes' AS DOUBLE PRECISION) AS p FROM answers a JOIN calls c ON c.id=a.call_id WHERE c.tool='refund' AND a.question_id='q' AND a.context_digest='c' AND c.step='queue-source'), pairs AS (SELECT a.evidence_digest,a.p,ABS(a.p-.5) AS distance,ROUND(CAST(ABS(a.p-b.p) AS NUMERIC),2) AS gap,a.reading->>'value' AS value FROM scoped a JOIN scoped b ON a.evidence_digest=b.evidence_digest AND b.model='B' WHERE a.model='A') SELECT evidence_digest,ROUND(CAST(distance AS NUMERIC),2) FROM pairs ORDER BY distance,evidence_digest",
    "expected": [
      [
        "u1",
        0.01
      ],
      [
        "u2",
        0.2
      ],
      [
        "u3",
        0.4
      ],
      [
        "u4",
        0.45
      ],
      [
        "u5",
        0.49
      ]
    ]
  },
  {
    "name": "28_queue_ordered_model_disagreement",
    "sql": "WITH scoped AS (SELECT a.*,CAST(a.probabilities->>'yes' AS DOUBLE PRECISION) AS p FROM answers a JOIN calls c ON c.id=a.call_id WHERE c.tool='refund' AND a.question_id='q' AND a.context_digest='c' AND c.step='queue-source'), pairs AS (SELECT a.evidence_digest,a.p,ABS(a.p-.5) AS distance,ROUND(CAST(ABS(a.p-b.p) AS NUMERIC),2) AS gap,a.reading->>'value' AS value FROM scoped a JOIN scoped b ON a.evidence_digest=b.evidence_digest AND b.model='B' WHERE a.model='A') SELECT evidence_digest,gap FROM pairs ORDER BY gap DESC,evidence_digest",
    "expected": [
      [
        "u4",
        0.9
      ],
      [
        "u2",
        0.6
      ],
      [
        "u1",
        0.19
      ],
      [
        "u3",
        0.1
      ],
      [
        "u5",
        0
      ]
    ]
  },
  {
    "name": "29_queue_confidence_20_percent_random",
    "sql": "WITH scoped AS (SELECT a.*,CAST(a.probabilities->>'yes' AS DOUBLE PRECISION) AS p FROM answers a JOIN calls c ON c.id=a.call_id WHERE c.tool='refund' AND a.question_id='q' AND a.context_digest='c' AND c.step='queue-source'), pairs AS (SELECT a.evidence_digest,a.p,ABS(a.p-.5) AS distance,ROUND(CAST(ABS(a.p-b.p) AS NUMERIC),2) AS gap,a.reading->>'value' AS value FROM scoped a JOIN scoped b ON a.evidence_digest=b.evidence_digest AND b.model='B' WHERE a.model='A'), draw(evidence_digest,random_draw) AS (VALUES ('u1',31),('u2',7),('u3',19),('u4',43),('u5',2),('wrong-tool',1),('wrong-question',0)), random_pick AS (SELECT p.evidence_digest FROM pairs p JOIN draw d ON d.evidence_digest=p.evidence_digest ORDER BY d.random_draw,p.evidence_digest LIMIT 1), priority AS (SELECT p.evidence_digest,ROW_NUMBER() OVER (ORDER BY distance,p.evidence_digest) AS n FROM pairs p JOIN live_right_answers r ON r.evidence_digest=p.evidence_digest AND r.question_id='q' AND r.stratum='imported' WHERE NOT EXISTS (SELECT 1 FROM random_pick z WHERE z.evidence_digest=p.evidence_digest)), selected AS (SELECT evidence_digest,'random' AS stratum,0 AS n FROM random_pick UNION ALL SELECT evidence_digest,'confidence',n FROM priority WHERE n<=4) SELECT evidence_digest,stratum FROM selected ORDER BY n,evidence_digest",
    "expected": [
      [
        "u5",
        "random"
      ],
      [
        "u1",
        "confidence"
      ],
      [
        "u2",
        "confidence"
      ],
      [
        "u3",
        "confidence"
      ],
      [
        "u4",
        "confidence"
      ]
    ]
  },
  {
    "name": "30_queue_model_disagreement_20_percent_random",
    "sql": "WITH scoped AS (SELECT a.*,CAST(a.probabilities->>'yes' AS DOUBLE PRECISION) AS p FROM answers a JOIN calls c ON c.id=a.call_id WHERE c.tool='refund' AND a.question_id='q' AND a.context_digest='c' AND c.step='queue-source'), pairs AS (SELECT a.evidence_digest,a.p,ABS(a.p-.5) AS distance,ROUND(CAST(ABS(a.p-b.p) AS NUMERIC),2) AS gap,a.reading->>'value' AS value FROM scoped a JOIN scoped b ON a.evidence_digest=b.evidence_digest AND b.model='B' WHERE a.model='A'), draw(evidence_digest,random_draw) AS (VALUES ('u1',31),('u2',7),('u3',19),('u4',43),('u5',2),('wrong-tool',1),('wrong-question',0)), random_pick AS (SELECT p.evidence_digest FROM pairs p JOIN draw d ON d.evidence_digest=p.evidence_digest ORDER BY d.random_draw,p.evidence_digest LIMIT 1), priority AS (SELECT p.evidence_digest,ROW_NUMBER() OVER (ORDER BY -gap,p.evidence_digest) AS n FROM pairs p JOIN live_right_answers r ON r.evidence_digest=p.evidence_digest AND r.question_id='q' AND r.stratum='imported' WHERE NOT EXISTS (SELECT 1 FROM random_pick z WHERE z.evidence_digest=p.evidence_digest)), selected AS (SELECT evidence_digest,'random' AS stratum,0 AS n FROM random_pick UNION ALL SELECT evidence_digest,'model_disagreement',n FROM priority WHERE n<=4) SELECT evidence_digest,stratum FROM selected ORDER BY n,evidence_digest",
    "expected": [
      [
        "u5",
        "random"
      ],
      [
        "u4",
        "model_disagreement"
      ],
      [
        "u2",
        "model_disagreement"
      ],
      [
        "u1",
        "model_disagreement"
      ],
      [
        "u3",
        "model_disagreement"
      ]
    ]
  },
  {
    "name": "31_queue_imported_label_disagreement_20_percent_random",
    "sql": "WITH scoped AS (SELECT a.*,CAST(a.probabilities->>'yes' AS DOUBLE PRECISION) AS p FROM answers a JOIN calls c ON c.id=a.call_id WHERE c.tool='refund' AND a.question_id='q' AND a.context_digest='c' AND c.step='queue-source'), pairs AS (SELECT a.evidence_digest,a.p,ABS(a.p-.5) AS distance,ROUND(CAST(ABS(a.p-b.p) AS NUMERIC),2) AS gap,a.reading->>'value' AS value FROM scoped a JOIN scoped b ON a.evidence_digest=b.evidence_digest AND b.model='B' WHERE a.model='A'), draw(evidence_digest,random_draw) AS (VALUES ('u1',31),('u2',7),('u3',19),('u4',43),('u5',2),('wrong-tool',1),('wrong-question',0)), random_pick AS (SELECT p.evidence_digest FROM pairs p JOIN draw d ON d.evidence_digest=p.evidence_digest ORDER BY d.random_draw,p.evidence_digest LIMIT 1), priority AS (SELECT p.evidence_digest,ROW_NUMBER() OVER (ORDER BY CASE WHEN p.value<>r.value THEN 0 ELSE 1 END,p.evidence_digest) AS n FROM pairs p JOIN live_right_answers r ON r.evidence_digest=p.evidence_digest AND r.question_id='q' AND r.stratum='imported' WHERE p.value<>r.value AND NOT EXISTS (SELECT 1 FROM random_pick z WHERE z.evidence_digest=p.evidence_digest)), selected AS (SELECT evidence_digest,'random' AS stratum,0 AS n FROM random_pick UNION ALL SELECT evidence_digest,'imported_label_disagreement',n FROM priority WHERE n<=4) SELECT evidence_digest,stratum FROM selected ORDER BY n,evidence_digest",
    "expected": [
      [
        "u5",
        "random"
      ],
      [
        "u1",
        "imported_label_disagreement"
      ],
      [
        "u2",
        "imported_label_disagreement"
      ],
      [
        "u3",
        "imported_label_disagreement"
      ],
      [
        "u4",
        "imported_label_disagreement"
      ]
    ]
  },
  {
    "name": "32_lookup_any_address",
    "sql": "SELECT o.id,o.model,o.answering_model FROM observations o JOIN wordings w ON w.digest=o.wording_digest WHERE o.adapter='fixture' AND w.question_id='q' AND w.text='Identical authored wording' AND o.evidence_digest='lookup-e' AND o.context_digest='lookup-c' AND o.model='A' AND o.answering_model='A-v1' AND o.wording_version=1 ORDER BY o.answered_at DESC,o.insertion_order DESC LIMIT 1",
    "expected": [
      [
        "lookup-address",
        "A",
        "A-v1"
      ]
    ]
  },
  {
    "name": "33_lookup_any_model_version",
    "sql": "SELECT o.id,o.model,o.answering_model FROM observations o JOIN wordings w ON w.digest=o.wording_digest WHERE o.adapter='fixture' AND w.question_id='q' AND w.text='Identical authored wording' AND o.evidence_digest='lookup-e' AND o.context_digest='lookup-c' AND o.address='https://example.invalid' AND o.model='A' AND o.wording_version=1 ORDER BY o.answered_at DESC,o.insertion_order DESC LIMIT 1",
    "expected": [
      [
        "lookup-version",
        "A",
        "A-v2"
      ]
    ]
  },
  {
    "name": "34_lookup_any_model",
    "sql": "SELECT o.id,o.model,o.answering_model FROM observations o JOIN wordings w ON w.digest=o.wording_digest WHERE o.adapter='fixture' AND w.question_id='q' AND w.text='Identical authored wording' AND o.evidence_digest='lookup-e' AND o.context_digest='lookup-c' AND o.address='https://example.invalid' AND o.wording_version=1 ORDER BY o.answered_at DESC,o.insertion_order DESC LIMIT 1",
    "expected": [
      [
        "lookup-model",
        "B",
        "B-v1"
      ]
    ]
  },
  {
    "name": "35_lookup_any_wording_version",
    "sql": "SELECT o.id,o.model,o.answering_model FROM observations o JOIN wordings w ON w.digest=o.wording_digest WHERE o.adapter='fixture' AND w.question_id='q' AND w.text='Identical authored wording' AND o.evidence_digest='lookup-e' AND o.context_digest='lookup-c' AND o.address='https://example.invalid' AND o.model='A' AND o.answering_model='A-v1' ORDER BY o.answered_at DESC,o.insertion_order DESC LIMIT 1",
    "expected": [
      [
        "lookup-wording-version",
        "A",
        "A-v1"
      ]
    ]
  },
  {
    "name": "36_lookup_combined",
    "sql": "SELECT o.id,o.model,o.answering_model FROM observations o JOIN wordings w ON w.digest=o.wording_digest WHERE o.adapter='fixture' AND w.question_id='q' AND w.text='Identical authored wording' AND o.evidence_digest='lookup-e' AND o.context_digest='lookup-c' ORDER BY o.answered_at DESC,o.insertion_order DESC LIMIT 1",
    "expected": [
      [
        "lookup-combined",
        "B",
        "B-v2"
      ]
    ]
  },
  {
    "name": "37_lookup_strict_parts",
    "sql": "SELECT o.id,o.model,o.answering_model FROM observations o JOIN wordings w ON w.digest=o.wording_digest WHERE o.adapter='fixture' AND w.question_id='q' AND w.text='Identical authored wording' AND o.evidence_digest='lookup-e' AND o.context_digest='lookup-c' AND o.address='https://example.invalid' AND o.model='A' AND o.answering_model='A-v1' AND o.wording_version=1 ORDER BY o.answered_at DESC,o.insertion_order DESC LIMIT 1",
    "expected": [
      [
        "lookup-base",
        "A",
        "A-v1"
      ]
    ]
  },
  {
    "name": "38_scope_guard_counterexample",
    "sql": "WITH scoped AS (SELECT a.*,CAST(a.probabilities->>'yes' AS DOUBLE PRECISION) AS p FROM answers a JOIN calls c ON c.id=a.call_id WHERE a.question_id='q' AND a.context_digest='c' AND c.step='queue-source'), pairs AS (SELECT a.evidence_digest,a.p,ABS(a.p-.5) AS distance,ROUND(CAST(ABS(a.p-b.p) AS NUMERIC),2) AS gap,a.reading->>'value' AS value FROM scoped a JOIN scoped b ON a.evidence_digest=b.evidence_digest AND b.model='B' WHERE a.model='A') SELECT evidence_digest FROM pairs ORDER BY distance,evidence_digest LIMIT 1",
    "expected": [
      [
        "wrong-tool"
      ]
    ]
  },
  {
    "name": "39_scope_guard_counterexample",
    "sql": "WITH scoped AS (SELECT a.*,CAST(a.probabilities->>'yes' AS DOUBLE PRECISION) AS p FROM answers a JOIN calls c ON c.id=a.call_id WHERE c.tool='refund' AND a.context_digest='c' AND c.step='queue-source'), pairs AS (SELECT a.evidence_digest,a.p,ABS(a.p-.5) AS distance,ROUND(CAST(ABS(a.p-b.p) AS NUMERIC),2) AS gap,a.reading->>'value' AS value FROM scoped a JOIN scoped b ON a.evidence_digest=b.evidence_digest AND b.model='B' WHERE a.model='A') SELECT evidence_digest FROM pairs ORDER BY distance,evidence_digest LIMIT 1",
    "expected": [
      [
        "wrong-question"
      ]
    ]
  }
]

```

## Confidence queue review correction

The second reviewer reproduced the 39-query corpus and accepted the lookup, scoping and version evidence, but rejected the confidence queries because they joined a second model and imported labels. A fresh experimenter corrected only the affected candidate queries and added explicit missing-data counterexamples. The following report and query blocks supersede the earlier confidence queue implementation.

### Confidence queue dependency correction

The narrow risk was that confidence selection silently required another model answer or an imported label. This additive experiment corrects that finding without changing any prior artifact. The source checkout remains read-only at a37ca83ca0a66d7681539f85c9b342f031a515af. OWNER still names thinkthen/0584.

The ideal-state gap, invented source data and source rights remain those recorded in PLAN.md and manifest.json. PRD 02 permits confidence curation without model B or truth labels. This run uses only invented observations and labels. It changes no policy or product schema. The stop rule was matching targeted literal expectations and the prior 39-query regression on both engines, followed by removal of the owned container.

Queries 27 and 29 now derive candidates directly from the observations public view, joining calls and wordings to retain the source step, tool, question and context guards. They select model A and never join model B, an answer, or an imported label. Query 28 and the model-disagreement queue require B but no imported label. Imported-label disagreement requires an imported mismatch but no model B; its random component can sample any scoped A observation.

confidence-fixture.sql adds three cases, all without answer rows. u0 has A=.501, no B, and no label. u0-labelled has A=.52, no B, and an imported no label. u0-model-only has A=.53 and B=.03, and no label. They preserve the original queue-source scope. The new cases have no stored queue stratum. A supplied draw remains a fixture input. All expected rows in confidence-queries.json are handwritten literals, independent of actual SQL output.

| Query | Exact expected result |
| --- | --- |
| 27 | u0 .001; u1 .01; u0-labelled .02; u0-model-only .03; u2 .20; u3 .40; u4 .45; u5 .49 |
| 28 | u4 .90; u2 .60; u0-model-only .50; u1 .19; u3 .10; u5 0 |
| 29 | random u5; confidence u0, u1, u0-labelled, u0-model-only |
| 30 | random u5; model disagreement u4, u2, u0-model-only, u1 |
| 31 | random u5; imported disagreement u0-labelled, u1, u2, u3 |
| 40 | u0 has zero B observations, zero labels and zero answer rows |
| 41 | Among new cases, only u0-model-only qualifies for model disagreement |
| 42 | Among new cases, only u0-labelled qualifies for imported disagreement |
| 43 | With u0's draw first: random u0; confidence u1, u0-labelled, u0-model-only, u2 |

SQLite 3.45.1 passes all 39 prior queries before adding the delta and all nine targeted queries after adding it. integrity_check returns ok and foreign_key_check returns no rows. PostgreSQL 16.15 passes the same 39 prior queries and nine targeted queries with foreign keys enforced during loading. confidence-results.json retains each actual and expected row. Both engines execute identical targeted SQL. confidence-evidence.sqlite and confidence-postgres.dump.sql freeze the final loaded evidence. confidence-prior-hashes.json records all prior top-level files before the run; every hash still matches. The prior 39-query SQL, runner, results and report remain unchanged.

This correction replaces the queue claims for queries 27 through 31 in repair-REPORT.md. It reuses the other accepted lookup, authored-version, scope and schema findings there. It does not rerun or broaden those policies. The fixture still chooses a .5 cut, absolute probability gaps, alphabetical ties for imported mismatches, deterministic supplied random draws and random-first display. Five selected rows contain exactly one random row; other queue sizes and rounding remain untested. No performance, concurrency, production migration, model accuracy or statistical validity claim follows.

The owned PostgreSQL container ran with --rm, labels pm.repo=thinkthen and pm.ticket=0584, --network none, --cpus .5 and --memory 256m, without a persistent volume. The local socket initially denied access; the exact container operations then ran with approved sandbox escalation. After the dump was saved, docker rm -f removed thinkthen-0584-confidence successfully. No repository edit, model call, pi-job, broad build, commit, publication, pm write or further agent was used. The generated confidence Python bytecode was removed; it is reproducible from confidence.py. Unique evidence remains in this owned folder while ticket 0584 needs it.


### confidence-fixture.sql

```sql
INSERT INTO evidence (digest,kind,text,truth_digest) VALUES ('u0','single','u0','u0');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,answered_at,insertion_order,call_id,origin) VALUES ('confidence-u0-A','confidence-u0-A','fixture','https://example.invalid','A','A-v1','w1','u0','c','{"yes": 0.501}',100,3001,'queue-call','synthetic');
INSERT INTO evidence (digest,kind,text,truth_digest) VALUES ('u0-labelled','single','u0-labelled','u0-labelled');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,answered_at,insertion_order,call_id,origin) VALUES ('confidence-u0-labelled-A','confidence-u0-labelled-A','fixture','https://example.invalid','A','A-v1','w1','u0-labelled','c','{"yes": 0.52}',100,3003,'queue-call','synthetic');
INSERT INTO right_answer (id,question_id,evidence_digest,value,basis,stratum,settled_at) VALUES ('confidence-label-u0-labelled','q','u0-labelled','no','person','imported',100);
INSERT INTO evidence (digest,kind,text,truth_digest) VALUES ('u0-model-only','single','u0-model-only','u0-model-only');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,answered_at,insertion_order,call_id,origin) VALUES ('confidence-u0-model-only-A','confidence-u0-model-only-A','fixture','https://example.invalid','A','A-v1','w1','u0-model-only','c','{"yes": 0.53}',100,3006,'queue-call','synthetic');
INSERT INTO observation (id,exact_key,adapter,address,model,answering_model,wording_digest,evidence_digest,context_digest,probabilities,answered_at,insertion_order,call_id,origin) VALUES ('confidence-u0-model-only-B','confidence-u0-model-only-B','fixture','https://example.invalid','B','B-v1','w1','u0-model-only','c','{"yes": 0.03}',100,3007,'queue-call','synthetic');

```

### confidence-queries.json

```json
[
  {
    "name": "27_queue_lowest_confidence",
    "sql": "WITH scoped AS (SELECT o.*,w.question_id,CAST(o.probabilities->>'yes' AS DOUBLE PRECISION) AS p FROM observations o JOIN calls c ON c.id=o.call_id JOIN wordings w ON w.digest=o.wording_digest WHERE c.tool='refund' AND w.question_id='q' AND o.context_digest='c' AND c.step='queue-source'), a_candidates AS (SELECT evidence_digest,p,ABS(p-.5) AS distance,CASE WHEN p>=.5 THEN 'yes' ELSE 'no' END AS value FROM scoped WHERE model='A') SELECT evidence_digest,ROUND(CAST(distance AS NUMERIC),3) FROM a_candidates ORDER BY distance,evidence_digest",
    "expected": [
      [
        "u0",
        0.001
      ],
      [
        "u1",
        0.01
      ],
      [
        "u0-labelled",
        0.02
      ],
      [
        "u0-model-only",
        0.03
      ],
      [
        "u2",
        0.2
      ],
      [
        "u3",
        0.4
      ],
      [
        "u4",
        0.45
      ],
      [
        "u5",
        0.49
      ]
    ]
  },
  {
    "name": "28_queue_ordered_model_disagreement",
    "sql": "WITH scoped AS (SELECT o.*,w.question_id,CAST(o.probabilities->>'yes' AS DOUBLE PRECISION) AS p FROM observations o JOIN calls c ON c.id=o.call_id JOIN wordings w ON w.digest=o.wording_digest WHERE c.tool='refund' AND w.question_id='q' AND o.context_digest='c' AND c.step='queue-source'), a_candidates AS (SELECT evidence_digest,p,ABS(p-.5) AS distance,CASE WHEN p>=.5 THEN 'yes' ELSE 'no' END AS value FROM scoped WHERE model='A'), pairs AS (SELECT a.*,ROUND(CAST(ABS(a.p-b.p) AS NUMERIC),2) AS gap FROM a_candidates a JOIN scoped b ON b.evidence_digest=a.evidence_digest AND b.model='B') SELECT evidence_digest,gap FROM pairs ORDER BY gap DESC,evidence_digest",
    "expected": [
      [
        "u4",
        0.9
      ],
      [
        "u2",
        0.6
      ],
      [
        "u0-model-only",
        0.5
      ],
      [
        "u1",
        0.19
      ],
      [
        "u3",
        0.1
      ],
      [
        "u5",
        0
      ]
    ]
  },
  {
    "name": "29_queue_confidence_20_percent_random",
    "sql": "WITH scoped AS (SELECT o.*,w.question_id,CAST(o.probabilities->>'yes' AS DOUBLE PRECISION) AS p FROM observations o JOIN calls c ON c.id=o.call_id JOIN wordings w ON w.digest=o.wording_digest WHERE c.tool='refund' AND w.question_id='q' AND o.context_digest='c' AND c.step='queue-source'), a_candidates AS (SELECT evidence_digest,p,ABS(p-.5) AS distance,CASE WHEN p>=.5 THEN 'yes' ELSE 'no' END AS value FROM scoped WHERE model='A'), draw(evidence_digest,random_draw) AS (VALUES ('u1',31),('u2',7),('u3',19),('u4',43),('u5',2),('wrong-tool',1),('wrong-question',0),('u0',60),('u0-labelled',61),('u0-model-only',62)), random_pick AS (SELECT p.evidence_digest FROM a_candidates p JOIN draw d ON d.evidence_digest=p.evidence_digest ORDER BY d.random_draw,p.evidence_digest LIMIT 1), priority AS (SELECT p.evidence_digest,ROW_NUMBER() OVER (ORDER BY distance,p.evidence_digest) AS n FROM a_candidates p WHERE NOT EXISTS (SELECT 1 FROM random_pick z WHERE z.evidence_digest=p.evidence_digest)), selected AS (SELECT evidence_digest,'random' AS stratum,0 AS n FROM random_pick UNION ALL SELECT evidence_digest,'confidence',n FROM priority WHERE n<=4) SELECT evidence_digest,stratum FROM selected ORDER BY n,evidence_digest",
    "expected": [
      [
        "u5",
        "random"
      ],
      [
        "u0",
        "confidence"
      ],
      [
        "u1",
        "confidence"
      ],
      [
        "u0-labelled",
        "confidence"
      ],
      [
        "u0-model-only",
        "confidence"
      ]
    ]
  },
  {
    "name": "30_queue_model_disagreement_20_percent_random",
    "sql": "WITH scoped AS (SELECT o.*,w.question_id,CAST(o.probabilities->>'yes' AS DOUBLE PRECISION) AS p FROM observations o JOIN calls c ON c.id=o.call_id JOIN wordings w ON w.digest=o.wording_digest WHERE c.tool='refund' AND w.question_id='q' AND o.context_digest='c' AND c.step='queue-source'), a_candidates AS (SELECT evidence_digest,p,ABS(p-.5) AS distance,CASE WHEN p>=.5 THEN 'yes' ELSE 'no' END AS value FROM scoped WHERE model='A'), pairs AS (SELECT a.*,ROUND(CAST(ABS(a.p-b.p) AS NUMERIC),2) AS gap FROM a_candidates a JOIN scoped b ON b.evidence_digest=a.evidence_digest AND b.model='B'), draw(evidence_digest,random_draw) AS (VALUES ('u1',31),('u2',7),('u3',19),('u4',43),('u5',2),('wrong-tool',1),('wrong-question',0),('u0',60),('u0-labelled',61),('u0-model-only',62)), random_pick AS (SELECT p.evidence_digest FROM pairs p JOIN draw d ON d.evidence_digest=p.evidence_digest ORDER BY d.random_draw,p.evidence_digest LIMIT 1), priority AS (SELECT p.evidence_digest,ROW_NUMBER() OVER (ORDER BY -gap,p.evidence_digest) AS n FROM pairs p WHERE NOT EXISTS (SELECT 1 FROM random_pick z WHERE z.evidence_digest=p.evidence_digest)), selected AS (SELECT evidence_digest,'random' AS stratum,0 AS n FROM random_pick UNION ALL SELECT evidence_digest,'model_disagreement',n FROM priority WHERE n<=4) SELECT evidence_digest,stratum FROM selected ORDER BY n,evidence_digest",
    "expected": [
      [
        "u5",
        "random"
      ],
      [
        "u4",
        "model_disagreement"
      ],
      [
        "u2",
        "model_disagreement"
      ],
      [
        "u0-model-only",
        "model_disagreement"
      ],
      [
        "u1",
        "model_disagreement"
      ]
    ]
  },
  {
    "name": "31_queue_imported_label_disagreement_20_percent_random",
    "sql": "WITH scoped AS (SELECT o.*,w.question_id,CAST(o.probabilities->>'yes' AS DOUBLE PRECISION) AS p FROM observations o JOIN calls c ON c.id=o.call_id JOIN wordings w ON w.digest=o.wording_digest WHERE c.tool='refund' AND w.question_id='q' AND o.context_digest='c' AND c.step='queue-source'), a_candidates AS (SELECT evidence_digest,p,ABS(p-.5) AS distance,CASE WHEN p>=.5 THEN 'yes' ELSE 'no' END AS value FROM scoped WHERE model='A'), draw(evidence_digest,random_draw) AS (VALUES ('u1',31),('u2',7),('u3',19),('u4',43),('u5',2),('wrong-tool',1),('wrong-question',0),('u0',60),('u0-labelled',61),('u0-model-only',62)), random_pick AS (SELECT p.evidence_digest FROM a_candidates p JOIN draw d ON d.evidence_digest=p.evidence_digest ORDER BY d.random_draw,p.evidence_digest LIMIT 1), priority AS (SELECT p.evidence_digest,ROW_NUMBER() OVER (ORDER BY p.evidence_digest,p.evidence_digest) AS n FROM a_candidates p JOIN live_right_answers r ON r.evidence_digest=p.evidence_digest AND r.question_id='q' AND r.stratum='imported' WHERE p.value<>r.value AND NOT EXISTS (SELECT 1 FROM random_pick z WHERE z.evidence_digest=p.evidence_digest)), selected AS (SELECT evidence_digest,'random' AS stratum,0 AS n FROM random_pick UNION ALL SELECT evidence_digest,'imported_label_disagreement',n FROM priority WHERE n<=4) SELECT evidence_digest,stratum FROM selected ORDER BY n,evidence_digest",
    "expected": [
      [
        "u5",
        "random"
      ],
      [
        "u0-labelled",
        "imported_label_disagreement"
      ],
      [
        "u1",
        "imported_label_disagreement"
      ],
      [
        "u2",
        "imported_label_disagreement"
      ],
      [
        "u3",
        "imported_label_disagreement"
      ]
    ]
  },
  {
    "name": "40_missing_both_observation_without_answer",
    "sql": "SELECT o.evidence_digest,(SELECT COUNT(*) FROM observations b WHERE b.evidence_digest=o.evidence_digest AND b.model='B'),(SELECT COUNT(*) FROM live_right_answers r WHERE r.evidence_digest=o.evidence_digest),(SELECT COUNT(*) FROM answers z WHERE z.evidence_digest=o.evidence_digest) FROM observations o WHERE o.id='confidence-u0-A'",
    "expected": [
      [
        "u0",
        0,
        0,
        0
      ]
    ]
  },
  {
    "name": "41_model_requires_B_but_no_label",
    "sql": "WITH scoped AS (SELECT o.*,w.question_id,CAST(o.probabilities->>'yes' AS DOUBLE PRECISION) AS p FROM observations o JOIN calls c ON c.id=o.call_id JOIN wordings w ON w.digest=o.wording_digest WHERE c.tool='refund' AND w.question_id='q' AND o.context_digest='c' AND c.step='queue-source'), a_candidates AS (SELECT evidence_digest,p,ABS(p-.5) AS distance,CASE WHEN p>=.5 THEN 'yes' ELSE 'no' END AS value FROM scoped WHERE model='A'), pairs AS (SELECT a.*,ROUND(CAST(ABS(a.p-b.p) AS NUMERIC),2) AS gap FROM a_candidates a JOIN scoped b ON b.evidence_digest=a.evidence_digest AND b.model='B') SELECT evidence_digest FROM pairs WHERE evidence_digest LIKE 'u0%' ORDER BY evidence_digest",
    "expected": [
      [
        "u0-model-only"
      ]
    ]
  },
  {
    "name": "42_imported_requires_label_but_no_B",
    "sql": "WITH scoped AS (SELECT o.*,w.question_id,CAST(o.probabilities->>'yes' AS DOUBLE PRECISION) AS p FROM observations o JOIN calls c ON c.id=o.call_id JOIN wordings w ON w.digest=o.wording_digest WHERE c.tool='refund' AND w.question_id='q' AND o.context_digest='c' AND c.step='queue-source'), a_candidates AS (SELECT evidence_digest,p,ABS(p-.5) AS distance,CASE WHEN p>=.5 THEN 'yes' ELSE 'no' END AS value FROM scoped WHERE model='A'), labelled AS (SELECT a.* FROM a_candidates a JOIN live_right_answers r ON r.evidence_digest=a.evidence_digest AND r.question_id='q' AND r.stratum='imported' WHERE a.value<>r.value) SELECT evidence_digest FROM labelled WHERE evidence_digest LIKE 'u0%' ORDER BY evidence_digest",
    "expected": [
      [
        "u0-labelled"
      ]
    ]
  },
  {
    "name": "43_confidence_random_accepts_missing_both",
    "sql": "WITH scoped AS (SELECT o.*,w.question_id,CAST(o.probabilities->>'yes' AS DOUBLE PRECISION) AS p FROM observations o JOIN calls c ON c.id=o.call_id JOIN wordings w ON w.digest=o.wording_digest WHERE c.tool='refund' AND w.question_id='q' AND o.context_digest='c' AND c.step='queue-source'), a_candidates AS (SELECT evidence_digest,p,ABS(p-.5) AS distance,CASE WHEN p>=.5 THEN 'yes' ELSE 'no' END AS value FROM scoped WHERE model='A'), draw(evidence_digest,random_draw) AS (VALUES ('u0',0),('u1',31),('u2',7),('u3',19),('u4',43),('u5',2),('u0-labelled',61),('u0-model-only',62)), random_pick AS (SELECT p.evidence_digest FROM a_candidates p JOIN draw d ON d.evidence_digest=p.evidence_digest ORDER BY d.random_draw,p.evidence_digest LIMIT 1), priority AS (SELECT p.evidence_digest,ROW_NUMBER() OVER (ORDER BY distance,p.evidence_digest) AS n FROM a_candidates p WHERE NOT EXISTS (SELECT 1 FROM random_pick z WHERE z.evidence_digest=p.evidence_digest)), selected AS (SELECT evidence_digest,'random' AS stratum,0 AS n FROM random_pick UNION ALL SELECT evidence_digest,'confidence',n FROM priority WHERE n<=4) SELECT evidence_digest,stratum FROM selected ORDER BY n,evidence_digest",
    "expected": [
      [
        "u0",
        "random"
      ],
      [
        "u1",
        "confidence"
      ],
      [
        "u0-labelled",
        "confidence"
      ],
      [
        "u0-model-only",
        "confidence"
      ],
      [
        "u2",
        "confidence"
      ]
    ]
  }
]

```
