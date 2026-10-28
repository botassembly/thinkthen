"""Publish checked draft assets without building packages or changing main."""
import argparse
import base64
import json
from pathlib import Path
import subprocess
import tarfile
import tempfile

from release_metadata import source_tags, version
from release_upload import verify_download


def run(*args, **options):
    return subprocess.run(list(map(str, args)), check=True, capture_output=True, text=True, **options).stdout.strip()


def api(endpoint, method=None, payload=None):
    args = ['gh', 'api', endpoint]
    if method:
        args += ['--method', method, '--input', '-']
    answer = run(*args, input=json.dumps(payload) if method else None)
    return json.loads(answer) if answer else None


def release_info(repository, tag):
    """Read draft or published metadata through the same entry point as uploads."""
    data = json.loads(run('gh', 'release', 'view', tag, '--repo', repository,
                          '--json', 'databaseId,tagName,isDraft,targetCommitish'))
    return {'id': data['databaseId'], 'tag_name': data['tagName'],
            'draft': data['isDraft'], 'target_commitish': data['targetCommitish']}


def publish(tag, digest, remote, repository):
    if tag != 'v' + version():
        raise ValueError('selected tag differs from the workspace version')
    with tempfile.TemporaryDirectory(prefix='thinkthen-publish-') as temporary:
        scratch = Path(temporary)
        assets = scratch / 'assets'
        assets.mkdir()
        review = 'review-' + tag
        try:
            release = release_info(repository, review)
        except subprocess.CalledProcessError:
            release = release_info(repository, tag)
        if release['tag_name'] not in (review, tag) or (release['tag_name'] == review and not release['draft']):
            raise ValueError('publication requires the reviewed draft or its final release')
        run('gh', 'release', 'download', release['tag_name'], '--repo', repository, '--dir', assets)
        verify_download(assets, digest)
        staged = scratch / 'registries'
        with tarfile.open(assets / f'thinkthen-{version()}-registries.tar.gz') as archive:
            archive.extractall(staged, filter='data')
        provenance = json.loads((staged / 'source.json').read_text())
        revision = provenance['revision']
        if provenance != {'revision': revision, 'version': version()}:
            raise ValueError('draft and package source revisions differ')
        additions = scratch / 'additions'
        with tarfile.open(staged / 'tag-source.tar.gz') as archive:
            if any(part.casefold() == '.git' for member in archive.getmembers() for part in Path(member.name).parts):
                raise ValueError('tag source cannot contain Git metadata')
            archive.extractall(additions, filter='data')
        if json.loads((additions / 'release-source.json').read_text()) != provenance:
            raise ValueError('tag source differs from the approved packages')
        entries = json.loads((staged / 'r/packages.json').read_text())
        entry, = entries
        if entry != {'package': 'thinkthen', 'url': f'https://github.com/{repository}',
                     'subdir': 'sdk/r', 'branch': '*release'}:
            raise ValueError('R-universe entry differs from the approved source')
        registry_endpoint = 'repos/' + repository.split('/')[0] + '/r-universe/contents/packages.json'
        registry = api(registry_endpoint)
        packages = json.loads(base64.b64decode(registry['content']))
        packages = [item for item in packages if item['package'] != entry['package']] + entries
        checkout = scratch / 'source'
        run('git', 'clone', '--quiet', '--no-checkout', remote, checkout)

        def git(*args):
            return run('git', '-C', checkout, *args)

        if git('rev-parse', revision + '^{commit}') != revision:
            raise ValueError('package revision must be a full source commit')
        git('merge-base', '--is-ancestor', revision, 'origin/main')
        git('checkout', '--quiet', '--detach', revision)
        # Copy only approved additions. Tar extraction never touches the Git directory.
        import shutil
        for name in ('sdk/go/thinkthen', 'sdk/swift'):
            if (additions / name).is_dir() and (checkout / name).is_dir():
                shutil.rmtree(checkout / name)
        shutil.copytree(additions, checkout, dirs_exist_ok=True)
        git('add', '--force', '.')
        expected_tree = git('write-tree')
        tags = source_tags(tag)
        present = set(git('tag', '--list', *tags).splitlines())
        if present:
            if present != set(tags):
                raise ValueError('existing release tag pair is incomplete')
            commit = git('rev-parse', tags[0])
            if (git('rev-parse', tags[1]) != commit
                    or git('rev-list', '--parents', '-n', '1', commit).split() != [commit, revision]
                    or git('rev-parse', commit + '^{tree}') != expected_tree):
                raise ValueError('existing release tags differ from the approved parent or tree')
        else:
            if release['tag_name'] != review:
                raise ValueError('final release must already have the approved tags')
            git('-c', 'user.name=ThinkThen release', '-c', 'user.email=imaurer@users.noreply.github.com',
                'commit', '--quiet', '-m', 'Stage approved native release ' + tag)
            commit = git('rev-parse', 'HEAD')
        # GitHub ignores target_commitish when the final tag already exists.
        # The verified Git tag pair owns the final commit identity.
        if release['tag_name'] == review and release['target_commitish'] != revision:
            raise ValueError('reviewed draft target differs from the approved source')
        if not present:
            for name in tags:
                git('tag', name, commit)
            git('push', '--atomic', 'origin', *('refs/tags/' + name for name in tags))
        if release['draft']:
            api(f'repos/{repository}/releases/{release["id"]}', 'PATCH',
                {'tag_name': tag, 'target_commitish': commit, 'draft': False})
        if packages != json.loads(base64.b64decode(registry['content'])):
            api(registry_endpoint, 'PUT', {'message': 'Publish ThinkThen ' + tag,
                'sha': registry['sha'], 'content': base64.b64encode((json.dumps(packages, indent=2) + '\n').encode()).decode()})
        print(f'Published {tag} and {tags[1]} at {commit}; updated R-universe')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('tag')
    parser.add_argument('checksum', help='Approved SHA-256 of the draft SHA256SUMS file')
    parser.add_argument('--ian-approved', action='store_true', help='Confirm Ian authorized this publication')
    parser.add_argument('--repository', default='botassembly/thinkthen', help='GitHub release repository')
    parser.add_argument('--remote', help='Git remote for the release repository')
    args = parser.parse_args()
    if not args.ian_approved:
        raise ValueError('Ian must authorize publication; invoke with --ian-approved only after his go')
    publish(args.tag, args.checksum, args.remote or f'https://github.com/{args.repository}.git', args.repository)
