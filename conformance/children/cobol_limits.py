"""Exercise the approved COBOL scalar boundary for six large-caption cases."""

def assert_scalar_boundary(row, got, sends):
    cases = {f'image-admission-{boundary}-{verb}'
             for boundary in ('body-excess', 'packing-overflow')
             for verb in ('decide', 'choose', 'score')}
    if row['id'] not in cases:
        return False
    assert got['code'] == 1002, got  # tt-n-cobol-overflow-constant
    assert sends == 0, sends
    return True
