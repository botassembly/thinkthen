'use strict';
// Private strict result/2 carriers; no native execution is projected here.
const models = {
  "Position": {"file?":"str","first?":"positive","last?":"positive","images?":"[str]"},
  "Usage": {"input_tokens?":"uint","output_tokens?":"uint"},
  "ProfileWarning": {"tuned_for":"str","running":"str"},
  "BatchWarning": {"tuned_for":"batch","running":"batch"},
  "Attempt": {"ordinal":"positive","request_sha256":"Digest","wall_ms":"uint","outcome":"outcome","sdk_request_id":"SdkRequestId","status?":"uint","server_ms?":"uint","request_id?":"str"},
  "PersistenceObservation": {"state":"=disabled|=pending|=written|=failed","observed_at":"str","advice?":"str"},
  "Facts": {"largest_request_bytes?":"uint","largest_request_estimated_input_tokens?":"uint|null","token_estimate_method?":"str","usage_persistence?":"PersistenceObservation","call_id":"CallId","records":"uint","requests_sent":"uint","cache_answers":"uint","seconds":"number","input_tokens?":"uint","output_tokens?":"uint","model?":"str","estimated_cost_usd?":"cost","command_ms?":"uint","attempts?":"[Attempt]","held_model_mismatch?":"bool"},
  "QuestionSource": {"origin":"origin","answered_by":"str","batch_size?":"positive"},
  "Observed": {"observation_id":"ObservationId"},
  "FailedObservation": {"failure_id":"FailureId"},
  "Meta": {"tool":"str","url":"str","model":"str","requests_sent":"uint","cached":"bool","requests":"[Digest]","failed_questions":"uint","origin":"origin|null","question_sources":"[QuestionSource]","observations":"[Observation]","question_sha256?":"Digest","questions_sha256?":"Digest","answered_by?":"str","usage?":"Usage","profile_warning?":"ProfileWarning","batch_setting?":"batch","batch_warning?":"BatchWarning","context_sha256?":"Digest","attempts?":"[Attempt]"},
  "YesNo": {"kind":"=yes_no","probability":"probability"},
  "Choice": {"kind":"=choice","pick":"str","probabilities":"{probability}","confidence?":"probability"},
  "Tags": {"kind":"=tag","probabilities":"{probability}"},
  "Score": {"kind":"=score","level":"str","probabilities":"{probability}","confidence?":"probability"},
  "FindAnswer": {"kind":"=find","pick":"str","probabilities":"{probability}","confidence?":"probability"},
  "DecideQuestion": {"verb":"=decide","text":"text","true?":"description","false?":"description","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "ChooseQuestion": {"verb":"=choose","text":"text","options":"[str]","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "TagQuestion": {"verb":"=tag","text":"text","labels":"[str]","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "ScoreQuestion": {"verb":"=score","text":"text","levels":"[str]","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "FindQuestion": {"verb":"=find","text":"text","none":"bool","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "RelationRule": {"name":"str","source":"str","target":"str","reads":"str","either":"bool","single?":"bool"},
  "RelateFields": {"name":"str","kind":"str"},
  "RelateQuestion": {"verb":"=relate","fields":"RelateFields|null","relations":"[RelationRule]","threshold":"threshold","profile?":"str","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "RecognizeQuestion": {"verb":"=recognize","kinds":"{description}","instructions?":"str","entity_definition?":"str","relations?":"[RelationRule]","threshold":"threshold","relation_threshold":"threshold","on?":"str|[str]","profile?":"str","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "Failure": {"kind":"=backend","cause":"cause"},
  "FailedField": {"failed":"Failure"},
  "Entity": {"text":"text","start":"uint","end":"uint","length":"uint","kind":"str","strength":"number","file?":"str","first_line?":"positive","last_line?":"positive"},
  "Endpoint": {"name":"str","kind":"str","record?":"json","file?":"str","first_line?":"positive","last_line?":"positive"},
  "Edge": {"relation":"str","source":"Endpoint","target":"Endpoint","probability":"probability","either?":"=true"},
  "EntityEdge": {"relation":"str","source":"Entity","target":"Entity","probability":"probability","either?":"=true"},
  "Recognition": {"entities":"[Entity]","relations?":"[EntityEdge]"},
  "PieceOdds": {"start":"uint","end":"uint","tags":"{probability}"},
  "NameOdds": {"start":"uint","end":"uint","kinds":"{probability}|null","edges":"{probability}|null"},
  "Span": {"start":"uint","end":"uint"},
  "PairOdds": {"relation":"str","source":"Span","target":"Span","probability":"probability"},
  "RecognitionAnswer": {"pieces":"[PieceOdds]","names":"[NameOdds]","pairs":"[PairOdds]"},
  "AnnotationSuccess": {"answer_id":"AnswerId","value":"SuccessValue","question":"AtomicQuestion","answer":"AtomicAnswer","threshold":"threshold","request":"Digest"},
  "AnnotationFailure": {"failure_id":"FailureId","question":"AtomicQuestion","failure":"Failure","request":"Digest"},
  "RelationSuccess": {"relation":"str","reads":"str","method":"str","direction":"str","source":"Endpoint","target":"Endpoint|null","request":"Digest","answer_id":"AnswerId","probability":"probability","accepted":"bool","answer?":"AtomicAnswer"},
  "RelationFailure": {"relation":"str","reads":"str","method":"str","direction":"str","source":"Endpoint","target":"Endpoint|null","request":"Digest","failure_id":"FailureId","failure":"Failure"},
  "RelationAnswer": {"questions":"[RelationEntry]"},
  "DecideResult": {"schema":"=thinkthen.result/2","answer_id":"AnswerId","meta":"Meta","value":"bool|description","question":"DecideQuestion","answer":"YesNo","threshold":"threshold","input?":"json","position?":"Position","input_file?":"str","file?":"str","first_line?":"positive","last_line?":"positive","source?":"PhysicalSource","images?":"[NativeImage]","index?":"uint"},
  "ChooseResult": {"schema":"=thinkthen.result/2","answer_id":"AnswerId","meta":"Meta","value":"str|null","question":"ChooseQuestion","answer":"Choice","threshold":"threshold","input?":"json","position?":"Position","input_file?":"str","file?":"str","first_line?":"positive","last_line?":"positive","source?":"PhysicalSource","images?":"[NativeImage]","index?":"uint"},
  "TagResult": {"schema":"=thinkthen.result/2","answer_id":"AnswerId","meta":"Meta","value":"[str]","question":"TagQuestion","answer":"Tags","threshold":"threshold","input?":"json","position?":"Position","input_file?":"str","file?":"str","first_line?":"positive","last_line?":"positive","source?":"PhysicalSource","index?":"uint"},
  "ScoreResult": {"schema":"=thinkthen.result/2","answer_id":"AnswerId","meta":"Meta","value":"number","question":"ScoreQuestion","answer":"Score","threshold":"null","input?":"json","position?":"Position","input_file?":"str","file?":"str","first_line?":"positive","last_line?":"positive","source?":"PhysicalSource","images?":"[NativeImage]","index?":"uint"},
  "FilterResult": {"schema":"=thinkthen.result/2","answer_id":"AnswerId","meta":"Meta","value":"bool","input":"json","question":"DecideQuestion","answer":"YesNo","threshold":"threshold","position?":"Position","file?":"str","first_line?":"positive","last_line?":"positive","source?":"PhysicalSource","index?":"uint"},
  "RankMemberResult": {"schema":"=thinkthen.result/2","answer_id":"AnswerId","value":"positive","question":"DecideQuestion","answer":"YesNo","threshold":"null","meta":"Meta","source?":"PhysicalSource"},
  "RankMember": {"name":"str","result":"RankMemberResult"},
  "RankResult": {"schema":"=thinkthen.result/2","answer_id":"AnswerId","meta":"Meta","value":"positive","input":"json","question":"AtomicQuestion","answer":"AtomicAnswer","threshold":"null","question_name?":"str","position?":"Position","file?":"str","first_line?":"positive","last_line?":"positive","source?":"PhysicalSource","index?":"uint","members?":"[RankMember]"},
  "FindResult": {"schema":"=thinkthen.result/2","answer_id":"AnswerId","meta":"Meta","value":"json","question":"FindQuestion","answer":"FindAnswer","threshold":"null","position?":"Position","file?":"str","first_line?":"positive","last_line?":"positive","index?":"uint|null","candidates?":"[FindCandidate]"},
  "AnnotateResult": {"schema":"=thinkthen.result/2","answer_id":"AnswerId","meta":"Meta","input":"json","value":"{AnnotatedValue}","answers":"{AnnotationEntry}","position?":"Position","file?":"str","first_line?":"positive","last_line?":"positive","index?":"uint","source?":"PhysicalSource"},
  "RecognizeResult": {"schema":"=thinkthen.result/2","answer_id":"AnswerId","meta":"Meta","value":"Recognition","question":"RecognizeQuestion","answer":"RecognitionAnswer","input?":"json","file?":"str","first_line?":"positive","last_line?":"positive","index?":"uint","source?":"PhysicalSource"},
  "RelateResult": {"schema":"=thinkthen.result/2","answer_id":"AnswerId","meta":"Meta","value":"[Edge]","question":"RelateQuestion","answer":"RelationAnswer","file?":"str","first_line?":"positive","last_line?":"positive","input?":"json","index?":"uint"},
  "CallError": {"kind":"error_kind","message":"str","retryable":"bool","facts?":"Facts","attempts?":"[Attempt]","stopped?":"Stopped"},
  "DecideSpec": {"decide":"text","true?":"description","false?":"description","threshold?":"threshold","model?":"str","profile?":"str","batch?":"batch","on?":"str|[str]","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "ChooseSpec": {"choose":"text","options":"Labels","threshold?":"probability","model?":"str","profile?":"str","batch?":"batch","on?":"str|[str]","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "TagSpec": {"tag":"text","labels":"Labels","threshold?":"probability","model?":"str","profile?":"str","batch?":"batch","on?":"str|[str]","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "ScoreSpec": {"score":"text","levels":"Labels","model?":"str","profile?":"str","batch?":"batch","on?":"str|[str]","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "FindSpec": {"find":"text","none?":"bool","model?":"str","profile?":"str","on?":"str|[str]","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "QuestionFile": {"path":"str"},
  "QuestionSet": {"version":"one","questions":"{AnnotationSpec}","batch?":"batch","threshold?":"threshold","profile?":"str"},
  "RecognitionPlan": {"kinds?":"Labels","instructions?":"str","entity_definition?":"str","relations?":"[PlanRule]"},
  "PlanRule": {"name":"str","source":"str","target":"str","reads?":"str","either?":"bool","single?":"bool"},
  "RecognitionSpec": {"version":"one","recognize":"RecognitionPlan","threshold?":"probability","relation_threshold?":"probability","model?":"str","profile?":"str","on?":"str|[str]","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "RelationPlan": {"relations":"[PlanRule]","fields?":"RelateFields"},
  "RelationSpec": {"version":"one","relate":"RelationPlan","threshold?":"probability","model?":"str","profile?":"str","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "Files": {"paths":"[str]","unit":"unit","window?":"positive","media?":"media"},
  "TextInput": {"text":"json"},
  "RecordInput": {"records":"[json]","context?":"json"},
  "CandidateInput": {"units":"[json]","context?":"json"},
  "ImageBytes": {"data":"bytes","name?":"str"},
  "ImageInput": {"images":"[ImageBytes]","text?":"json"},
  "Controls": {"batch?":"batch","context?":"json","on?":"str|[str]","threshold?":"threshold","top?":"positive","none?":"bool","model?":"str","attempts?":"bool","deadline_ms?":"uint"},
  "DecideMember": {"decide":"text","true?":"description","false?":"description","threshold?":"threshold","on?":"str|[str]","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "ChooseMember": {"choose":"text","options":"Labels","threshold?":"probability","on?":"str|[str]","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "TagMember": {"tag":"text","labels":"Labels","threshold?":"probability","on?":"str|[str]","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "ScoreMember": {"score":"text","levels":"Labels","on?":"str|[str]","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "StringDeclaration": {"type":"=string"},
  "NumberDeclaration": {"type":"=number"},
  "BooleanDeclaration": {"type":"=boolean"},
  "ArrayDeclaration": {"type":"=array","items":"StringDeclaration"},
  "ObjectDeclaration": {"type":"=object","properties":"{PropertyDeclaration}","required?":"[str]"},
  "FindCandidate": {"index":"uint|null","input":"json","probability":"probability","source?":"PhysicalSource"},
  "PhysicalSource": {"file":"str","first_line?":"positive","last_line?":"positive"},
  "NativeImage": {"media":"image_media","base64":"str","width":"positive","height":"positive"},
  "Stopped": {"cause":"stop_cause","retryable":"bool","status?":"uint","at?":"uint"},
  "NativeInput": {"original":"json","location?":"PhysicalSource","images":"[NativeImage]"}
};

const aliases = {
  "SuccessValue": ["bool", "null", "str", "number", "[str]"],
  "Observation": ["Observed", "FailedObservation"],
  "AtomicAnswer": ["YesNo", "Choice", "Tags", "Score", "FindAnswer"],
  "AtomicQuestion": ["DecideQuestion", "ChooseQuestion", "TagQuestion", "ScoreQuestion", "FindQuestion"],
  "AnnotationEntry": ["AnnotationSuccess", "AnnotationFailure"],
  "RelationEntry": ["RelationSuccess", "RelationFailure"],
  "Result": ["DecideResult", "ChooseResult", "TagResult", "ScoreResult", "FilterResult", "RankResult", "FindResult", "AnnotateResult", "RecognizeResult", "RelateResult"],
  "AnnotatedValue": ["bool", "null", "str", "number", "[str]", "FailedField"],
};
const enums = {
  "origin": ["live", "cache", "replay", "proxy", "memory"],
  "outcome": ["ok", "status", "transport"],
  "cause": ["missing_answer", "wrong_kind", "missing_probability", "invalid_probability", "invalid_distribution", "unexpected_probability"],
  "error_kind": ["usage", "backend", "local", "cancelled", "deadline", "defect"],
};
const ids = ["CallId", "SdkRequestId", "ObservationId", "FailureId", "AnswerId", "Digest"];
const hidden = Symbol.for('nodejs.util.inspect.custom');
const invalid = () => { throw new TypeError('invalid complete result'); };
const object = v => v !== null && typeof v === 'object' && !Array.isArray(v) && [Object.prototype, null].includes(Object.getPrototypeOf(v));
const has = (v, k) => Object.hasOwn(v, k);
function frozen(v) {
  Object.defineProperty(v, hidden, { value: () => '<complete carrier: content withheld>' });
  return Object.freeze(v);
}
function json(v) {
  if (v === null || typeof v === 'string' || typeof v === 'boolean' || (typeof v === 'number' && Number.isFinite(v))) return v;
  if (Array.isArray(v)) return Object.freeze(v.map(json));
  if (object(v)) return frozen(Object.fromEntries(Object.entries(v).map(([k, x]) => [k, json(x)])));
  return invalid();
}
function identity(value) {
  if (typeof value !== 'string' || !/^[0-9a-f]{64}$/.test(value)) return invalid();
  return value;
}
function decode(kind, value, preserveUnknown = false) {
  if (aliases[kind] || kind.includes('|')) {
    const variants = aliases[kind] ?? kind.split('|');
    if (preserveUnknown && object(value)) {
      const identities = variants.map(v => Object.entries(models[v] ?? {}).filter(([, t]) => ids.includes(t)).map(([k]) => k.replace(/\?$/, '')));
      const shared = identities[0].filter(k => identities.every(keys => keys.includes(k)));
      if (identities.filter(keys => keys.some(k => !shared.includes(k) && has(value, k))).length > 1) return invalid();
    }
    for (const one of variants) {
      try { return decode(one, value, preserveUnknown); } catch (error) { if (!(error instanceof TypeError)) throw error; }
    }
    return invalid();
  }
  if (kind.startsWith('[')) {
    if (!Array.isArray(value)) return invalid();
    return Object.freeze(value.map(x => decode(kind.slice(1, -1), x, preserveUnknown)));
  }
  if (kind.startsWith('{')) {
    if (!object(value)) return invalid();
    return frozen(Object.fromEntries(Object.entries(value).map(([k, v]) => [k, decode(kind.slice(1, -1), v, preserveUnknown)])));
  }
  if (models[kind]) {
    const shape = models[kind];
    const names = Object.keys(shape).map(k => k.replace(/\?$/, ''));
    if (!object(value) || (!preserveUnknown && Reflect.ownKeys(value).some(k => k !== hidden && !names.includes(k)))) return invalid();
    const held = preserveUnknown ? Object.fromEntries(Object.entries(value).filter(([k]) => !names.includes(k)).map(([k, v]) => [k, json(v)])) : {};
    for (const [key, type] of Object.entries(shape)) {
      const name = key.replace(/\?$/, '');
      if (!has(value, name)) { if (!key.endsWith('?')) return invalid(); continue; }
      const item = decode(type, value[name], preserveUnknown);
      Object.defineProperty(held, name, type === 'bytes'
        ? { get: () => new Uint8Array(item), enumerable: true }
        : { value: item, enumerable: true });
    }
    check(kind, value);
    return frozen(held);
  }
  if (kind === 'one') return value === 1 ? value : invalid();
  if (kind === 'bytes') return value instanceof Uint8Array ? new Uint8Array(value) : invalid();
  if (enums[kind]) return enums[kind].includes(value) ? value : invalid();
  if (ids.includes(kind)) return identity(value);
  if (kind.startsWith('=')) return value === (kind === '=true' ? true : kind.slice(1)) ? value : invalid();
  if (['json', 'description', 'text'].includes(kind)) {
    if (['description', 'text'].includes(kind) && value !== null && typeof value !== 'string' && !Array.isArray(value) && !object(value)) return invalid();
    if (kind === 'text' && value === null) return invalid();
    return json(value);
  }
  if (kind === 'null') return value === null ? null : invalid();
  if (kind === 'bool') return typeof value === 'boolean' ? value : invalid();
  if (kind === 'str') return typeof value === 'string' ? value : invalid();
  if (['uint', 'positive', 'batch'].includes(kind)) {
    if (kind === 'batch' && value === 'max') return value;
    return Number.isSafeInteger(value) && value >= (kind === 'uint' ? 0 : 1) ? value : invalid();
  }
  if (['number', 'probability', 'threshold'].includes(kind)) {
    if (typeof value === 'number' && Number.isFinite(value) && (kind === 'number' || (value <= 1 && (kind === 'probability' ? value >= 0 : value > 0)))) return value;
    if (kind === 'threshold') {
      if (value === null) return null;
      if (typeof value === 'string' && /^(?:0(?:\.[0-9]+)?|1(?:\.0+)?):(?:0(?:\.[0-9]+)?|1(?:\.0+)?)$/.test(value)) {
        const [low, high] = value.split(':').map(Number);
        if (low < high) return value;
      }
    }
    return invalid();
  }
  if (kind === 'cost' && typeof value === 'string' && /^[0-9]+\.[0-9]{6}$/.test(value)) return value;
  return invalid();
}
function check(kind, v) {
  const cutOnly = ['ChooseSpec','TagSpec','ChooseMember','TagMember','RecognitionSpec','RelationSpec','TagResult','FilterResult','RecognizeQuestion','RelateQuestion'];
  if (cutOnly.includes(kind)) for (const key of ['threshold', 'relation_threshold']) {
    if (has(v, key) && (typeof v[key] !== 'number' || v[key] <= 0 || v[key] > 1)) invalid();
  }
  if (kind === 'ChooseResult' && v.threshold !== null && typeof v.threshold !== 'number') invalid();
  if (kind === 'Meta') {
    if (v.requests.length !== v.question_sources.length || v.requests.length !== v.observations.length || has(v, 'question_sha256') === has(v, 'questions_sha256')) invalid();
    if (v.question_sources.some(s => !['live', 'cache', 'replay'].includes(s.origin))) invalid();
    if (!v.question_sources.length) {
      if (v.origin !== null || v.cached || v.requests_sent || has(v, 'answered_by')) invalid();
    } else {
      const origins = v.question_sources.map(s => s.origin);
      const origin = origins.includes('live') ? 'live' : origins.includes('replay') ? 'replay' : 'cache';
      if (v.origin !== origin || v.cached !== (origin !== 'live')) invalid();
      const names = new Set(v.question_sources.map(s => s.answered_by));
      if (names.size === 1 ? v.answered_by !== [...names][0] : has(v, 'answered_by')) invalid();
    }
    if (v.failed_questions !== v.observations.filter(o => has(o, 'failure_id')).length) invalid();
  }
  if (['Span', 'NameOdds', 'PieceOdds', 'Entity'].includes(kind)) {
    if (v.end < v.start || (kind === 'Entity' && v.length !== v.end - v.start)) invalid();
  }
  if (kind === 'Position' && (has(v, 'first') !== has(v, 'last') || (has(v, 'first') && (!has(v, 'file') || v.last < v.first)))) invalid();
  if (['Entity', 'Endpoint'].includes(kind) && (has(v, 'first_line') !== has(v, 'last_line') || (has(v, 'first_line') && (!has(v, 'file') || v.last_line < v.first_line)))) invalid();
  if (kind === 'Usage' && Object.keys(v).length===0) invalid();
  if (kind === 'RankResult' && 'members' in v && (!v.members.length || !('question_name' in v) || v.question.verb!=='decide' || v.answer.kind!=='yes_no')) invalid();
  if (kind === 'RankResult' && !['yes_no', 'score'].includes(v.answer.kind)) invalid();
  if (['DecideResult', 'FilterResult'].includes(kind) && v.threshold === null) invalid();
  if (kind.endsWith('Result') && (kind === 'AnnotateResult') !== has(v.meta, 'questions_sha256')) invalid();
}
module.exports = { decode, ...Object.fromEntries(ids.map(name => [name, identity])) };

Object.assign(aliases, {
  "Labels": ["[str]", "{description}"],
  "QuestionSpec": ["DecideSpec", "ChooseSpec", "TagSpec", "ScoreSpec"],
  "RankSpec": ["DecideSpec", "ScoreSpec", "QuestionSet", "QuestionFile"],
  "Selection": ["TextInput", "RecordInput", "CandidateInput", "ImageInput", "Files"],
});
Object.assign(enums, {
  "unit": ["line", "window", "file"],
  "media": ["text", "image"],
});

aliases.AnnotationSpec = ["DecideMember", "ChooseMember", "TagMember", "ScoreMember"];

Object.assign(aliases, {InputDeclaration:["StringDeclaration","ObjectDeclaration"], PropertyDeclaration:["StringDeclaration","NumberDeclaration","BooleanDeclaration","ArrayDeclaration"]});
Object.assign(enums, {image_media:["image/png","image/jpeg"],stop_cause:["usage","local","no_key","transport","status","too_large","reply","backend","cancelled","deadline","defect"]});

models.NativeInput = {"original": "json", "location?": "PhysicalSource", "images": "[NativeImage]"};

models.RelateResult["input?"]="json";
