"""Installed scalar and details checks reuse the surface's counted fixtures."""
from __future__ import annotations

import argparse
import os
import sys
from pathlib import Path


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--extension', required=True, type=Path)
    extension = parser.parse_args().extension.resolve(strict=True)
    os.environ['THINKTHEN_DUCKDB_EXTENSION'] = str(extension)
    sys.path.insert(0,str(Path(__file__).resolve().parents[1] / 'tools'))
    from harness import run
    from native_records import check
    from portable_suite import (named_decide_binds_by_name_and_settings_refuse_before_send,
                                listed_settings_and_members_are_distinct_overloads,
                                keyed_decide_preserves_key_body_and_one_send,
                                annotate_accepts_call_settings_and_refuses_question_only_keys)
    from verbs_suite import r1_1_answers_map_back_by_text
    for case in (r1_1_answers_map_back_by_text,
                 named_decide_binds_by_name_and_settings_refuse_before_send,
                 listed_settings_and_members_are_distinct_overloads,
                 keyed_decide_preserves_key_body_and_one_send,
                 annotate_accepts_call_settings_and_refuses_question_only_keys):
        case()
    check(run)
    print('Installed scalar admission, NULL, cache, batching and typed recovery checks pass')


if __name__ == '__main__':
    main()
