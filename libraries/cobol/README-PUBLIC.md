# ThinkThen for COBOL

ThinkThen answers typed questions about text. Use it to decide yes or no, choose a label, score an answer, or filter and rank records. ThinkThen gives code typed answers to branch on, caches each question so you pay once, and replays recorded answers offline for tests. Backends include Jev, SystemOne, OpenAI and local models.

Download the matching release archive, then unpack it:

```sh
tar -xzf thinkthen-cobol-0.2.0-TARGET.tar.gz
```

Replace `TARGET` with the platform name on your downloaded archive. Set your hosted backend key: `export THINKTHEN_API_KEY=...`.

The packaged `examples/blocking.cob` builds two generated question selectors and three text records. Each call blocks until the native result settles and returns an owned typed table. This excerpt reads the first yes-or-no answer and label:

```cobol
call "TT_DECIDE" using by value engine decide-question input-rows no-controls
   by reference tt-n-cobol-AnswerDecideResult tt-n-cobol-AnswerStatus
   returning status-code
if status-code = 0 and v-count of tt-n-cobol-AnswerDecideResult not = 3
   move 2 to return-code goback
end-if
if status-code = 0 and v-first-present of tt-n-cobol-AnswerDecideResult = 1
   display v-first-value of tt-n-cobol-AnswerDecideResult
end-if
call "TT_ANSWER_FREE" using
   by value v-owner of tt-n-cobol-AnswerStatus returning omitted

call "TT_CHOOSE" using by value engine choose-question input-rows no-controls
   by reference tt-n-cobol-AnswerChooseResult tt-n-cobol-AnswerStatus
   returning status-code
if status-code = 0 and v-count of tt-n-cobol-AnswerDecideResult not = 3
   move 2 to return-code goback
end-if
if status-code = 0 and v-first-present of tt-n-cobol-AnswerDecideResult = 1
   display v-first-value-text of tt-n-cobol-AnswerChooseResult
      (1:v-first-value-length of tt-n-cobol-AnswerChooseResult)
end-if
call "TT_ANSWER_FREE" using
   by value v-owner of tt-n-cobol-AnswerStatus returning omitted
```

Use the decision and choice first-answer presence indicators when a question permits an unresolved answer. Free the status owner once after reading the answer and diagnostic facts. The answer table shares that owner.

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
