# Quick Fix: prepare the installed Windows Python fixtures

The native Windows run on `c7cf3890c` built both production and probe wheels. Its installed Python suite had 116 passes and these four failures:

- `test_files.py::test_all_functions_keep_the_shared_documents_and_source_coordinates`
- `test_files.py::test_published_ten_function_script_replays_original_sources_without_requests`
- `test_named_backends.py::test_constructor_backend_errors_send_nothing`
- `test_named_backends.py::test_selected_setup_survives_overrides_and_pickle_recaptures_keys`

The file tests lacked their copied documents, questions, published script, golden and replay file. The backend-name expectation already has its reviewed seven-name correction in `efb19ae2d`. The setup/pickle fixture had shared Windows permissions, so the engine correctly refused its configuration.

The runner now copies only those missing shared inputs into its owned scratch. It still copies no source package and checks both installed import paths. Both setup fixtures share the existing real PowerShell ACL operation. It assigns the current owner and protects the DACL with grants only to that user and SYSTEM. Production privacy checks, fake-key recapture, request counts and secrecy assertions stay intact. The published golden adjusts only the two folder-produced `file` spellings on Windows, including entity and relation source positions. Explicit window paths and every record, value and coordinate remain pinned. The Python test counter grows 13 lines to 5,268.

The runner's actual copy statements completed in an isolated Linux scratch. Required files matched their sources and no source package appeared. Five tests passed against the previously built installed wheel: the four cases above and the shared setup-profile test. Syntax, whitespace and the Python ratchet passed. The run used a 2 GiB memory limit with no swap. Earlier memory, ABI and wheel-content checks were reused without repetition. Linux cannot qualify the changed Windows ACL execution or native golden spellings. Fresh whole-change High review and actual Windows qualification remain for the coordinator.
