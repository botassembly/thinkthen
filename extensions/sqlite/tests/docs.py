"""Run every literal SQL block in the SQLite README and guide in the shell."""
from pathlib import Path
import re
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'databases'))
from testing import child_environment

extension = Path(sys.argv[1]).resolve()
for document in ('extensions/sqlite/README.md', 'docs/surfaces/sqlite.md'):
    blocks = re.findall(r'^```sql\n(.*?)^```', (ROOT / document).read_text(), re.M | re.S)
    assert blocks, document
    with tempfile.TemporaryDirectory() as scratch:
        home = Path(scratch)
        (home / extension.name).symlink_to(extension)
        # The guide's final settings-only example selects OpenAI but sends no request.
        env = {**child_environment(), 'HOME': scratch, 'THINKTHEN_BACKEND': 'stub',
               'OPENAI_API_KEY': 'documentation-test-key'}
        result = subprocess.run(['sqlite3', '-bail', ':memory:'], input='\n'.join(blocks),
                                text=True, capture_output=True, cwd=home, env=env)
        assert result.returncode == 0, (document, result.stdout, result.stderr)
        print(f'{document}: {len(blocks)} literal SQL blocks passed')
