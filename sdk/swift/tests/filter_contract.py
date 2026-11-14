"""Project shared engine expectations through Swift's kept-record filter API."""
import copy


def expected_filter(case):
    rows = case['expected']
    if not isinstance(rows, list):
        return copy.deepcopy(rows)
    return {
        'kept': [copy.deepcopy(case['request']['records'][row['position'] - 1])
                 for row in rows if row['status'] == 'answered' and row['answer'] is True],
        'failed': [copy.deepcopy(row) for row in rows if row['status'] == 'failed'],
    }
