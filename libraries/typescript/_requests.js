'use strict';
// Private named builders. The engine owns question parsing and all execution.
const { decode } = require('./_complete.js');
const expected = {
  decide: ['DecideSpec', 'QuestionFile'], choose: ['ChooseSpec', 'QuestionFile'],
  tag: ['TagSpec', 'QuestionFile'], score: ['ScoreSpec', 'QuestionFile'],
  filter: ['DecideSpec', 'QuestionFile'], rank: ['RankSpec'], find: ['FindSpec', 'QuestionFile'],
  annotate: ['QuestionSet', 'QuestionFile'], recognize: ['RecognitionSpec', 'QuestionFile'],
  relate: ['RelationSpec', 'QuestionFile'],
};
function build(verb, question, input, controls = {}) {
  let held;
  for (const type of expected[verb]) {
    try { held = decode(type, question); break; } catch (error) { if (!(error instanceof TypeError)) throw error; }
  }
  if (!held) throw new TypeError('wrong question kind');
  input = decode('Selection', input);
  controls = decode('Controls', controls);
  const has = (v, k) => Object.hasOwn(v, k);
  if ((has(input, 'images') || input.media === 'image') && !['decide', 'choose', 'score'].includes(verb)) throw new TypeError('this function is text-only');
  if (has(input, 'units') && verb !== 'find') throw new TypeError('candidates require find');
  if (has(input, 'text') && !has(input, 'images') && ['filter', 'rank', 'find', 'annotate', 'relate'].includes(verb)) throw new TypeError('this function requires a complete record set');
  if (has(input, 'paths')) {
    if ((input.unit === 'window') !== has(input, 'window')) throw new TypeError('window requires window units and a size');
    if (input.media === 'image' && input.unit !== 'file') throw new TypeError('image sources require file units');
  }
  if (has(input, 'images') && !input.images.length) throw new TypeError('images require attachments');
  if (has(controls, 'top') && verb !== 'rank') throw new TypeError('top requires rank');
  if (has(controls, 'none') && verb !== 'find') throw new TypeError('none requires find');
  const request = { function: verb, question: held, input, controls };
  Object.defineProperty(request, Symbol.for('nodejs.util.inspect.custom'), { value: () => '<request: content withheld>' });
  return Object.freeze(request);
}
module.exports = Object.fromEntries(Object.keys(expected).map(verb => [verb, (question, input, controls) => build(verb, question, input, controls)]));
