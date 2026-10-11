# PHP generated request inputs

PHP now generates the request declarations it consumes from the existing request graph. One encoder replaces handwritten request JSON assembly and function-to-input maps. Public calls, results and cleanup remain unchanged. Native admission retains Find and Relate identifiers and input order.

## Review and proof

Fresh review found that the first encoder called JsonSerializable during validation and again during writing. A stateful serializer could change the transmitted original or introduce a cycle on its second invocation. The correction materializes each serializer result once, then validates and encodes that frozen value. Installed regressions cover the invocation count, retained value, recursive arrays and serializer-produced cycles. Raw question bytes remain preserved and encoding failures retain their UsageFailure carrier.

The reviewer accepted the correction and the PHP ceiling of 1854 nonblank lines, an increase of 289 from 1565. The growth consists of generated declarations, the shared encoder safeguards and distinct installed regressions. Unused generated constructors were removed before acceptance.

Seven installed session modes and thirteen routine fixtures passed. Generation, PHP syntax, policy and registry checks passed. The final installed archive has SHA256 `dcd883ee17bbe45fdaca91383020c0634fb0075d3f7f7a1e8ab641e6987086a3`. The original installed output existed in tool responses only; the serializer regression's original failure and corrected checks have retained filesystem logs. No filesystem log is claimed for the earlier output.

Closure lint covers the final PHP and generator changes. The unchanged core routine run from 0535 is reused with the affected installed PHP and generation checks; the final integrated uninterrupted lint and routine run belongs to 0542. All check processes were awaited.

Local evidence is retained under `target/0536-php-generated-requests/` in the main checkout, including serializer failure and passing logs, the registry logs, closure lint and the final package archive. Build evidence is not pushed.

## What the build taught us

Validation must not invoke user serializers a second time before sending. Materialize once and use that value throughout admission and encoding. Generate only the input declarations a binding consumes, and retain failures that demonstrate the correction rather than inventing missing logs.
