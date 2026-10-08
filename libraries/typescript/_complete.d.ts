// Private result/2 integration types; no installed complete execution door.
export type JsonValue = null | boolean | number | string | readonly JsonValue[] | { readonly [key: string]: JsonValue };
export type QuestionText = string | readonly JsonValue[] | Readonly<Record<string, JsonValue>>;
export type Description = QuestionText | null;
export type CallId = string & { readonly __CallId: unique symbol };
export function CallId(value: string): CallId;
export type SdkRequestId = string & { readonly __SdkRequestId: unique symbol };
export function SdkRequestId(value: string): SdkRequestId;
export type ObservationId = string & { readonly __ObservationId: unique symbol };
export function ObservationId(value: string): ObservationId;
export type FailureId = string & { readonly __FailureId: unique symbol };
export function FailureId(value: string): FailureId;
export type AnswerId = string & { readonly __AnswerId: unique symbol };
export function AnswerId(value: string): AnswerId;
export type Digest = string & { readonly __Digest: unique symbol };
export function Digest(value: string): Digest;
export interface Position {
  readonly file?: string;
  readonly first?: number;
  readonly last?: number;
  readonly images?: readonly (string)[];
}
export interface Usage {
  readonly input_tokens?: number;
  readonly output_tokens?: number;
}
export interface ProfileWarning {
  readonly tuned_for: string;
  readonly running: string;
}
export interface BatchWarning {
  readonly tuned_for: number | "max";
  readonly running: number | "max";
}
export interface Attempt {
  readonly ordinal: number;
  readonly request_sha256: Digest;
  readonly wall_ms: number;
  readonly outcome: "ok" | "status" | "transport";
  readonly sdk_request_id: SdkRequestId;
  readonly status?: number;
  readonly server_ms?: number;
  readonly request_id?: string;
}
export interface Facts {
  readonly call_id: CallId;
  readonly records: number;
  readonly requests_sent: number;
  readonly cache_answers: number;
  readonly seconds: number;
  readonly input_tokens?: number;
  readonly output_tokens?: number;
  readonly model?: string;
  readonly estimated_cost_usd?: string;
  readonly command_ms?: number;
  readonly attempts?: readonly (Attempt)[];
  readonly held_model_mismatch?: boolean;
}
export interface QuestionSource {
  readonly origin: "live" | "cache" | "replay" | "proxy" | "memory";
  readonly answered_by: string;
  readonly batch_size?: number;
}
export interface Observed {
  readonly observation_id: ObservationId;
}
export interface FailedObservation {
  readonly failure_id: FailureId;
}
export interface Meta {
  readonly tool: string;
  readonly url: string;
  readonly model: string;
  readonly requests_sent: number;
  readonly cached: boolean;
  readonly requests: readonly (Digest)[];
  readonly failed_questions: number;
  readonly origin: "live" | "cache" | "replay" | "proxy" | "memory" | null;
  readonly question_sources: readonly (QuestionSource)[];
  readonly observations: readonly (Observation)[];
  readonly question_sha256?: Digest;
  readonly questions_sha256?: Digest;
  readonly answered_by?: string;
  readonly usage?: Usage;
  readonly profile_warning?: ProfileWarning;
  readonly batch_setting?: number | "max";
  readonly batch_warning?: BatchWarning;
  readonly context_sha256?: Digest;
  readonly attempts?: readonly (Attempt)[];
}
export interface YesNo {
  readonly kind: "yes_no";
  readonly probability: number;
}
export interface Choice {
  readonly kind: "choice";
  readonly pick: string;
  readonly probabilities: Readonly<Record<string, number>>;
  readonly confidence?: number;
}
export interface Tags {
  readonly kind: "tag";
  readonly probabilities: Readonly<Record<string, number>>;
}
export interface Score {
  readonly kind: "score";
  readonly level: string;
  readonly probabilities: Readonly<Record<string, number>>;
  readonly confidence?: number;
}
export interface FindAnswer {
  readonly kind: "find";
  readonly pick: string;
  readonly probabilities: Readonly<Record<string, number>>;
  readonly confidence?: number;
}
export interface DecideQuestion {
  readonly verb: "decide";
  readonly text: QuestionText;
  readonly true?: Description;
  readonly false?: Description;
  readonly name?: string;
  readonly wording_version?: number;
  readonly item_schema?: InputDeclaration;
  readonly context_schema?: InputDeclaration;
}
export interface ChooseQuestion {
  readonly verb: "choose";
  readonly text: QuestionText;
  readonly options: readonly (string)[];
  readonly name?: string;
  readonly wording_version?: number;
  readonly item_schema?: InputDeclaration;
  readonly context_schema?: InputDeclaration;
}
export interface TagQuestion {
  readonly verb: "tag";
  readonly text: QuestionText;
  readonly labels: readonly (string)[];
  readonly name?: string;
  readonly wording_version?: number;
  readonly item_schema?: InputDeclaration;
  readonly context_schema?: InputDeclaration;
}
export interface ScoreQuestion {
  readonly verb: "score";
  readonly text: QuestionText;
  readonly levels: readonly (string)[];
  readonly name?: string;
  readonly wording_version?: number;
  readonly item_schema?: InputDeclaration;
  readonly context_schema?: InputDeclaration;
}
export interface FindQuestion {
  readonly verb: "find";
  readonly text: QuestionText;
  readonly none: boolean;
  readonly name?: string;
  readonly wording_version?: number;
  readonly item_schema?: InputDeclaration;
  readonly context_schema?: InputDeclaration;
}
export interface RelationRule {
  readonly name: string;
  readonly source: string;
  readonly target: string;
  readonly reads: string;
  readonly either: boolean;
  readonly single?: boolean;
}
export interface RelateFields {
  readonly name: string;
  readonly kind: string;
}
export interface RelateQuestion {
  readonly verb: "relate";
  readonly fields: RelateFields | null;
  readonly relations: readonly (RelationRule)[];
  readonly threshold: number | string | null;
  readonly profile?: string;
  readonly name?: string;
  readonly wording_version?: number;
  readonly item_schema?: InputDeclaration;
  readonly context_schema?: InputDeclaration;
}
export interface RecognizeQuestion {
  readonly instructions?: string;
  readonly entity_definition?: string;
  readonly verb: "recognize";
  readonly kinds: Readonly<Record<string, Description>>;
  readonly relations?: readonly (RelationRule)[];
  readonly threshold: number | string | null;
  readonly relation_threshold: number | string | null;
  readonly on?: string | readonly string[];
  readonly profile?: string;
  readonly name?: string;
  readonly wording_version?: number;
  readonly item_schema?: InputDeclaration;
  readonly context_schema?: InputDeclaration;
}
export interface Failure {
  readonly kind: "backend";
  readonly cause: "missing_answer" | "wrong_kind" | "missing_probability" | "invalid_probability" | "invalid_distribution" | "unexpected_probability";
}
export interface FailedField {
  readonly failed: Failure;
}
export interface Entity {
  readonly text: string;
  readonly start: number;
  readonly end: number;
  readonly length: number;
  readonly kind: string;
  readonly strength: number;
  readonly file?: string;
  readonly first_line?: number;
  readonly last_line?: number;
}
export interface Endpoint {
  readonly name: string;
  readonly kind: string;
  readonly record?: JsonValue;
  readonly file?: string;
  readonly first_line?: number;
  readonly last_line?: number;
}
export interface Edge {
  readonly relation: string;
  readonly source: Endpoint;
  readonly target: Endpoint;
  readonly probability: number;
  readonly either?: true;
}
export interface EntityEdge {
  readonly relation: string;
  readonly source: Entity;
  readonly target: Entity;
  readonly probability: number;
  readonly either?: true;
}
export interface Recognition {
  readonly entities: readonly (Entity)[];
  readonly relations?: readonly (EntityEdge)[];
}
export interface PieceOdds {
  readonly start: number;
  readonly end: number;
  readonly tags: Readonly<Record<string, number>>;
}
export interface NameOdds {
  readonly start: number;
  readonly end: number;
  readonly kinds: Readonly<Record<string, number>> | null;
  readonly edges: Readonly<Record<string, number>> | null;
}
export interface Span {
  readonly start: number;
  readonly end: number;
}
export interface PairOdds {
  readonly relation: string;
  readonly source: Span;
  readonly target: Span;
  readonly probability: number;
}
export interface RecognitionAnswer {
  readonly pieces: readonly (PieceOdds)[];
  readonly names: readonly (NameOdds)[];
  readonly pairs: readonly (PairOdds)[];
}
export interface AnnotationSuccess {
  readonly answer_id: AnswerId;
  readonly value: SuccessValue;
  readonly question: AtomicQuestion;
  readonly answer: AtomicAnswer;
  readonly threshold: number | string | null;
  readonly request: Digest;
}
export interface AnnotationFailure {
  readonly failure_id: FailureId;
  readonly question: AtomicQuestion;
  readonly failure: Failure;
  readonly request: Digest;
}
export interface RelationSuccess {
  readonly relation: string;
  readonly reads: string;
  readonly method: string;
  readonly direction: string;
  readonly source: Endpoint;
  readonly target: Endpoint | null;
  readonly request: Digest;
  readonly answer_id: AnswerId;
  readonly probability: number;
  readonly accepted: boolean;
  readonly answer?: AtomicAnswer;
}
export interface RelationFailure {
  readonly relation: string;
  readonly reads: string;
  readonly method: string;
  readonly direction: string;
  readonly source: Endpoint;
  readonly target: Endpoint | null;
  readonly request: Digest;
  readonly failure_id: FailureId;
  readonly failure: Failure;
}
export interface RelationAnswer {
  readonly questions: readonly (RelationEntry)[];
}
export interface DecideResult {
  readonly schema: "thinkthen.result/2";
  readonly answer_id: AnswerId;
  readonly meta: Meta;
  readonly value: boolean | Description;
  readonly question: DecideQuestion;
  readonly answer: YesNo;
  readonly threshold: number | string | null;
  readonly input?: JsonValue;
  readonly position?: Position;
  readonly input_file?: string;
  readonly file?: string;
  readonly first_line?: number;
  readonly last_line?: number;
  readonly source?: PhysicalSource;
  readonly images?: readonly NativeImage[];
  readonly index?: number;
}
export interface ChooseResult {
  readonly schema: "thinkthen.result/2";
  readonly answer_id: AnswerId;
  readonly meta: Meta;
  readonly value: string | null;
  readonly question: ChooseQuestion;
  readonly answer: Choice;
  readonly threshold: number | string | null;
  readonly input?: JsonValue;
  readonly position?: Position;
  readonly input_file?: string;
  readonly file?: string;
  readonly first_line?: number;
  readonly last_line?: number;
  readonly source?: PhysicalSource;
  readonly images?: readonly NativeImage[];
  readonly index?: number;
}
export interface TagResult {
  readonly schema: "thinkthen.result/2";
  readonly answer_id: AnswerId;
  readonly meta: Meta;
  readonly value: readonly (string)[];
  readonly question: TagQuestion;
  readonly answer: Tags;
  readonly threshold: number | string | null;
  readonly input?: JsonValue;
  readonly position?: Position;
  readonly input_file?: string;
  readonly file?: string;
  readonly first_line?: number;
  readonly last_line?: number;
  readonly source?: PhysicalSource;
  readonly index?: number;
}
export interface ScoreResult {
  readonly schema: "thinkthen.result/2";
  readonly answer_id: AnswerId;
  readonly meta: Meta;
  readonly value: number;
  readonly question: ScoreQuestion;
  readonly answer: Score;
  readonly threshold: null;
  readonly input?: JsonValue;
  readonly position?: Position;
  readonly input_file?: string;
  readonly file?: string;
  readonly first_line?: number;
  readonly last_line?: number;
  readonly source?: PhysicalSource;
  readonly images?: readonly NativeImage[];
  readonly index?: number;
}
export interface FilterResult {
  readonly schema: "thinkthen.result/2";
  readonly answer_id: AnswerId;
  readonly meta: Meta;
  readonly value: boolean;
  readonly input: JsonValue;
  readonly question: DecideQuestion;
  readonly answer: YesNo;
  readonly threshold: number | string | null;
  readonly position?: Position;
  readonly file?: string;
  readonly first_line?: number;
  readonly last_line?: number;
  readonly source?: PhysicalSource;
  readonly index?: number;
}
export interface RankMemberResult {
  readonly schema: "thinkthen.result/2";
  readonly answer_id: AnswerId;
  readonly value: number;
  readonly question: DecideQuestion;
  readonly answer: YesNo;
  readonly threshold: null;
  readonly meta: Meta;
  readonly source?: PhysicalSource;
}
export interface RankMember {
  readonly name: string;
  readonly result: RankMemberResult;
}
export interface RankResult {
  readonly schema: "thinkthen.result/2";
  readonly answer_id: AnswerId;
  readonly meta: Meta;
  readonly value: number;
  readonly input: JsonValue;
  readonly question: AtomicQuestion;
  readonly answer: AtomicAnswer;
  readonly threshold: null;
  readonly question_name?: string;
  readonly position?: Position;
  readonly file?: string;
  readonly first_line?: number;
  readonly last_line?: number;
  readonly source?: PhysicalSource;
  readonly index?: number;
  readonly members?: readonly RankMember[];
}
export interface FindResult {
  readonly schema: "thinkthen.result/2";
  readonly answer_id: AnswerId;
  readonly meta: Meta;
  readonly value: JsonValue;
  readonly question: FindQuestion;
  readonly answer: FindAnswer;
  readonly threshold: null;
  readonly position?: Position;
  readonly file?: string;
  readonly first_line?: number;
  readonly last_line?: number;
  readonly index?: number | null;
  readonly candidates?: readonly FindCandidate[];
}
export interface AnnotateResult {
  readonly schema: "thinkthen.result/2";
  readonly answer_id: AnswerId;
  readonly meta: Meta;
  readonly input: JsonValue;
  readonly value: Readonly<Record<string, AnnotatedValue>>;
  readonly answers: Readonly<Record<string, AnnotationEntry>>;
  readonly position?: Position;
  readonly file?: string;
  readonly first_line?: number;
  readonly last_line?: number;
  readonly index?: number;
  readonly source?: PhysicalSource;
}
export interface RecognizeResult {
  readonly schema: "thinkthen.result/2";
  readonly answer_id: AnswerId;
  readonly meta: Meta;
  readonly value: Recognition;
  readonly question: RecognizeQuestion;
  readonly answer: RecognitionAnswer;
  readonly input?: JsonValue;
  readonly file?: string;
  readonly first_line?: number;
  readonly last_line?: number;
  readonly index?: number;
  readonly source?: PhysicalSource;
}
export interface RelateResult {
  readonly input?: JsonValue;
  readonly schema: "thinkthen.result/2";
  readonly answer_id: AnswerId;
  readonly meta: Meta;
  readonly value: readonly (Edge)[];
  readonly question: RelateQuestion;
  readonly answer: RelationAnswer;
  readonly file?: string;
  readonly first_line?: number;
  readonly last_line?: number;
  readonly index?: number;
}
export interface CallError {
  readonly kind: "usage" | "backend" | "local" | "cancelled" | "deadline" | "defect";
  readonly message: string;
  readonly retryable: boolean;
  readonly facts?: Facts;
  readonly attempts?: readonly (Attempt)[];
  readonly stopped?: Stopped;
}
export type Observation = Observed | FailedObservation;
export type AtomicAnswer = YesNo | Choice | Tags | Score | FindAnswer;
export type AtomicQuestion = DecideQuestion | ChooseQuestion | TagQuestion | ScoreQuestion | FindQuestion;
export type AnnotationEntry = AnnotationSuccess | AnnotationFailure;
export type RelationEntry = RelationSuccess | RelationFailure;
export type Result = DecideResult | ChooseResult | TagResult | ScoreResult | FilterResult | RankResult | FindResult | AnnotateResult | RecognizeResult | RelateResult;
export type SuccessValue = boolean | null | string | number | readonly string[];
export type AnnotatedValue = boolean | null | string | number | readonly (string)[] | FailedField;
export function decode<T extends keyof Models>(type: T, value: unknown): Models[T];
export interface Models {
  Position: Position;
  Usage: Usage;
  ProfileWarning: ProfileWarning;
  BatchWarning: BatchWarning;
  Attempt: Attempt;
  Facts: Facts;
  QuestionSource: QuestionSource;
  Observed: Observed;
  FailedObservation: FailedObservation;
  Meta: Meta;
  YesNo: YesNo;
  Choice: Choice;
  Tags: Tags;
  Score: Score;
  FindAnswer: FindAnswer;
  DecideQuestion: DecideQuestion;
  ChooseQuestion: ChooseQuestion;
  TagQuestion: TagQuestion;
  ScoreQuestion: ScoreQuestion;
  FindQuestion: FindQuestion;
  RelationRule: RelationRule;
  RelateFields: RelateFields;
  RelateQuestion: RelateQuestion;
  RecognizeQuestion: RecognizeQuestion;
  Failure: Failure;
  FailedField: FailedField;
  Entity: Entity;
  Endpoint: Endpoint;
  Edge: Edge;
  EntityEdge: EntityEdge;
  Recognition: Recognition;
  PieceOdds: PieceOdds;
  NameOdds: NameOdds;
  Span: Span;
  PairOdds: PairOdds;
  RecognitionAnswer: RecognitionAnswer;
  AnnotationSuccess: AnnotationSuccess;
  AnnotationFailure: AnnotationFailure;
  RelationSuccess: RelationSuccess;
  RelationFailure: RelationFailure;
  RelationAnswer: RelationAnswer;
  DecideResult: DecideResult;
  ChooseResult: ChooseResult;
  TagResult: TagResult;
  ScoreResult: ScoreResult;
  FilterResult: FilterResult;
  RankResult: RankResult;
  RankMemberResult: RankMemberResult;
  RankMember: RankMember;
  FindResult: FindResult;
  AnnotateResult: AnnotateResult;
  RecognizeResult: RecognizeResult;
  RelateResult: RelateResult;
  CallError: CallError;
  Observation: Observation;
  AtomicAnswer: AtomicAnswer;
  AtomicQuestion: AtomicQuestion;
  AnnotationEntry: AnnotationEntry;
  RelationEntry: RelationEntry;
  Result: Result;
  AnnotatedValue: AnnotatedValue;
}

export interface DecideSpec {
  readonly decide: QuestionText;
  readonly true?: Description;
  readonly false?: Description;
  readonly threshold?: number | string | null;
  readonly model?: string;
  readonly profile?: string;
  readonly batch?: number | "max";
  readonly on?: string | readonly string[];
  readonly name?: string;
  readonly wording_version?: number;
  readonly item_schema?: InputDeclaration;
  readonly context_schema?: InputDeclaration;
}
export interface ChooseSpec {
  readonly choose: QuestionText;
  readonly options: Labels;
  readonly threshold?: number;
  readonly model?: string;
  readonly profile?: string;
  readonly batch?: number | "max";
  readonly on?: string | readonly string[];
  readonly name?: string;
  readonly wording_version?: number;
  readonly item_schema?: InputDeclaration;
  readonly context_schema?: InputDeclaration;
}
export interface TagSpec {
  readonly tag: QuestionText;
  readonly labels: Labels;
  readonly threshold?: number;
  readonly model?: string;
  readonly profile?: string;
  readonly batch?: number | "max";
  readonly on?: string | readonly string[];
  readonly name?: string;
  readonly wording_version?: number;
  readonly item_schema?: InputDeclaration;
  readonly context_schema?: InputDeclaration;
}
export interface ScoreSpec {
  readonly score: QuestionText;
  readonly levels: Labels;
  readonly model?: string;
  readonly profile?: string;
  readonly batch?: number | "max";
  readonly on?: string | readonly string[];
  readonly name?: string;
  readonly wording_version?: number;
  readonly item_schema?: InputDeclaration;
  readonly context_schema?: InputDeclaration;
}
export interface FindSpec {
  readonly find: QuestionText;
  readonly none?: boolean;
  readonly model?: string;
  readonly profile?: string;
  readonly on?: string | readonly string[];
  readonly name?: string;
  readonly wording_version?: number;
  readonly item_schema?: InputDeclaration;
  readonly context_schema?: InputDeclaration;
}
export interface QuestionFile {
  readonly path: string;
}
export interface QuestionSet {
  readonly version: 1;
  readonly questions: Readonly<Record<string, AnnotationSpec>>;
  readonly batch?: number | "max";
  readonly threshold?: number | string | null;
  readonly profile?: string;
}
export interface RecognitionPlan {
  readonly instructions?: string;
  readonly entity_definition?: string;
  readonly kinds?: Labels;
  readonly relations?: readonly (PlanRule)[];
}
export interface PlanRule {
  readonly name: string;
  readonly source: string;
  readonly target: string;
  readonly reads?: string;
  readonly either?: boolean;
  readonly single?: boolean;
}
export interface RecognitionSpec {
  readonly version: 1;
  readonly recognize: RecognitionPlan;
  readonly threshold?: number;
  readonly relation_threshold?: number;
  readonly model?: string;
  readonly profile?: string;
  readonly on?: string | readonly string[];
  readonly name?: string;
  readonly wording_version?: number;
  readonly item_schema?: InputDeclaration;
  readonly context_schema?: InputDeclaration;
}
export interface RelationPlan {
  readonly relations: readonly (PlanRule)[];
  readonly fields?: RelateFields;
}
export interface RelationSpec {
  readonly version: 1;
  readonly relate: RelationPlan;
  readonly threshold?: number;
  readonly model?: string;
  readonly profile?: string;
  readonly name?: string;
  readonly wording_version?: number;
  readonly item_schema?: InputDeclaration;
  readonly context_schema?: InputDeclaration;
}
export interface Files {
  readonly paths: readonly (string)[];
  readonly unit: "line" | "window" | "file";
  readonly window?: number;
  readonly media?: "text" | "image";
}
export interface TextInput {
  readonly text: JsonValue;
}
export interface RecordInput {
  readonly records: readonly (JsonValue)[];
  readonly context?: JsonValue;
}
export interface CandidateInput {
  readonly units: readonly (JsonValue)[];
  readonly context?: JsonValue;
}
export interface ImageBytes {
  readonly data: Uint8Array;
  readonly name?: string;
}
export interface ImageInput {
  readonly images: readonly (ImageBytes)[];
  readonly text?: JsonValue;
}
export interface Controls {
  readonly batch?: number | "max";
  readonly context?: JsonValue;
  readonly on?: string | readonly string[];
  readonly threshold?: number | string | null;
  readonly top?: number;
  readonly none?: boolean;
  readonly model?: string;
  readonly attempts?: boolean;
  readonly deadline_ms?: number;
}
export type Labels = readonly (string)[] | Readonly<Record<string, Description>>;
export type QuestionSpec = DecideSpec | ChooseSpec | TagSpec | ScoreSpec;
export type RankSpec = DecideSpec | ScoreSpec | QuestionSet | QuestionFile;
export type Selection = TextInput | RecordInput | CandidateInput | ImageInput | Files;
export interface Models {
  DecideSpec: DecideSpec;
  ChooseSpec: ChooseSpec;
  TagSpec: TagSpec;
  ScoreSpec: ScoreSpec;
  FindSpec: FindSpec;
  QuestionFile: QuestionFile;
  QuestionSet: QuestionSet;
  RecognitionPlan: RecognitionPlan;
  PlanRule: PlanRule;
  RecognitionSpec: RecognitionSpec;
  RelationPlan: RelationPlan;
  RelationSpec: RelationSpec;
  Files: Files;
  TextInput: TextInput;
  RecordInput: RecordInput;
  CandidateInput: CandidateInput;
  ImageBytes: ImageBytes;
  ImageInput: ImageInput;
  Controls: Controls;
  Labels: Labels;
  QuestionSpec: QuestionSpec;
  RankSpec: RankSpec;
  Selection: Selection;
}

export interface DecideMember {
  readonly decide: QuestionText;
  readonly true?: Description;
  readonly false?: Description;
  readonly threshold?: number | string | null;
  readonly on?: string | readonly string[];
  readonly name?: string;
  readonly wording_version?: number;
  readonly item_schema?: InputDeclaration;
  readonly context_schema?: InputDeclaration;
}

export interface ChooseMember {
  readonly choose: QuestionText;
  readonly options: Labels;
  readonly threshold?: number;
  readonly on?: string | readonly string[];
  readonly name?: string;
  readonly wording_version?: number;
  readonly item_schema?: InputDeclaration;
  readonly context_schema?: InputDeclaration;
}

export interface TagMember {
  readonly tag: QuestionText;
  readonly labels: Labels;
  readonly threshold?: number;
  readonly on?: string | readonly string[];
  readonly name?: string;
  readonly wording_version?: number;
  readonly item_schema?: InputDeclaration;
  readonly context_schema?: InputDeclaration;
}

export interface ScoreMember {
  readonly score: QuestionText;
  readonly levels: Labels;
  readonly on?: string | readonly string[];
  readonly name?: string;
  readonly wording_version?: number;
  readonly item_schema?: InputDeclaration;
  readonly context_schema?: InputDeclaration;
}
export type AnnotationSpec = DecideMember | ChooseMember | TagMember | ScoreMember;
export interface Models {
  DecideMember: DecideMember;
  ChooseMember: ChooseMember;
  TagMember: TagMember;
  ScoreMember: ScoreMember;
  AnnotationSpec: AnnotationSpec;
}

export interface StringDeclaration {
  readonly type: "string";
}
export interface NumberDeclaration {
  readonly type: "number";
}
export interface BooleanDeclaration {
  readonly type: "boolean";
}
export interface ArrayDeclaration {
  readonly type: "array";
  readonly items: StringDeclaration;
}
export interface ObjectDeclaration {
  readonly type: "object";
  readonly properties: Readonly<Record<string, PropertyDeclaration>>;
  readonly required?: readonly (string)[];
}
export interface PhysicalSource {
  readonly file: string;
  readonly first_line?: number;
  readonly last_line?: number;
}
export interface NativeImage {
  readonly media: "image/png" | "image/jpeg";
  readonly base64: string;
  readonly width: number;
  readonly height: number;
}
export interface Stopped {
  readonly cause: string;
  readonly retryable: boolean;
  readonly status?: number;
  readonly at?: number;
}
export type InputDeclaration = StringDeclaration | ObjectDeclaration;
export type PropertyDeclaration = StringDeclaration | NumberDeclaration | BooleanDeclaration | ArrayDeclaration;

export interface NativeInput { readonly original: JsonValue; readonly images: readonly NativeImage[]; readonly location?: PhysicalSource; }

export interface FindCandidate {
  readonly index: number | null;
  readonly input: JsonValue;
  readonly probability: number;
  readonly source?: PhysicalSource;
}
