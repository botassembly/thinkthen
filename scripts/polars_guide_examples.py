"""Execute the dedicated Polars guide using public eager table operations."""
import subprocess

from guide_examples import IDEAL, blocks
from python_guide_examples import run_enum_compiler
from pandas_guide_examples import MESSAGES, form_policies, run_case, table_refusals


def polars_case(index, name):
    failed = name == 'failed'
    state = 'failed' if failed else 'unsure' if name == 'unsure' and index != 17 else 'answered'
    value = None if state != 'answered' else name != 'no'
    proof, output = '', repr('')
    raises = failed and index in (1, 6, 8, 10, 11, 16, 20, 21)
    if index == 0:
        proof = 'assert hasattr(pl.Series("text", []), "thinkthen")'
    elif index == 1:
        proof = f'assert is_refund.dtype == pl.Boolean\nassert is_refund.to_list() == [{name != "no"!r}]'
        output = repr(('refund queue' if name != 'no' else 'inbox') + '\n')
    elif index == 2:
        proof = f'assert is_refund["status"].to_list() == [{state!r}]\nassert is_refund["answer"].to_list() == [{value!r}]'
        output = repr('try again later: backend.status the backend answered status 503: {"error":"unavailable"}\n' if failed else 'a person reads it\n' if name == 'unsure' else 'refund queue\n' if value else 'inbox\n')
    elif index == 3:
        if failed:
            proof = '_expected = pl.DataFrame({"position": [1], "status": pl.Series(["failed"], dtype=pl.Enum([s.value for s in tt.Status])), "error_code": pl.Series(["backend.status"], dtype=pl.Enum([c.value for c in tt.ErrorCode]))})'
            from bash_guide_examples import FAILURE
            output = repr('could not judge every record: ' + FAILURE.removeprefix('failed: backend.status: ') + '\n') + ' + str(_expected) + "\\n"'
        else:
            proof = f'assert is_refund.to_list() == [{name != "no"!r}]\nassert is_refund.dtype == pl.Boolean'
            output = 'str(is_refund) + "\\n"'
    elif index == 4:
        probability = None if failed else .5 if name == 'unsure' else .05 if name == 'no' else .95
        proof = f'assert is_refund["status"].to_list() == [{state!r}]\nassert is_refund["answer"].to_list() == [{value!r}]\nassert is_refund["probability"].to_list() == [{probability!r}]'
        output = 'str(is_refund.select("status", "answer", "probability")) + "\\n"'
    elif index == 5:
        choice = None if state != 'answered' else 'shipping' if name in ('no', 'shipping') else 'other' if name == 'other' else 'billing'
        proof = f'assert topic["status"].to_list() == [{state!r}]\nassert topic["answer"].to_list() == [{choice!r}]\nassert topic["answer"].dtype == pl.Enum(["billing", "shipping", "other"])'
        output = repr('try again later: backend.status\n' if failed else 'a person reads it\n' if name == 'unsure' else {'billing': 'billing team\n', 'shipping': 'warehouse\n', 'other': 'front desk\n'}[choice])
    elif index == 6:
        proof = f'assert refund_checks.to_list() == [{value!r}] * 3\nassert refund_checks.dtype == pl.Boolean'
        output = repr(''.join(('a person reads it: ' if name == 'unsure' else 'refund queue ' if name == 'yes' else 'inbox ') + m + '\n' for m in MESSAGES))
    elif index == 7:
        proof = f'assert refund_checks["status"].to_list() == [{state!r}, "failed", {state!r}]\nassert refund_checks["position"].to_list() == [1, 2, 3]\nassert refund_checks["error_code"].to_list()[1] == "record.missing"'
        lines = []
        for position in (1, 2, 3):
            lines.append(f'try again later: {position} ' + ('record.missing' if position == 2 else 'backend.status') if position == 2 or failed else f'a person reads it: {position}' if name == 'unsure' else ('refund queue' if name == 'yes' else 'inbox') + ' ' + str(position))
        output = repr('\n'.join(lines) + '\n')
    elif index == 8:
        proof = 'assert refund_requests.to_dicts() == ' + repr([{'message': m} for m in MESSAGES] if name == 'yes' else []) + '\nassert refund_requests.schema == messages.schema'
        output = 'str(refund_requests) + "\\n"'
    elif index == 9:
        labels = None if failed else [] if name == 'no' else ['billing', 'shipping']
        proof = f'assert topics["status"].to_list() == [{"failed" if failed else "answered"!r}]\nassert topics["answer"].to_list() == [{labels!r}]\nassert topics["answer"].dtype == pl.List(pl.String)'
        proof += '\n_selected = topics.select("position", "id", "status", "answer")\n_expanded = _selected.with_columns(pl.when(pl.col("answer").list.len() == 0).then(pl.lit([None])).otherwise(pl.col("answer")).alias("answer")).explode("answer")\nassert _expanded.height == ' + str(2 if labels else 1) + '\nassert _expanded["position"].to_list() == [1] * _expanded.height'
        output = 'str(_selected) + "\\n" + str(_expanded) + "\\n"'
    elif index == 10:
        number = 0.0 if name == 'low-score' else 1.0
        proof = f'assert urgency.to_list() == [{number}]\nassert urgency.dtype == pl.Float64'
        output = 'str(urgency.round(2)) + "\\n" + str(urgency > 0.5) + "\\n"'
    elif index == 11:
        proof = 'assert urgency_rank.dtype == pl.Int64\nassert urgency_rank.to_list() == [1, 2, 3]'
        output = 'str(messages.with_columns(urgency_rank.alias("urgency_rank")).sort("urgency_rank")) + "\\n"'
    elif index == 12:
        proof = f'assert action["status"].to_list() == [{state!r}]\nassert action["answer"].dtype == pl.Struct({{"text": pl.String, "start": pl.Int64, "end": pl.Int64}})'
        if name == 'yes':
            proof += '\n_span = action["answer"].to_list()[0]\nassert _span["text"] == "Please refund my order."\nassert messages[0][_span["start"]:_span["end"]] == _span["text"]'
        else:
            proof += '\nassert action["answer"].to_list() == [None]'
        branch = {'yes': 'Please refund my order.\n', 'no': 'no line answers\n', 'unsure': 'a person reads it\n', 'failed': 'try again later: backend.status\n'}[name]
        output = repr(branch) + ' + str(action.filter(pl.col("status") == "answered").select(pl.col("answer").struct.field("text").alias("text"))) + "\\n"'
    elif index in (13, 14):
        result = 'names' if index == 13 else 'relations'
        proof = f'assert {result}["status"].to_list() == [{"failed" if failed else "answered"!r}]'
        if name != 'relations':
            proof += f'\nassert {result}["answer"].to_list() == [{None if failed else []!r}]'
        if index == 13:
            if name == 'relations':
                proof += '\nassert [(x["kind"], x["text"]) for x in names["answer"].to_list()[0]] == [("person", "Morgan"), ("organization", "Example Group")]'
            expression = 'names.select("position", "id", "status", "answer").with_columns(pl.when(pl.col("answer").list.len() == 0).then(pl.lit([None])).otherwise(pl.col("answer")).alias("answer")).explode("answer").with_columns(pl.col("answer").struct.field("kind").alias("kind"), pl.col("answer").struct.field("text").alias("text"))'
        else:
            if name == 'relations':
                proof += '\nassert [(x["source"]["text"], x["relation"], x["target"]["text"]) for x in relations["answer"].to_list()[0]] == [("Morgan", "works_for", "Example Group")]'
            expression = 'relations.select("position", "id", "status", "answer").with_columns(pl.when(pl.col("answer").list.len() == 0).then(pl.lit([None])).otherwise(pl.col("answer")).alias("answer")).explode("answer").with_columns(pl.col("answer").struct.field("source").struct.field("text").alias("source_text"), pl.col("answer").struct.field("relation").alias("relation"), pl.col("answer").struct.field("target").struct.field("text").alias("target_text"))'
        proof += '\n_expanded = ' + expression + '\nassert _expanded.height == ' + str(2 if index == 13 and name == 'relations' else 1) + '\nassert _expanded["position"].to_list() == [1] * _expanded.height'
        output = 'str(_expanded) + "\\n"'
    elif index == 15:
        child_failed = failed or name.endswith('-failed')
        unsure = name == 'unsure' or name.endswith('-unsure')
        proof = 'assert forms["status"].to_list() == ["answered"] * 3'
        for child in ('refund', 'topic', 'urgency'):
            child_state = 'failed' if failed or name == child + '-failed' else 'unsure' if name == child + '-unsure' or name == 'unsure' and child in ('refund', 'topic') else 'answered'
            proof += f'\nassert forms["{child}_status"].to_list() == [{child_state!r}] * 3'
        output = repr(''.join(('try again later: ' if child_failed else 'a person reads it: ') + str(i) + '\n' for i in (1, 2, 3)) if child_failed or unsure else '' if name == 'no' else 'refund queue: billing 1.0\n' * 3)
    elif index in (16, 20):
        proof = f'assert is_refund.dtype == pl.Boolean\nassert is_refund.to_list() == [{name != "no"!r}]'
        output = 'str(is_refund) + "\\n"'
    elif index == 17:
        proof = f'assert is_refund["status"].to_list() == [{state!r}]\nassert is_refund["id"].to_list() == [{{"kind": "string", "string": "a1", "signed": None, "unsigned": None}}]\nassert is_refund["item"].to_list() == [{{"id": "a1", "message": "Please refund my order."}}]'
        output = 'str(is_refund.select("position", "status", pl.col("id").struct.field("string").alias("record_id"))) + "\\n"'
    elif index == 18:
        first = None if failed else name != 'no'
        proof = f'assert is_refund.dtype == pl.Boolean\nassert is_refund.to_list() == [{first!r}, None]\nassert failures["position"].to_list() == { [1, 2] if failed else [2]!r}\nassert failures["error_code"].to_list() == { ["backend.status", "record.missing"] if failed else ["record.missing"]!r}'
        output = 'str(failures.select("position", "question", "error_code", "error_message")) + "\\n" + str(is_refund) + "\\n"'
    elif index == 19:
        proof = f'assert is_refund["status"].to_list() == [{state!r}]\nassert is_refund["answer"].to_list() == [{value!r}]'
        output = 'str(is_refund.select("status", "answer")) + "\\n"'
    elif index == 21:
        proof = 'assert usage.attempts == 1\nassert usage.input_tokens.reported == usage.output_tokens.reported == 0'
        output = repr('1 0 0\n')
    else:
        raise AssertionError('unaccounted Polars fence ' + str(index))
    if not raises:
        proof += f'\nassert sum(issubclass(w.category, tt.FailedRowsWarning) for w in _warnings) == {int(index == 18)}'
    return proof, output, raises


def run_polars(python, negative=None):
    examples = blocks(IDEAL / 'polars.md')
    if negative == 'refusal':
        table_refusals(python, 'polars', negative)
        raise AssertionError('negative refusal control was accepted')
    assert len(examples) == 22 and all(language == 'python' for language, _, _ in examples), 'account for every Polars fence'
    prelude = examples[0][1] + '\n'
    failures = []
    for index, (_, body, _) in enumerate(examples):
        names = ['yes', 'no', 'unsure', 'failed']
        if index in (0, 1, 3, 8, 11, 16, 18, 20, 21):
            names = ['yes'] if index == 0 else ['yes', 'no', 'failed']
        if index == 5:
            names += ['shipping', 'other']
        if index == 10:
            names = ['yes', 'low-score', 'failed']
        if index in (13, 14):
            names = ['relations', 'empty', 'failed']
        if index == 15:
            names += ['refund-failed', 'topic-failed', 'urgency-failed', 'refund-unsure', 'topic-unsure']
        for name in names:
            try:
                proof, output, raises = polars_case(index, name)
                run_case(python, 'polars', index, name, prelude + body, proof, output,
                         raises=raises, negative=negative)
            except (AssertionError, RuntimeError, OSError, ValueError, KeyError, TypeError, subprocess.SubprocessError) as error:
                if negative:
                    raise
                failures.append(f"polars/guide/{index}/{name}: {error}")
    for label, run in [
        ('forms', lambda: form_policies(python, 'polars', prelude + examples[15][1].split('\nforms =', 1)[0])),
        ('refusals', lambda: table_refusals(python, 'polars', negative)),
        ('compiler', lambda: run_enum_compiler(python, examples[5][1], 'polars')),
    ]:
        try:
            run()
        except (AssertionError, RuntimeError, OSError, ValueError, KeyError, TypeError, subprocess.SubprocessError) as error:
            if negative:
                raise
            failures.append(label + ": " + str(error))
    if failures:
        raise AssertionError("\n".join(failures))
    print('PASS Polars guide: all 22 literal fences, scenario outputs, branches and refusals')
