#!/usr/bin/python3
import copy
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

BASE = Path(__file__).parent
TAG = 'rc/0.2.0-rc.14'
spec = importlib.util.spec_from_file_location('campaign', BASE / 'campaign-owner.py')
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
SHA = 'fa100ce09f9e99b131a433f2f5cce9927c60ea8f'
RUN = 38691891976

def stream(pages):
    return '\n \t'.join(json.dumps(p, indent=2) for p in pages) + '\n'

class RecoveryTests(unittest.TestCase):
    def test_full_paginated_stream(self):
        jobs = [dict(id=i, run_id=RUN, head_sha=SHA) for i in range(1,252)]
        pages = [dict(total_count=len(jobs), jobs=jobs[i:i+100]) for i in range(0,len(jobs),100)]
        self.assertEqual(m.parse_job_pages(stream(pages)), jobs)
        owner=m.Owner(SHA,'hosted',tag=TAG); calls=[]
        owner.gh=lambda *args: calls.append(args) or stream(pages)
        self.assertEqual(owner.jobs(RUN),jobs)
        self.assertEqual(calls,[('api','--paginate',f'repos/{m.REPO}/actions/runs/{RUN}/jobs?per_page=100')])
        for key,bad in [('head_sha','f'*40),('run_id',RUN+1)]:
            changed=copy.deepcopy(pages); changed[-1]['jobs'][-1][key]=bad
            owner.gh=lambda *args:stream(changed)
            with self.assertRaises(RuntimeError): owner.jobs(RUN)

    def test_reject_garbage_and_bad_pages(self):
        good=dict(total_count=1,jobs=[dict(id=1)])
        invalid=['','garbage'+stream([good]),stream([good])+'garbage',stream([good])+'{',
                 '[]','null',stream([dict(total_count=2,jobs=[dict(id=1)])]),
                 stream([dict(total_count=2,jobs=[dict(id=1)]),dict(total_count=3,jobs=[dict(id=2)])]),
                 stream([dict(total_count=2,jobs=[dict(id=1),dict(id=1)])]),
                 stream([dict(total_count=1,jobs=[None])]),stream([dict(total_count=True,jobs=[])]),
                 stream([dict(total_count=1,jobs=[dict(id=True)])]),
                 stream([dict(total_count=1,jobs=[dict(id=0)])]),
                 stream([dict(total_count=1,jobs=[dict(id=-1)])]),
                 '{"total_count":0,"total_count":1,"jobs":[]}',
                 '{"total_count":0,"jobs":[],"extra":NaN}']
        for raw in invalid:
            with self.subTest(raw=raw),self.assertRaises((RuntimeError,ValueError,TypeError)):
                m.parse_job_pages(raw)
        self.assertEqual(m.parse_job_pages(stream([dict(total_count=0,jobs=[])])),[])

    def valid_jobs(self):
        jobs=[dict(name=n,conclusion='success',labels=['ubuntu-24.04']) for n in ['resolve','apple-coverage','wheels','gems','npm-pack','crate','registries','windows-sdks','draft']]
        targets={'x86_64-unknown-linux-gnu':'ubuntu-24.04','aarch64-unknown-linux-gnu':'ubuntu-24.04-arm','aarch64-apple-darwin':'macos-15','x86_64-apple-darwin':'macos-15-intel','x86_64-pc-windows-msvc':'windows-2025'}
        for family in ['build','smoke']:
            jobs += [dict(name=f'{family} ({t}, {r})',labels=[r],conclusion='success') for t,r in targets.items()]
        jobs += [dict(name=f'apple-packages ({t}, {r})',labels=[r],conclusion='success') for t,r in targets.items() if 'apple' in t]
        jobs += [dict(name=n,labels=[],conclusion='skipped') for n in ['crates','pypi','npm','rubygems','tap','nuget','maven','pub','publish']]
        return jobs

    def test_required_jobs_and_publish_skip_still_strict(self):
        o=m.Owner(SHA,'hosted',tag=TAG); jobs=self.valid_jobs(); o.validate_jobs(jobs)
        for i in range(len(jobs)):
            with self.subTest(missing=jobs[i]['name']),self.assertRaises(RuntimeError):
                o.validate_jobs(jobs[:i]+jobs[i+1:])
            changed=copy.deepcopy(jobs);changed[i]['conclusion']='failure' if changed[i]['conclusion']=='success' else 'success'
            with self.subTest(changed=changed[i]['name']),self.assertRaises(RuntimeError):o.validate_jobs(changed)
        changed=copy.deepcopy(jobs);changed[0]['labels']=['self-hosted']
        with self.assertRaises(RuntimeError):o.validate_jobs(changed)
        with self.assertRaises(RuntimeError):o.validate_jobs(jobs+[jobs[0]])

    def test_resume_joins_exact_persisted_run_without_dispatch(self):
        class AtMonitor(Exception):pass
        with tempfile.TemporaryDirectory(prefix='0543-recovery-check-') as t:
            f=Path(t);o=m.Owner(SHA,'hosted',True,tag=TAG);o.folder=f;o.check=lambda:None
            m.atomic(f/'accepted.json',dict(id=RUN,head_sha=SHA,head_branch=TAG,event='workflow_dispatch',path='.github/workflows/release.yml'))
            m.atomic(f/'dispatch.json',dict(sha=SHA,tag=TAG,mode='rehearse',previous_run_ids=[1]))
            calls=[]
            def api(path):calls.append(path);raise AtMonitor()
            o.api=api;o.gh=lambda *args:self.fail('resume attempted gh command before exact-run monitor')
            with self.assertRaises(AtMonitor):o.hosted()
            self.assertEqual(calls,[f'actions/runs/{RUN}'])
            d=m.read(f/'dispatch.json');d['mode']='publish';m.atomic(f/'dispatch.json',d)
            calls.clear()
            with self.assertRaises(RuntimeError):o.hosted()
            self.assertEqual(calls,[])

    def test_successful_phase_reused_without_running_or_overwriting(self):
        with tempfile.TemporaryDirectory(prefix='0543-recovery-phase-') as t:
            f=Path(t);o=m.Owner(SHA,'local',True,tag=TAG);o.folder=f;o.check=lambda:None
            command=['sh','sdlc/scripts/test-full-cases','--run']
            receipt=dict(sha=SHA,tag=TAG,command=command,state='completed',exit=0,command_exit=0,guard_error=None,remaining_children=[])
            m.atomic(f/'fullcases.json',receipt);before=(f/'fullcases.json').read_bytes()
            o.phase('fullcases',command)
            self.assertEqual((f/'fullcases.json').read_bytes(),before)
            self.assertEqual(o.result['phases']['fullcases'],receipt)
            for key,bad in [('sha','f'*40),('tag','rc/0.2.0-rc.13'),('command',['false']),('state','running'),('exit',1),('command_exit',1),('guard_error','failed guard'),('remaining_children',[123])]:
                changed=copy.deepcopy(receipt);changed[key]=bad;m.atomic(f/'fullcases.json',changed)
                with self.subTest(field=key),self.assertRaises(RuntimeError):o.phase('fullcases',command)

    def test_resume_rejects_persisted_real_failures_before_monitoring(self):
        with tempfile.TemporaryDirectory(prefix='0543-recovery-refuse-') as t:
            o=m.Owner(SHA,'hosted',True,tag=TAG);o.folder=Path(t);o.check=lambda:None
            accepted=dict(id=RUN,head_sha=SHA,head_branch=TAG,event='workflow_dispatch',path='.github/workflows/release.yml')
            m.atomic(o.folder/'accepted.json',accepted)
            m.atomic(o.folder/'dispatch.json',dict(sha=SHA,tag=TAG,mode='rehearse',previous_run_ids=[]))
            o.api=lambda *args:self.fail('must refuse persisted failure before API')
            for name,value in [('jobs-latest.json',[dict(id=1,run_id=RUN,head_sha=SHA,name='crate',conclusion='failure')]),('completed.json',dict(accepted,status='completed',conclusion='failure'))]:
                with self.subTest(receipt=name):
                    m.atomic(o.folder/name,value)
                    with self.assertRaises(RuntimeError):o.hosted()
                    (o.folder/name).unlink()

    def test_resume_refuses_live_keeper_and_preserves_all_files(self):
        import os
        from unittest.mock import patch
        with tempfile.TemporaryDirectory(prefix='0543-recovery-live-') as t:
            f=Path(t);o=m.Owner(SHA,'hosted',True,tag=TAG);o.folder=f
            m.atomic(f/'Result.json',dict(sha=SHA,tag=TAG,kind='hosted',state='recovering',recovery=dict(keeper_pid=os.getpid(),existing_run_id=RUN)))
            m.atomic(f/'accepted.json',dict(id=RUN))
            (f/'PID').write_text(str(os.getpid()))
            (f/'OWNER').write_text('preserve me')
            before={p.name:p.read_bytes() for p in f.iterdir()}
            o.cli_compatible=lambda:self.fail('must refuse a live owner before preflight')
            cwd=os.getcwd()
            try:
                with patch.object(m,'ROOT',f),self.assertRaises(RuntimeError):o.run()
            finally:
                os.chdir(cwd)
            self.assertEqual(before,{p.name:p.read_bytes() for p in f.iterdir() if p.name!='owner.lock'})
            o.owner_lock.close()

    def test_resume_keeps_history_and_never_dispatches(self):
        import os
        from unittest.mock import patch
        with tempfile.TemporaryDirectory(prefix='0543-recovery-history-') as t:
            f=Path(t);o=m.Owner(SHA,'hosted',True,tag=TAG);o.folder=f
            m.atomic(f/'Result.json',dict(sha=SHA,tag=TAG,kind='hosted',state='failed',phases={}))
            m.atomic(f/'accepted.json',dict(id=RUN,head_sha=SHA,head_branch=TAG,event='workflow_dispatch',path='.github/workflows/release.yml'))
            m.atomic(f/'dispatch.json',dict(sha=SHA,tag=TAG,mode='rehearse',previous_run_ids=[]))
            (f/'PID').write_text('99999999')
            (f/'OWNER').write_text('prior owner')
            (f/'finished.log').write_text('prior log')
            m.atomic(f/'jobs-latest.json',[])
            before={p.name:p.read_bytes() for p in f.iterdir()}
            o.cli_compatible=lambda:None;o.check=lambda:None
            o.api=lambda *args:(_ for _ in ()).throw(RuntimeError('stop synthetic monitor'))
            o.gh=lambda *args:self.fail('resume dispatched')
            cwd=os.getcwd()
            try:
                with patch.object(m,'ROOT',f),patch.object(m.os,'kill',side_effect=ProcessLookupError):
                    self.assertEqual(o.run(),1)
            finally:
                os.chdir(cwd);o.owner_lock.close()
            for name,data in before.items():
                copies=list(f.glob(name+'.before-resume-*'))
                self.assertEqual(len(copies),1,name)
                self.assertEqual(copies[0].read_bytes(),data)
            self.assertEqual((f/'finished.log').read_text(),'prior log')

    def test_explicit_tag_flows_through_guards_dispatch_paths_and_recovery(self):
        from unittest.mock import patch
        class StopAfterDispatch(Exception): pass
        for tag in ('rc/0.2.0-rc.13', 'rc/0.2.0-rc.14'):
            with self.subTest(tag=tag), tempfile.TemporaryDirectory(prefix='0543-tag-flow-') as t:
                o = m.Owner(SHA, 'hosted', tag=tag)
                self.assertEqual(o.result['tag'], tag)
                self.assertIn(tag.replace('/', '-'), o.folder.name)
                self.assertNotEqual(o.folder, m.Owner(SHA, 'hosted', tag='rc/0.2.0-rc.15').folder)
                calls = []
                def git(args, **kwargs):
                    calls.append(args)
                    if args[1:3] == ['rev-parse', 'HEAD']: return SHA
                    if args[1:3] == ['branch', '--show-current']: return m.BRANCH
                    if args[1] in ('status', 'ls-files'): return ''
                    if args[1] == 'rev-parse':
                        self.assertEqual(args[-1], tag + '^{commit}')
                        return SHA
                    if args[1] == 'ls-remote':
                        self.assertEqual(args[-2:], ['refs/tags/' + tag, 'refs/tags/' + tag + '^{}'])
                        return SHA + '\trefs/heads/' + m.BRANCH + '\n' + SHA + '\trefs/tags/' + tag
                    raise AssertionError(args)
                with patch.object(m, 'output', side_effect=git): m.guard(SHA, tag)
                for wrong in ('local-sha', 'remote-sha', 'remote-ref'):
                    def broken(args, **kwargs):
                        answer = git(args, **kwargs)
                        if wrong == 'local-sha' and args[-1] == tag + '^{commit}': return 'b' * 40
                        if wrong == 'remote-sha' and args[1] == 'ls-remote': return answer.replace(SHA, 'b' * 40)
                        if wrong == 'remote-ref' and args[1] == 'ls-remote': return answer.replace(tag, 'rc/0.2.0-rc.99')
                        return answer
                    with patch.object(m, 'output', side_effect=broken), self.assertRaises(RuntimeError): m.guard(SHA, tag)
                o.folder = Path(t); o.check = lambda: None
                count = []
                def api(path):
                    count.append(path)
                    if len(count) == 1: return {'workflow_runs': []}
                    raise StopAfterDispatch()
                dispatches = []
                o.api = api; o.gh = lambda *args: dispatches.append(args)
                with self.assertRaises(StopAfterDispatch): o.hosted()
                self.assertEqual(dispatches, [('workflow', 'run', 'release.yml', '--repo', m.REPO, '--ref', tag, '-f', 'mode=rehearse')])
                self.assertEqual(m.read(o.folder / 'dispatch.json')['tag'], tag)
                accepted = dict(id=RUN, head_sha=SHA, head_branch=tag, event='workflow_dispatch', path='.github/workflows/release.yml')
                m.atomic(o.folder / 'accepted.json', accepted)
                self.assertEqual(o.resumed_run(), accepted)
                for key, bad in [('head_branch', 'rc/0.2.0-rc.99'), ('head_sha', 'b' * 40), ('path', '.github/workflows/other.yml')]:
                    m.atomic(o.folder / 'accepted.json', dict(accepted, **{key: bad}))
                    with self.assertRaises(RuntimeError): o.resumed_run()
        for tag in ('main', '--help', 'rc/0.2.0-rc.014', 'rc/0.2.1-rc.14', 'rc/0.2.0-rc.12', 'rc/0.2.0-rc.14;echo bad'):
            with self.assertRaises(m.argparse.ArgumentTypeError): m.candidate_tag(tag)

    def test_installed_cli_supports_every_runtime_flag(self):
        m.Owner(SHA,'hosted',tag=TAG).cli_compatible()

if __name__=='__main__':unittest.main(verbosity=2)
