"""Resolve prepared images once and pass content identities to container runs."""
import os
import re
import subprocess
import sys

TAGS = {'floor': 'thinkthen-floor-build:glibc2.28', 'swift': 'thinkthen-swift-build:6.3.2-ubuntu22.04'}


def image(name):
    variable = 'THINKTHEN_' + name.upper() + '_IMAGE'
    identity = os.environ.get(variable)
    if identity is None:
        identity = subprocess.check_output(['docker', 'image', 'inspect', TAGS[name],
                                             '--format', '{{.Id}}'], text=True).strip()
    if not re.fullmatch('sha256:[0-9a-f]{64}', identity):
        raise ValueError('prepared image needs an immutable identity: ' + name)
    os.environ[variable] = identity
    return identity


if __name__ == '__main__':
    if len(sys.argv) != 2 or sys.argv[1] not in TAGS: sys.exit('usage: release_images.py floor|swift')
    print(image(sys.argv[1]))
