#!/usr/bin/env python3
"""Bind hosted platform checks to the dispatched candidate commit."""
import re
import subprocess
import sys


def checkout(candidate, dispatch):
    if not re.fullmatch(r'[0-9a-f]{40}', candidate):
        raise ValueError('candidate SHA must be forty lowercase hexadecimal digits')
    head = subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()
    if candidate != dispatch or candidate != head:
        raise ValueError('checkout and dispatch must equal the candidate SHA')


def main(args):
    try:
        if len(args) == 3 and args[0] == 'checkout':
            checkout(*args[1:])
        else:
            raise ValueError('use checkout SHA DISPATCH_SHA')
    except (ValueError, subprocess.CalledProcessError) as error:
        print(f'hosted-candidate: {error}', file=sys.stderr)
        return 1
    return 0


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
