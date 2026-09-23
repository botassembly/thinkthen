"""Review-5: choose, score and tag settle their question in their own verb.

Wave 5 ran the built-question kind check on every input, which parsed a
plain string as a decide question first. An empty choose question then
blamed `decide`, and a non-text question lost the verb's own sentence.

Run: ENGINE_NULL=1 python -m pytest tests/test_review5_verbs.py -q
"""

import pytest

import thinkthen as tt

TEXT = "I want a refund"
VERBS = {
    "choose": ("options", lambda q: tt.choose(q, TEXT, options=["billing", "shipping"])),
    "score": ("levels", lambda q: tt.score(q, TEXT, levels=["low", "mid", "high"])),
    "tag": ("labels", lambda q: tt.tag(q, TEXT, labels=["refund", "praise"])),
}


@pytest.mark.parametrize("verb", VERBS)
@pytest.mark.parametrize("blank", ["", "   "])
def test_a_blank_question_is_refused_in_its_own_verb(verb, blank):
    _, call = VERBS[verb]
    with pytest.raises(tt.UsageError) as raised:
        call(blank)
    assert str(raised.value) == f"the question file's `{verb}`: a question is text, not white space"


@pytest.mark.parametrize("verb", VERBS)
@pytest.mark.parametrize("wrong", [5, {"choose": "x"}])
def test_a_question_that_is_not_text_names_its_verb(verb, wrong):
    key, call = VERBS[verb]
    with pytest.raises(tt.UsageError) as raised:
        call(wrong)
    assert str(raised.value) == f"a {verb} question is text plus its {key}, or a built question"


@pytest.mark.parametrize("verb", VERBS)
def test_a_built_question_of_another_kind_is_still_refused(verb):
    _, call = VERBS[verb]
    with pytest.raises(tt.UsageError) as raised:
        call(tt.question(decide="Refund?"))
    assert str(raised.value) == (
        f"this question is a decide question; it answers decide, and handing it to {verb}"
        " would read the wrong answer shape"
    )
