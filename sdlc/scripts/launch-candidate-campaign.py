#!/usr/bin/env python3
"""Launch future frozen candidate owners only with completed same-SHA gates."""
import argparse
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('campaign_owner', HERE / 'campaign-owner.py')
owner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(owner)


def native_receipt(path, sha, language):
    receipt = owner.read(path)
    if receipt.get('head_sha') != sha or receipt.get('conclusion') != 'success' or receipt.get('exit') != 0 or receipt.get('completed_log_exit') != 0:
        raise RuntimeError('need completed successful same-SHA ' + language + ' receipt')
    jobs = receipt.get('jobs', [])
    required = 'foundation' if language == 'objective-c' else 'swift'
    selected = [j for j in jobs if j.get('name') == required]
    if len(selected) != 1 or selected[0].get('conclusion') != 'success' or selected[0].get('failed_steps'):
        raise RuntimeError('required native SDK/runtime job did not pass')
    if 'macos-15' not in selected[0].get('labels', []):
        raise RuntimeError('missing own-platform Apple ARM native job')
    for job in jobs:
        if job.get('failed_steps') or 'self-hosted' in [v.lower() for v in job.get('labels', [])]:
            raise RuntimeError('native job failed or used self-hosted runner')
        if job.get('conclusion') != 'success' and not (job.get('name') == 'website' and job.get('conclusion') == 'skipped'):
            raise RuntimeError('only optional website job may skip')
    return {'path': str(path.resolve()), 'receipt': receipt}


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('sha')
    p.add_argument('tag', type=owner.candidate_tag)
    p.add_argument('--swift-receipt', type=Path, required=True)
    p.add_argument('--objective-c-receipt', type=Path, required=True)
    p.add_argument('--run', action='store_true', required=True)
    a = p.parse_args()
    owner.guard(a.sha, a.tag)
    if owner.COVERAGE.exists():
        p.error('canonical coverage-candidate already exists; preserve it')
    native = {language: native_receipt(path, a.sha, language) for language, path in
              (('swift', a.swift_receipt), ('objective-c', a.objective_c_receipt))}
    owner.gate_inputs(a.sha)
    folders = {kind: owner.campaign_folder(a.sha, a.tag, kind) for kind in ('local', 'hosted')}
    if any(folder.exists() for folder in folders.values()):
        raise RuntimeError('campaign output already exists; preserve receipts')
    for kind, folder in folders.items():
        folder.mkdir(mode=0o700)
        owner.atomic(folder / 'native-proof.json', {'sha': a.sha, 'tag': a.tag, 'exit': 0, 'scope': 'ARM macOS own-platform native Swift/Foundation checks; release matrix and coverage remain required', 'proof': native})
    started = {}
    children = []
    try:
        for kind in ('hosted', 'local'):
            owner.guard(a.sha, a.tag)
            folder = folders[kind]
            with (folder / 'supervisor.log').open('wb') as log:
                child = subprocess.Popen([sys.executable, '-u', str(HERE / 'campaign-owner.py'), kind, a.sha, a.tag], cwd=owner.ROOT,
                                         stdout=log, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL, start_new_session=True)
            children.append(child)
            (folder / 'launch.pid').write_text(str(child.pid) + '\n')
            started[kind] = {'pid': child.pid, 'output': str(folder), 'result': str(folder / 'Result.json')}
    except BaseException:
        for child in children:
            child.terminate()
        for child in children:
            try:
                child.wait(timeout=30)
            except subprocess.TimeoutExpired:
                child.kill()
                child.wait()
        raise
    print(json.dumps({'sha': a.sha, 'tag': a.tag, 'owners': started}, indent=2))


if __name__ == '__main__':
    main()
