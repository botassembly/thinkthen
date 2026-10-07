// Private integration builders. No function below executes a request.
import type { CandidateInput, ChooseSpec, Controls, DecideSpec, Files, FindSpec, ImageInput, QuestionFile, QuestionSet, RankSpec, RecognitionSpec, RecordInput, RelationSpec, ScoreSpec, TagSpec, TextInput } from './_complete.js';
export interface Request<Q, I> {
  readonly function: string;
  readonly question: Q;
  readonly input: I;
  readonly controls: Controls;
}
type Scalar = TextInput | RecordInput | Files;
type Visual = Scalar | ImageInput;
type Records = RecordInput | Files;
export function decide<Q extends DecideSpec | QuestionFile, I extends Visual>(question: Q, input: I, controls?: Controls): Request<Q, I>;
export function choose<Q extends ChooseSpec | QuestionFile, I extends Visual>(question: Q, input: I, controls?: Controls): Request<Q, I>;
export function tag<Q extends TagSpec | QuestionFile, I extends Scalar>(question: Q, input: I, controls?: Controls): Request<Q, I>;
export function score<Q extends ScoreSpec | QuestionFile, I extends Visual>(question: Q, input: I, controls?: Controls): Request<Q, I>;
export function filter<Q extends DecideSpec | QuestionFile, I extends Records>(question: Q, input: I, controls?: Controls): Request<Q, I>;
export function rank<Q extends RankSpec, I extends Records>(question: Q, input: I, controls?: Controls): Request<Q, I>;
export function find<Q extends FindSpec | QuestionFile, I extends CandidateInput | Records>(question: Q, input: I, controls?: Controls): Request<Q, I>;
export function annotate<Q extends QuestionSet | QuestionFile, I extends Records>(question: Q, input: I, controls?: Controls): Request<Q, I>;
export function recognize<Q extends RecognitionSpec | QuestionFile, I extends Scalar>(question: Q, input: I, controls?: Controls): Request<Q, I>;
export function relate<Q extends RelationSpec | QuestionFile, I extends Records>(question: Q, input: I, controls?: Controls): Request<Q, I>;
