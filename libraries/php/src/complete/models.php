<?php
declare(strict_types=1);
namespace ThinkThen\Complete;

interface Observation {}
interface AtomicAnswer {}
interface AtomicQuestion {}
interface AnnotationEntry {}
interface RelationEntry {}
interface Result {}
interface QuestionSpec {}
interface RankSpec {}
interface Selection {}
interface AnnotationSpec {}
enum OriginKind: string { case LIVE = "live"; case CACHE = "cache"; case REPLAY = "replay"; case PROXY = "proxy"; case MEMORY = "memory"; }
enum OutcomeKind: string { case OK = "ok"; case STATUS = "status"; case TRANSPORT = "transport"; }
enum CauseKind: string { case MISSING_ANSWER = "missing_answer"; case WRONG_KIND = "wrong_kind"; case MISSING_PROBABILITY = "missing_probability"; case INVALID_PROBABILITY = "invalid_probability"; case INVALID_DISTRIBUTION = "invalid_distribution"; case UNEXPECTED_PROBABILITY = "unexpected_probability"; }
enum FailureKind: string { case USAGE = "usage"; case BACKEND = "backend"; case LOCAL = "local"; case CANCELLED = "cancelled"; case DEADLINE = "deadline"; case DEFECT = "defect"; }
enum UnitKind: string { case LINE = "line"; case WINDOW = "window"; case FILE = "file"; }
enum MediaKind: string { case TEXT = "text"; case IMAGE = "image"; }
enum MethodKind: string { case YES_NO = "yes_no"; case CHOICE = "choice"; }
enum DirectionKind: string { case SOURCE_TO_TARGET = "source_to_target"; case EITHER = "either"; }
enum StopCauseKind: string { case USAGE = "usage"; case LOCAL = "local"; case NO_KEY = "no_key"; case TRANSPORT = "transport"; case STATUS = "status"; case TOO_LARGE = "too_large"; case REPLY = "reply"; case BACKEND = "backend"; case CANCELLED = "cancelled"; case DEFECT = "defect"; case DEADLINE = "deadline"; }

enum MediaType: string { case JPEG = 'jpeg'; case PNG = 'png'; }
