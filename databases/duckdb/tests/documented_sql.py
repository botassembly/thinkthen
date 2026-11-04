"""Run every authored SQL block with its documented table and file fixtures."""
import re
import shutil
from pathlib import Path


def documents(root, extension, directory):
    # Run the README's relative LOAD verbatim from its owned fixture folder.
    shutil.copy2(extension, Path(directory) / 'thinkthen.duckdb_extension')
    result = []
    for source in ('databases/duckdb/README.md', 'docs/surfaces/duckdb.md'):
        text = (root / source).read_text()
        blocks = re.findall(r'```sql\n(.*?)\n```', text, re.S)
        assert blocks, source
        statements = []
        for index, block in enumerate(blocks, 1):
            # Examples reuse their materialized table names. Clear only that
            # fixture before the next literal CREATE, without editing the SQL.
            for table in re.findall(r'CREATE TABLE (\w+) AS', block, re.I):
                statements.append('DROP TABLE IF EXISTS ' + table + ';')
            block = block.replace('/absolute/path/to/thinkthen.duckdb_extension', str(extension))
            block = re.sub(r"(LOAD\s+'[^']+';)", r"\1\nSET thinkthen_backend = 'stub';", block, flags=re.I)
            statements.append('-- ' + source + ' block ' + str(index) + '\n' + block)
        result.append({'source': source, 'blocks': len(blocks), 'sql': '\n'.join(statements)})
    guide = (root / 'docs/surfaces/duckdb.md').read_text()
    records = re.search(r'```json\n(.*?)\n```', guide, re.S)
    assert records is not None
    (Path(directory) / 'records.jsonl').write_text(records[1] + '\n')
    return result
