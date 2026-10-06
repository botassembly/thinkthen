# QuickFix: portable DuckDB file refusal fixtures

The hosted Mac ARM smoke, job 112085010615, reached two fixture construction failures before the file reader ran. macOS rejected creation of a non-UTF-8 filename with EILSEQ and refused the deeply nested manifest fixture with ENAMETOOLONG. Neither result established a production reader defect.

The manifest fixture now uses two shallow directories and enough 200-byte filenames to exceed the unchanged 16 MiB encoded-path cap. Its exact refusal and zero-request assertions stay intact. The invalid-name case still exercises the exact reader refusal on filesystems that accept the filename. Only Darwin EILSEQ during fixture creation takes the host-refusal path; it asserts zero requests and explicitly reports the reader check as not run. Other creation errors fail the case. No production code or Windows qualification changes.

All seven existing file-suite cases passed on Linux with DuckDB 1.5.5 and the already-built extension, including the actual reader refusals for invalid UTF-8 and oversized manifests. The lane's older extension lacked the reader and failed catalog lookup; the passing run reused the qualified extension from the other lane without rebuilding production code. Python syntax and diff checks passed. Real Mac qualification requires the authorized hosted rerun.
