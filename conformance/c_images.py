"""Project canonical original-image fixtures into counted typed C inputs."""
import base64
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
QUESTIONS = {'decide': {'decide': 'Is red visible?'}, 'choose': {'choose': 'Which color?', 'options': ['red', 'blue']}, 'score': {'score': 'How red?', 'levels': ['none', 'all']}}


def project(row):
    corpus = json.loads((ROOT / 'conformance/cases.json').read_text())['parity']
    verb = row['verb']
    steps = []
    for scenario in corpus['image_admission_scenarios'][row['input']['scenarios_ref']]:
        construction = scenario['construction']
        profile = corpus['image_profiles'][scenario['profile_ref']]
        paths = construction.get('images', [construction.get('existing_image')] * construction.get('repeat', 1))
        recipe = construction.get('text_recipe')
        caption = None
        if recipe:
            counts = recipe['by_function'][verb]
            caption = recipe['prefix'] + recipe['repeat_token'] * counts['repeat_count'] + 'x' * counts['ascii_tail_bytes']
        many = construction.get('mode') == 'many inputs'
        captions = [caption]
        if many:
            captions += [('I' + caption[1:]) if construction.get('distinct_image_states') else caption]
        expected = scenario['expect']
        vendor = 'perplexity' if scenario['profile_ref'] == 'perplexity-decider' else 'liquid'
        settings = {'backend': 'llamacpp' if 'profile_file' in profile else vendor, 'model': profile['model'], 'batch': 'max' if many else 1}
        if 'profile_file' in profile:
            settings['profile'] = str(ROOT / profile['profile_file'])
        step = {'verb': verb, 'question': QUESTIONS[verb], 'items': captions, 'text': True,
                'caption_files': caption is not None, 'image_only': caption is None,
                'image_paths': paths, 'media': construction.get('declared_media', construction.get('media', 'image/png')),
                'expect': {'requests_sent': expected['requests'], **({'error': expected['kind']} if 'kind' in expected else {})},
                'settings': settings, 'arm': 'arm/images/%s/%s/v1' % (vendor, verb),
                'image_scenario': scenario, 'image_profile': profile}
        if step['media'] in ('image/gif', 'image/webp'):
            step.update(paths=paths, source_unit=4)
        steps.append(step)
        if many and verb == 'choose' and construction.get('same_state_choose_candidate_orders'):
            steps.append({**step, 'items': [caption, caption], 'candidate_orders': construction['same_state_choose_candidate_orders']})
    return {'steps': steps, 'arm': steps[0]['arm'], 'image_variants': True}


def assert_images(step, got, bodies):
    scenario = step['image_scenario']
    expected_count = step['expect']['requests_sent']
    assert len(bodies) == expected_count, (scenario['id'], len(bodies), expected_count)
    if got['code']:
        return
    images = [(ROOT / path).read_bytes() for path in step['image_paths']]
    assert len(got['rows']) == len(step['items']), (scenario['id'], got)
    for row in got['rows']:
        assert [bytes.fromhex(value) for value in row['images']] == images, scenario['id']
        construction=scenario['construction']
        if construction.get('dimensions'):
            media=1 if construction.get('media')=='image/jpeg' else 2
            assert row['image_properties'] == [[media,*construction['dimensions']]] * len(images), scenario['id']
        original = row['input']
        if isinstance(original, dict):
            width, height = construction.get('dimensions', [1, 1])
            assert original == {'text': step['items'][row['index']], 'images': [
                {'media': step['media'], 'base64': base64.b64encode(raw).decode(),
                 'width': width, 'height': height} for raw in images]}, scenario['id']
            original = original['text']
        assert original == step['items'][row['index']], scenario['id']
        assert row['value'] == {'decide': True, 'choose': 'red', 'score': .8}[step['verb']], (scenario['id'],row['value'])
        if step['verb'] == 'decide':
            assert abs(row['probability'] - .9) < 1e-10, (scenario['id'],row['probability'])
        if step['verb'] != 'decide':
            assert row['probabilities'] == ({'red': .8, 'blue': .2} if step['verb'] == 'choose' else {'none': .2, 'all': .8}), (scenario['id'],row['probabilities'])
    urls = ['data:%s;base64,%s' % ('image/jpeg' if path.endswith('.jpg') else 'image/png', base64.b64encode(raw).decode()) for path, raw in zip(step['image_paths'], images)]
    wants = []
    for at in range(expected_count):
        caption = step['items'][at] or ''
        role = step['verb']
        q = {'type': 'noul', 'instructions': 'Is red visible?'} if role == 'decide' else {'type': 'choice' if role == 'choose' else 'score', 'instructions': 'Which color?' if role == 'choose' else 'How red?', 'criteria': {'red': None, 'blue': None} if role == 'choose' else ['none', 'all']}
        if step.get('candidate_orders'):
            q['criteria'] = dict.fromkeys(step['candidate_orders'][at])
        profile = step['image_profile']
        want = {'state': caption, 'model': profile['model'], 'questions': {'q1': q}, 'images': urls}
        if profile['model'] == 'pplx-decider-v1-27b':
            want = {'state': ([caption] if caption else []) + [{'type': 'image_url', 'image_url': {'url': url}} for url in urls], 'model': profile['model'], 'questions': {'q1': q}}
        wants.append(want)
    # Concurrent requests can arrive in either order. Consume whole matches so
    # duplicates cannot hide a missing request; candidate and image order stay exact.
    for body in bodies:
        actual=json.loads(body)
        matches = [at for at, want in enumerate(wants) if actual == want]
        assert matches, (scenario['id'], 'wire fields differ')
        if role == 'choose':
            matches = [at for at in matches if list(actual['questions']['q1']['criteria']) == list(wants[at]['questions']['q1']['criteria'])]
            assert matches, (scenario['id'],'candidate order changed')
        wants.pop(matches[0])
        construction = scenario['construction']
        target = construction.get('final_serialized_body_bytes')
        if construction.get('mode') == 'many inputs':
            target = construction['text_recipe']['by_function'][role].get('each_final_body_bytes')
        if target is not None:
            assert len(body.encode()) == target, (scenario['id'], len(body.encode()), target)
        if 'compressed_bytes_min' in construction:
            assert len(images[0]) >= construction['compressed_bytes_min'], scenario['id']
        if 'final_body_bytes_min' in construction:
            assert len(body.encode()) >= construction['final_body_bytes_min'], scenario['id']
        limit = profile.get('final_body_bytes')
        if limit:
            assert len(body.encode()) <= limit['upper'] if limit['inclusive'] else len(body.encode()) < limit['upper']
