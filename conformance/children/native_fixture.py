"""Retained host entry point to the shared fixture module, including producer hooks."""
import sys
import fixture_suite

sys.modules[__name__] = fixture_suite
