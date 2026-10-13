# ThinkThen for the Command Line and PHP

ThinkThen puts ten functions on top of any model: decide, choose, tag, score, filter, rank, find, annotate, recognize and relate. Your code branches on typed answers, gets not sure instead of a guess, packs many records into few requests, and caches each question so you pay once. Tests replay saved answers with no key or network. Use Jev, OpenAI or a local model from every language and database.

```sh
cargo install thinkthen
composer require botassembly/thinkthen
```

Set your hosted backend key: `export THINKTHEN_API_KEY=...`.

```sh
curl -fsSL https://github.com/botassembly/thinkthen/releases/latest/download/thinkthen-first-run.tar.gz | tar -xz && cd thinkthen-first-run
thinkthen decide 'Does this report say what the person did before the problem appeared?' --replay recording < report.txt
# true
```

```php
<?php
require 'vendor/autoload.php';

$client = new ThinkThen\Client();
try {
    $answer = $client->decide('Does this ask for a refund?', 'Please refund my order.')->decideAnswer();
    var_dump($answer);
} finally { $client->close(); }
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
