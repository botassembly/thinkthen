// The ESM face of the same package: one wrapper, two doors.
import { createRequire } from 'node:module';

const cjs = createRequire(import.meta.url)('./index.js');

export const {
  complete,
  CompleteTypes,
  ThinkThenError,
  Engine,
  question,
  questionFile,
  usage,
  failed,
  outcome,
  YES,
  NO,
  UNSURE,
  plan,
  files,
  decide,
  decide_many,
  choose_many,
  score_many,
  tag_many,
  choose,
  score,
  tag,
  filter,
  rank,
  find,
  annotate,
  details,
  recognize,
  relate,
} = cjs;
export default cjs;
