"""Reuse the canonical CLI row projection for copied native session packets."""
from cli_parity import project_row
from c_parity import ERRORS


def project_session(payload, fixture):
    if 'admission' in payload:
        return payload['admission']
    # Native packets already retain the original input; only CLI callers wrap it.
    projection_fixture = dict(fixture, context_present=False, contexts=None, candidate_orders=None)
    packets = payload['packets']
    terminal = next(packet for packet in packets if packet['kind'] == 'terminal')
    facts = terminal.get('facts', {})
    rows = []
    aggregates = [packet for packet in packets if packet['kind'] == 'aggregate']
    for packet in aggregates or [packet for packet in packets if packet['kind'] == 'row']:
        values = packet['value'] if isinstance(packet['value'], list) else [packet['value']]
        for result in values:
            result = dict(result)
            result.update(result.get('source', {}))
            if fixture['verb'] == 'relate' and 'input_sources' in result:
                sources = {source['index']: source['source'] for source in result['input_sources']}
                result['input'] = [{'record': original, **sources.get(index, {})}
                                   for index, original in enumerate(result['input'])]
            rows.append(project_row(result, projection_fixture, len(rows)))
    failure = terminal.get('failure')
    if failure:
        error = failure['error']
        output = {'code': ERRORS[error['kind']], 'message': error['message']}
        if 'at' in error.get('stopped', {}):
            output['stopped_at'] = error['stopped']['at']
        if fixture.get('incremental'):
            output['completed'] = rows
    else:
        output = {'code': 0, 'schema': 'thinkthen.result/2', 'rows': rows,
                  'observations': sum(packet['kind'] == 'observation' for packet in packets)}
    output.update({key: facts[key] for key in ('requests_sent', 'records', 'cache_answers',
                  'call_id', 'input_tokens', 'output_tokens') if key in facts})
    return output
