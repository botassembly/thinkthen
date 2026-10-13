# ThinkThen for the Command Line and PHP

ThinkThen answers typed questions about text. Use it to decide yes or no, choose a label, score an answer, or filter and rank records. ThinkThen gives code typed answers to branch on, caches each question so you pay once, and replays recorded answers offline for tests. Backends include Jev, SystemOne, OpenAI and local models.

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
