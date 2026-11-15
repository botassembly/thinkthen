"""Resolve prepared images once and pass content identities to container runs."""
import os
import re
import subprocess
import sys

TAGS = {'floor': 'thinkthen-floor-build:glibc2.28', 'swift': 'thinkthen-swift-build:6.3.2-ubuntu22.04',
        'installer': 'thinkthen-installer-test:fish3.7.0-1'}


def image(name, *, require_local=True):
    variable = 'THINKTHEN_' + name.upper() + '_IMAGE'
    identity = os.environ.get(variable)
    if identity is not None and not re.fullmatch('sha256:[0-9a-f]{64}', identity):
        raise ValueError('prepared image needs an immutable identity: ' + name)
    if identity is not None and not require_local:
        return identity
    reference = identity if identity is not None else TAGS[name]
    try:
        identity = subprocess.check_output(['docker', 'image', 'inspect', reference,
                                            '--format', '{{.Id}}'], text=True).strip()
    except subprocess.CalledProcessError as error:
        raise RuntimeError('prepared image unavailable: ' + name + ' (' + reference + ')') from error
    if not re.fullmatch('sha256:[0-9a-f]{64}', identity):
        raise ValueError('prepared image needs an immutable identity: ' + name)
    os.environ[variable] = identity
    return identity


if __name__ == '__main__':
    if len(sys.argv) != 2 or sys.argv[1] not in TAGS: sys.exit('usage: release_images.py floor|swift')
    print(image(sys.argv[1]))
