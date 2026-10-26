"""Reject reserved test routing entry points in delivered native bytes."""


def check(data, label):
    # These names exist only in the explicit debug fixture module. The Rust
    # release guard also refuses the feature before any artifact can be linked.
    for name in (b'thinkthen_fixture_routes_install', b'thinkthen_fixture_routes_free'):
        if name in data:
            raise ValueError('production artifact contains test routing: ' + label)
