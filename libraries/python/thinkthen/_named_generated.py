# Generated from the canonical request schema; do not edit.
from __future__ import annotations
from os import PathLike
from typing import Literal, TypedDict, TYPE_CHECKING, Union, _SpecialForm

if TYPE_CHECKING:
    from . import CancelToken
Labels = (
    list[str]
    | dict[str, str]
    | dict[str, object]
    | type[object]
    | tuple[object, ...]
    | _SpecialForm
)
RequestBatch = Union[int, str]
DisabledCache = Literal[False]
CacheDocument = Union[str, DisabledCache]


class EngineSettings(TypedDict, total=False):
    backend: str | None
    base_url: str | None
    batch: RequestBatch | None
    cache: CacheDocument | PathLike[str] | None
    max_estimated_input_tokens_total: Union[int, None] | None
    max_request_bytes: int | None
    max_requests: Union[int, None] | None
    max_requests_total: Union[int, None] | None
    max_retries: int | None
    model: str | None
    profile: str | PathLike[str] | None
    proxy: object | None
    record: str | PathLike[str] | None
    refresh_cache: bool | None
    replay: str | PathLike[str] | None
    throttle: int | None
    timeout: int | None
    usd_per_million_input: str | None
    usd_per_million_output: str | None


class _RecognitionExampleEntityRequired(TypedDict):
    start: int
    end: int
    kind: str


class RecognitionExampleEntity(_RecognitionExampleEntityRequired, total=False):
    pass


class _RecognitionExampleTextRequired(TypedDict):
    text: str
    entities: list[RecognitionExampleEntity]


class RecognitionExampleText(_RecognitionExampleTextRequired, total=False):
    kinds: list[str]


RecognitionExample = Union[str, RecognitionExampleText]
RecognitionMode = Union[Literal["whole"], Literal["boundary_only"]]
RequestThreshold = Union[float, str]


class _RecognitionSeedSpanRequired(TypedDict):
    start: int
    end: int


class RecognitionSeedSpan(_RecognitionSeedSpanRequired, total=False):
    kind: str


class RecognitionStageContext(TypedDict, total=False):
    boundary: str
    kind_edge: str
    relation: str


class RequestOptions(TypedDict, total=False):
    attempts: bool
    batch: RequestBatch
    context: str
    context_field: str
    deadline_ms: int
    examples: list[RecognitionExample]
    examples_field: str
    field: list[str]
    files_only: bool
    max_requests_total: int
    mode: RecognitionMode
    model: str
    none: bool
    options_field: str
    relation_threshold: RequestThreshold
    seed_spans: list[RecognitionSeedSpan]
    seed_spans_field: str
    snippet_pieces: int
    stage_context: RecognitionStageContext
    threshold: RequestThreshold
    top: int


class CallOptions(RequestOptions, total=False):
    token: CancelToken
    on: str | list[str]


class DecideOptions(CallOptions, total=False):
    true: object
    false: object


class ChooseOptions(CallOptions, total=False):
    options: Labels


class TagOptions(CallOptions, total=False):
    labels: Labels


class ScoreOptions(CallOptions, total=False):
    levels: Labels | list[float]


class FilterOptions(DecideOptions):
    pass


class RankOptions(DecideOptions):
    pass


class FindOptions(CallOptions):
    pass


class AnnotateOptions(CallOptions):
    pass


class RelateOptions(CallOptions, total=False):
    relations: dict[str, tuple[str, str]] | list[tuple[str, str, str]]
    either: list[str]


class RecognizeOptions(RelateOptions, total=False):
    kinds: Labels
    descriptions: dict[str, object]
    instructions: object
    entity_definition: object


class PlanOptions(
    DecideOptions, ChooseOptions, TagOptions, ScoreOptions, RecognizeOptions
):
    pass


MISSING = object()


class NamedCalls:
    __slots__ = ()

    def __init__(
        self,
        *,
        backend=None,
        base_url=None,
        batch=None,
        cache=None,
        max_estimated_input_tokens_total=None,
        max_request_bytes=None,
        max_requests=None,
        max_requests_total=None,
        max_retries=None,
        model=None,
        profile=None,
        proxy=None,
        record=None,
        refresh_cache=None,
        replay=None,
        throttle=None,
        timeout=None,
        usd_per_million_input=None,
        usd_per_million_output=None,
    ):
        self._configure(
            dict(
                backend=backend,
                base_url=base_url,
                batch=batch,
                cache=cache,
                max_estimated_input_tokens_total=max_estimated_input_tokens_total,
                max_request_bytes=max_request_bytes,
                max_requests=max_requests,
                max_requests_total=max_requests_total,
                max_retries=max_retries,
                model=model,
                profile=profile,
                proxy=proxy,
                record=record,
                refresh_cache=refresh_cache,
                replay=replay,
                throttle=throttle,
                timeout=timeout,
                usd_per_million_input=usd_per_million_input,
                usd_per_million_output=usd_per_million_output,
            )
        )

    def decide(
        self,
        question,
        text,
        *,
        attempts=MISSING,
        batch=MISSING,
        context=MISSING,
        context_field=MISSING,
        deadline_ms=MISSING,
        examples=MISSING,
        examples_field=MISSING,
        field=MISSING,
        files_only=MISSING,
        max_requests_total=MISSING,
        mode=MISSING,
        model=MISSING,
        none=MISSING,
        options_field=MISSING,
        relation_threshold=MISSING,
        seed_spans=MISSING,
        seed_spans_field=MISSING,
        snippet_pieces=MISSING,
        stage_context=MISSING,
        threshold=MISSING,
        top=MISSING,
        token=MISSING,
        on=MISSING,
        true=MISSING,
        false=MISSING,
    ):
        controls = dict(
            attempts=attempts,
            batch=batch,
            context=context,
            context_field=context_field,
            deadline_ms=deadline_ms,
            examples=examples,
            examples_field=examples_field,
            field=field,
            files_only=files_only,
            max_requests_total=max_requests_total,
            mode=mode,
            model=model,
            none=none,
            options_field=options_field,
            relation_threshold=relation_threshold,
            seed_spans=seed_spans,
            seed_spans_field=seed_spans_field,
            snippet_pieces=snippet_pieces,
            stage_context=stage_context,
            threshold=threshold,
            top=top,
            token=token,
            on=on,
            true=true,
            false=false,
        )
        return self._named(
            "decide",
            question,
            text,
            **{key: value for key, value in controls.items() if value is not MISSING},
        )

    def choose(
        self,
        question,
        text,
        *,
        attempts=MISSING,
        batch=MISSING,
        context=MISSING,
        context_field=MISSING,
        deadline_ms=MISSING,
        examples=MISSING,
        examples_field=MISSING,
        field=MISSING,
        files_only=MISSING,
        max_requests_total=MISSING,
        mode=MISSING,
        model=MISSING,
        none=MISSING,
        options_field=MISSING,
        relation_threshold=MISSING,
        seed_spans=MISSING,
        seed_spans_field=MISSING,
        snippet_pieces=MISSING,
        stage_context=MISSING,
        threshold=MISSING,
        top=MISSING,
        token=MISSING,
        on=MISSING,
        options=MISSING,
    ):
        controls = dict(
            attempts=attempts,
            batch=batch,
            context=context,
            context_field=context_field,
            deadline_ms=deadline_ms,
            examples=examples,
            examples_field=examples_field,
            field=field,
            files_only=files_only,
            max_requests_total=max_requests_total,
            mode=mode,
            model=model,
            none=none,
            options_field=options_field,
            relation_threshold=relation_threshold,
            seed_spans=seed_spans,
            seed_spans_field=seed_spans_field,
            snippet_pieces=snippet_pieces,
            stage_context=stage_context,
            threshold=threshold,
            top=top,
            token=token,
            on=on,
            options=options,
        )
        return self._named(
            "choose",
            question,
            text,
            **{key: value for key, value in controls.items() if value is not MISSING},
        )

    def tag(
        self,
        question,
        text,
        *,
        attempts=MISSING,
        batch=MISSING,
        context=MISSING,
        context_field=MISSING,
        deadline_ms=MISSING,
        examples=MISSING,
        examples_field=MISSING,
        field=MISSING,
        files_only=MISSING,
        max_requests_total=MISSING,
        mode=MISSING,
        model=MISSING,
        none=MISSING,
        options_field=MISSING,
        relation_threshold=MISSING,
        seed_spans=MISSING,
        seed_spans_field=MISSING,
        snippet_pieces=MISSING,
        stage_context=MISSING,
        threshold=MISSING,
        top=MISSING,
        token=MISSING,
        on=MISSING,
        labels=MISSING,
    ):
        controls = dict(
            attempts=attempts,
            batch=batch,
            context=context,
            context_field=context_field,
            deadline_ms=deadline_ms,
            examples=examples,
            examples_field=examples_field,
            field=field,
            files_only=files_only,
            max_requests_total=max_requests_total,
            mode=mode,
            model=model,
            none=none,
            options_field=options_field,
            relation_threshold=relation_threshold,
            seed_spans=seed_spans,
            seed_spans_field=seed_spans_field,
            snippet_pieces=snippet_pieces,
            stage_context=stage_context,
            threshold=threshold,
            top=top,
            token=token,
            on=on,
            labels=labels,
        )
        return self._named(
            "tag",
            question,
            text,
            **{key: value for key, value in controls.items() if value is not MISSING},
        )

    def score(
        self,
        question,
        text,
        *,
        attempts=MISSING,
        batch=MISSING,
        context=MISSING,
        context_field=MISSING,
        deadline_ms=MISSING,
        examples=MISSING,
        examples_field=MISSING,
        field=MISSING,
        files_only=MISSING,
        max_requests_total=MISSING,
        mode=MISSING,
        model=MISSING,
        none=MISSING,
        options_field=MISSING,
        relation_threshold=MISSING,
        seed_spans=MISSING,
        seed_spans_field=MISSING,
        snippet_pieces=MISSING,
        stage_context=MISSING,
        threshold=MISSING,
        top=MISSING,
        token=MISSING,
        on=MISSING,
        levels=MISSING,
    ):
        controls = dict(
            attempts=attempts,
            batch=batch,
            context=context,
            context_field=context_field,
            deadline_ms=deadline_ms,
            examples=examples,
            examples_field=examples_field,
            field=field,
            files_only=files_only,
            max_requests_total=max_requests_total,
            mode=mode,
            model=model,
            none=none,
            options_field=options_field,
            relation_threshold=relation_threshold,
            seed_spans=seed_spans,
            seed_spans_field=seed_spans_field,
            snippet_pieces=snippet_pieces,
            stage_context=stage_context,
            threshold=threshold,
            top=top,
            token=token,
            on=on,
            levels=levels,
        )
        return self._named(
            "score",
            question,
            text,
            **{key: value for key, value in controls.items() if value is not MISSING},
        )

    def filter(
        self,
        question,
        records,
        *,
        attempts=MISSING,
        batch=MISSING,
        context=MISSING,
        context_field=MISSING,
        deadline_ms=MISSING,
        examples=MISSING,
        examples_field=MISSING,
        field=MISSING,
        files_only=MISSING,
        max_requests_total=MISSING,
        mode=MISSING,
        model=MISSING,
        none=MISSING,
        options_field=MISSING,
        relation_threshold=MISSING,
        seed_spans=MISSING,
        seed_spans_field=MISSING,
        snippet_pieces=MISSING,
        stage_context=MISSING,
        threshold=MISSING,
        top=MISSING,
        token=MISSING,
        on=MISSING,
        true=MISSING,
        false=MISSING,
    ):
        controls = dict(
            attempts=attempts,
            batch=batch,
            context=context,
            context_field=context_field,
            deadline_ms=deadline_ms,
            examples=examples,
            examples_field=examples_field,
            field=field,
            files_only=files_only,
            max_requests_total=max_requests_total,
            mode=mode,
            model=model,
            none=none,
            options_field=options_field,
            relation_threshold=relation_threshold,
            seed_spans=seed_spans,
            seed_spans_field=seed_spans_field,
            snippet_pieces=snippet_pieces,
            stage_context=stage_context,
            threshold=threshold,
            top=top,
            token=token,
            on=on,
            true=true,
            false=false,
        )
        return self._named(
            "filter",
            question,
            records,
            **{key: value for key, value in controls.items() if value is not MISSING},
        )

    def rank(
        self,
        question,
        records,
        *,
        attempts=MISSING,
        batch=MISSING,
        context=MISSING,
        context_field=MISSING,
        deadline_ms=MISSING,
        examples=MISSING,
        examples_field=MISSING,
        field=MISSING,
        files_only=MISSING,
        max_requests_total=MISSING,
        mode=MISSING,
        model=MISSING,
        none=MISSING,
        options_field=MISSING,
        relation_threshold=MISSING,
        seed_spans=MISSING,
        seed_spans_field=MISSING,
        snippet_pieces=MISSING,
        stage_context=MISSING,
        threshold=MISSING,
        top=MISSING,
        token=MISSING,
        on=MISSING,
        true=MISSING,
        false=MISSING,
    ):
        controls = dict(
            attempts=attempts,
            batch=batch,
            context=context,
            context_field=context_field,
            deadline_ms=deadline_ms,
            examples=examples,
            examples_field=examples_field,
            field=field,
            files_only=files_only,
            max_requests_total=max_requests_total,
            mode=mode,
            model=model,
            none=none,
            options_field=options_field,
            relation_threshold=relation_threshold,
            seed_spans=seed_spans,
            seed_spans_field=seed_spans_field,
            snippet_pieces=snippet_pieces,
            stage_context=stage_context,
            threshold=threshold,
            top=top,
            token=token,
            on=on,
            true=true,
            false=false,
        )
        return self._named(
            "rank",
            question,
            records,
            **{key: value for key, value in controls.items() if value is not MISSING},
        )

    def find(
        self,
        question,
        units,
        *,
        attempts=MISSING,
        batch=MISSING,
        context=MISSING,
        context_field=MISSING,
        deadline_ms=MISSING,
        examples=MISSING,
        examples_field=MISSING,
        field=MISSING,
        files_only=MISSING,
        max_requests_total=MISSING,
        mode=MISSING,
        model=MISSING,
        none=MISSING,
        options_field=MISSING,
        relation_threshold=MISSING,
        seed_spans=MISSING,
        seed_spans_field=MISSING,
        snippet_pieces=MISSING,
        stage_context=MISSING,
        threshold=MISSING,
        top=MISSING,
        token=MISSING,
        on=MISSING,
    ):
        controls = dict(
            attempts=attempts,
            batch=batch,
            context=context,
            context_field=context_field,
            deadline_ms=deadline_ms,
            examples=examples,
            examples_field=examples_field,
            field=field,
            files_only=files_only,
            max_requests_total=max_requests_total,
            mode=mode,
            model=model,
            none=none,
            options_field=options_field,
            relation_threshold=relation_threshold,
            seed_spans=seed_spans,
            seed_spans_field=seed_spans_field,
            snippet_pieces=snippet_pieces,
            stage_context=stage_context,
            threshold=threshold,
            top=top,
            token=token,
            on=on,
        )
        return self._named(
            "find",
            question,
            units,
            **{key: value for key, value in controls.items() if value is not MISSING},
        )

    def annotate(
        self,
        questions,
        records,
        *,
        attempts=MISSING,
        batch=MISSING,
        context=MISSING,
        context_field=MISSING,
        deadline_ms=MISSING,
        examples=MISSING,
        examples_field=MISSING,
        field=MISSING,
        files_only=MISSING,
        max_requests_total=MISSING,
        mode=MISSING,
        model=MISSING,
        none=MISSING,
        options_field=MISSING,
        relation_threshold=MISSING,
        seed_spans=MISSING,
        seed_spans_field=MISSING,
        snippet_pieces=MISSING,
        stage_context=MISSING,
        threshold=MISSING,
        top=MISSING,
        token=MISSING,
        on=MISSING,
    ):
        controls = dict(
            attempts=attempts,
            batch=batch,
            context=context,
            context_field=context_field,
            deadline_ms=deadline_ms,
            examples=examples,
            examples_field=examples_field,
            field=field,
            files_only=files_only,
            max_requests_total=max_requests_total,
            mode=mode,
            model=model,
            none=none,
            options_field=options_field,
            relation_threshold=relation_threshold,
            seed_spans=seed_spans,
            seed_spans_field=seed_spans_field,
            snippet_pieces=snippet_pieces,
            stage_context=stage_context,
            threshold=threshold,
            top=top,
            token=token,
            on=on,
        )
        return self._named(
            "annotate",
            questions,
            records,
            **{key: value for key, value in controls.items() if value is not MISSING},
        )

    def recognize(
        self,
        text,
        ask=None,
        *,
        attempts=MISSING,
        batch=MISSING,
        context=MISSING,
        context_field=MISSING,
        deadline_ms=MISSING,
        examples=MISSING,
        examples_field=MISSING,
        field=MISSING,
        files_only=MISSING,
        max_requests_total=MISSING,
        mode=MISSING,
        model=MISSING,
        none=MISSING,
        options_field=MISSING,
        relation_threshold=MISSING,
        seed_spans=MISSING,
        seed_spans_field=MISSING,
        snippet_pieces=MISSING,
        stage_context=MISSING,
        threshold=MISSING,
        top=MISSING,
        token=MISSING,
        on=MISSING,
        kinds=MISSING,
        relations=MISSING,
        either=MISSING,
        descriptions=MISSING,
        instructions=MISSING,
        entity_definition=MISSING,
    ):
        controls = dict(
            attempts=attempts,
            batch=batch,
            context=context,
            context_field=context_field,
            deadline_ms=deadline_ms,
            examples=examples,
            examples_field=examples_field,
            field=field,
            files_only=files_only,
            max_requests_total=max_requests_total,
            mode=mode,
            model=model,
            none=none,
            options_field=options_field,
            relation_threshold=relation_threshold,
            seed_spans=seed_spans,
            seed_spans_field=seed_spans_field,
            snippet_pieces=snippet_pieces,
            stage_context=stage_context,
            threshold=threshold,
            top=top,
            token=token,
            on=on,
            kinds=kinds,
            relations=relations,
            either=either,
            descriptions=descriptions,
            instructions=instructions,
            entity_definition=entity_definition,
        )
        return self._named(
            "recognize",
            ask,
            text,
            **{key: value for key, value in controls.items() if value is not MISSING},
        )

    def relate(
        self,
        entities,
        ask=None,
        *,
        attempts=MISSING,
        batch=MISSING,
        context=MISSING,
        context_field=MISSING,
        deadline_ms=MISSING,
        examples=MISSING,
        examples_field=MISSING,
        field=MISSING,
        files_only=MISSING,
        max_requests_total=MISSING,
        mode=MISSING,
        model=MISSING,
        none=MISSING,
        options_field=MISSING,
        relation_threshold=MISSING,
        seed_spans=MISSING,
        seed_spans_field=MISSING,
        snippet_pieces=MISSING,
        stage_context=MISSING,
        threshold=MISSING,
        top=MISSING,
        token=MISSING,
        on=MISSING,
        relations=MISSING,
        either=MISSING,
    ):
        controls = dict(
            attempts=attempts,
            batch=batch,
            context=context,
            context_field=context_field,
            deadline_ms=deadline_ms,
            examples=examples,
            examples_field=examples_field,
            field=field,
            files_only=files_only,
            max_requests_total=max_requests_total,
            mode=mode,
            model=model,
            none=none,
            options_field=options_field,
            relation_threshold=relation_threshold,
            seed_spans=seed_spans,
            seed_spans_field=seed_spans_field,
            snippet_pieces=snippet_pieces,
            stage_context=stage_context,
            threshold=threshold,
            top=top,
            token=token,
            on=on,
            relations=relations,
            either=either,
        )
        return self._named(
            "relate",
            ask,
            entities,
            **{key: value for key, value in controls.items() if value is not MISSING},
        )
