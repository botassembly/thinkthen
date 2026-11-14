"""Run the literal Ruby ideal guide against installed-gem scenario fixtures."""
from pathlib import Path
import argparse
import subprocess

from guide_examples import IDEAL, blocks, check, scenario

ROOT = Path(__file__).resolve().parent.parent
MESSAGES = ['Please refund my order.', 'Where is my parcel?', 'I was charged twice for one invoice.']
FAILED = 'failed: backend.status: the backend answered status 503: {"error":"unavailable"}'
PREFIX = '''require "thinkthen"
ThinkThen.configure { |settings| settings.retry = ThinkThen::RetryPolicy.new(max_retries: 0) }
at_exit { ThinkThen.close }
'''


def run_ruby(command=None, negative=None):
    command = command or 'ruby'
    guide = blocks(IDEAL / 'ruby.md')
    assert len(guide) == 27 and all(language == 'ruby' for language, _, _ in guide), 'unaccounted Ruby guide fence'
    source = [body for _, body, _ in guide]
    for path in (ROOT / 'sdk/ruby/README.md', ROOT / 'site/src/content/docs/sdks/ruby.mdx'):
        assert [body for language, body, _ in blocks(path) if language == 'ruby'] == source, f'{path}: guide fences differ'
    band_setup = source[1].split('if is_refund.answered?', 1)[0]
    topic_setup = source[5].split('def team_for', 1)[0]
    scale_setup = source[14].split('urgency =', 1)[0]
    message_setup = source[11].split('ThinkThen.filter', 1)[0]
    form_setup = source[19].split('ThinkThen.annotate_each', 1)[0]
    failures = []
    if negative == 'refusal':
        exercise(command, 'yes', 'negative-refusal', 'ThinkThen.choose("Which?", "record", choices: %i[billing shipping]).answer\n', '', failures, refused='NoMethodError')
        assert not failures, failures
        return
    for name in ('yes', 'no', 'unsure', 'failed'):
        queue = 'inbox' if name == 'no' else 'refund queue'
        guarded = 'a person reads it' if name == 'unsure' else queue
        explicit = 'try again later backend.status' if name == 'failed' else 'a person reads it 0.5' if name == 'unsure' else queue
        scalar_word = 'false' if name == 'no' else 'true'
        probability = {'yes':'0.95','no':'0.05','unsure':'0.5','failed':''}[name]
        per_record = ''.join((('try again later' if name == 'failed' else guarded)+': '+text+'\n') for text in MESSAGES)
        cases = [
            (0, source[0], queue+'\n', name == 'failed'),
            (1, source[1], guarded+'\n', name == 'failed'),
            (2, band_setup+source[2], guarded+'\n', name == 'failed'),
            (3, source[3], ('could not judge it: '+FAILED.split(': ',2)[2] if name == 'failed' else scalar_word)+'\n', False),
            (4, source[4], probability+'\n', name == 'failed'),
            (5, source[5], ('a person reads it' if name == 'unsure' else 'warehouse' if name == 'no' else 'billing team')+'\n', name == 'failed'),
            (6, source[6], per_record, False),
            (7, message_setup+source[7], (('unsure' if name == 'unsure' else scalar_word)+'\n')*3, name == 'failed'),
            (8, message_setup+source[8], (explicit+'\n')*3, False),
            # The method fence is executed through the literal next fence on every state.
            (10, message_setup+source[9]+source[10], per_record, False),
            (11, source[11], ''.join('refund queue: '+text+'\n' for text in MESSAGES) if name != 'no' else '', name == 'failed'),
            (12, message_setup+source[12], ''.join(('try again later: '+text+' backend.status\n') if name == 'failed' else ('refund queue: '+text+'\n') for text in MESSAGES) if name != 'no' else '', False),
            (13, source[13], '' if name == 'no' else 'billing team\nbilling\nshipping\n', name == 'failed'),
            (14, source[14], '0.5\n' if name == 'unsure' else 'answer today\n1.0\n', name == 'failed'),
            (15, scale_setup+source[15], ''.join(f'{index} {text}\n' for index,text in enumerate(MESSAGES,1)), name == 'failed'),
            (16, source[16], ('a person reads it' if name == 'unsure' else 'no line answers' if name == 'no' else 'Please refund my order.')+'\n', name == 'failed'),
            (20, topic_setup+scale_setup+form_setup+source[20], '', name == 'failed'),
            (21, topic_setup+source[21]+'\nputs is_refund\nputs topic\n', scalar_word+'\n'+('unsure' if name == 'unsure' else 'shipping' if name == 'no' else 'billing')+'\n', name == 'failed'),
            (22, source[22]+'\nraise "lost id" unless is_refund.id == "a1"\nputs is_refund\n', ('unsure' if name == 'unsure' else scalar_word)+'\n', name == 'failed'),
            (23, toolbox_setup()+source[23]+'\nputs topic\n', ('unsure' if name == 'unsure' else 'shipping' if name == 'no' else 'billing')+'\n', name == 'failed'),
            (26, source[26]+'\nputs is_refund\n', scalar_word+'\n', name == 'failed'),
        ]
        for index, body, output, refused in cases:
            control = negative if index == 0 and name == 'yes' and negative in ('output','branch','exit') else None
            exercise(command, name, index, body, '' if refused else output, failures,
                     refused=refused, negative=control)
    for name in ('relations','unsure','failed'):
        exercise(command, name, 17, source[17], 'person Morgan\norganization Example Group\n' if name == 'relations' else '', failures, refused=name == 'failed')
        exercise(command, name, 18, source[18], 'Morgan works_for Example Group\n' if name == 'relations' else '', failures, refused=name == 'failed')
    for name in ('yes','no','unsure','failed','refund-failed','topic-failed','urgency-failed','refund-unsure','topic-unsure'):
        branch = 'try again later: ' if name in ('failed','refund-failed','topic-failed','urgency-failed') else 'a person reads it: ' if name in ('unsure','refund-unsure','topic-unsure') else None
        output = ''.join(branch+text+'\n' for text in MESSAGES) if branch else '' if name == 'no' else 'refund queue: billing 1.0\n'*3
        exercise(command, name, 19, topic_setup+scale_setup+source[19], output, failures)
        if name not in ('yes','no','unsure','failed'):
            exercise(command, name, 20, topic_setup+scale_setup+form_setup+source[20], '', failures, refused=name.endswith('-failed'))
    exercise(command, 'low-score', 14, source[14], '0.0\n', failures)
    other = {'replies':[{'distribution':{'billing':0,'shipping':0,'other':1}}]}
    exercise(command, 'other', 5, source[5], 'front desk\n', failures, case=other)
    exercise(command, 'yes', 24, 'is_refund = nil\n'+source[24]+'\nputs is_refund\n', 'true\n', failures)
    exercise(command, 'yes', 25, source[25], '1 0 0\n', failures)
    truth = 'result = ThinkThen.decide("Ready?", "record")\nputs "object:true answer:#{result.answer}" if result\n'
    exercise(command, 'no', 'truthiness', truth, 'object:true answer:false\n', failures)
    refusal = 'ThinkThen.choose("Which?", "record", choices: %i[billing shipping]).yes?\n'
    exercise(command, 'yes', 'non-decide-yes', refusal, '', failures, refused='NoMethodError')
    refusals = [
        ('bare-equality', 'topic = ThinkThen.choose("Which?", "record", choices: %i[billing shipping])\nputs(topic == :billing)\n', 'TypeError'),
        ('collection', 'ThinkThen.decide("Ready?", ["record"])\n', 'ThinkThen::UsageError'),
        ('filter-band', 'ThinkThen.filter("Ready?", ["record"], threshold: 0.2..0.8).to_a\n', 'ThinkThen::UsageError'),
        ('saved-filter-band', 'ThinkThen::FilterQuestion.new("Ready?", threshold: 0.2..0.8)\n', 'ThinkThen::UsageError'),
        ('missing-topic', source[5].split('topic = ThinkThen.choose',1)[0]+'team_for(:new)\n', 'NoMatchingPatternError'),
        ('missing-entry', topic_setup+scale_setup+form_setup+'form = ThinkThen.annotate(questions, "record")\nform.answer[:absent]\n', 'KeyError'),
        ('missing-toolbox', toolbox_setup()+'ThinkThen::Toolbox.load_file("toolbox.json").choose(:absent)\n', 'ThinkThen::UsageError'),
        ('wrong-toolbox', toolbox_setup()+'ThinkThen::Toolbox.load_file("toolbox.json").decide(:topic)\n', 'ThinkThen::UsageError'),
        ('cancelled', source[26].replace('is_refund =', 'stop.cancel\nis_refund ='), 'ThinkThen::CancelledError'),
    ]
    for label, body, refusal in refusals:
        exercise(command, 'yes', label, body, '', failures, refused=refusal)
    assert not failures, failures
    print('Ruby ideal guide: all 27 literal blocks, reachable branches and loud reads passed')


def toolbox_setup():
    return '''File.write("toolbox.json", ThinkThen::Toolbox.new(questions: {
refund: ThinkThen::DecideQuestion.new("Does the customer request a refund?", field: "/text"),
topic: ThinkThen::ChooseQuestion.new("What is the message about?", choices: %i[billing shipping other], field: "/text")
}).to_json)
'''


def exercise(command, name, index, body, output, failures, *, refused=False, negative=None, case=None):
    try:
        with scenario(name, case=case) as (folder, env):
            path = folder / 'example.rb'
            path.write_text(PREFIX+body)
            result = subprocess.run([command, '-w', str(path)], cwd=folder, env=env,
                                    capture_output=True, text=True, timeout=60)
            check(f'ruby/ideal/{index}/{name}', result, output=output,
                  branch=output.splitlines()[0] if output else None,
                  exit_code=1 if refused else 0,
                  refusal=str(refused) if isinstance(refused,str) else 'ThinkThen::FailedError' if refused else None,
                  negative=negative)
    except AssertionError as error:
        if negative:
            raise
        failures.append(str(error))
        print('FAIL '+str(error), flush=True)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--controls', action='store_true')
    args = parser.parse_args()
    if args.controls:
        for control, diagnostic in (('output', 'output:'), ('branch', 'branch:'), ('exit', 'exit:'), ('refusal', 'promised refusal')):
            try:
                run_ruby(negative=control)
            except AssertionError as error:
                assert diagnostic in str(error), str(error)
                print('Ruby negative control rejected: '+control, flush=True)
            else:
                raise AssertionError('Ruby negative control was accepted: '+control)
    else:
        run_ruby()
