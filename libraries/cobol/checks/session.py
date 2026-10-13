"""Exercise ordinary generated records against a locally installed native fixture."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

REPO = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(REPO / 'conformance/children'))
from children import child_env
from backend import Backend

PACKAGE = Path(os.environ.get('THINKTHEN_SESSION_PACKAGE', REPO / 'libraries/cobol')).resolve(strict=True)
HEADER = Path(os.environ.get('THINKTHEN_C_HEADER', REPO / 'libraries/c/include/thinkthen.h'))
(REPO / 'libraries/cobol/checks/target').mkdir(exist_ok=True)
LIBRARY = Path(os.environ.get('THINKTHEN_C_LIBRARY', PACKAGE / 'checks/target/libthinkthen.so.0'))
questions = {
    'decide': {'decide': 'Refund?'},
    'choose': {'choose': 'Which?', 'options': ['a', 'b']},
    'tag': {'tag': 'Which?', 'labels': ['a', 'b']},
    'score': {'score': 'How much?', 'levels': ['low', 'high']},
    'filter': {'decide': 'Refund?'},
    'rank': {'decide': 'Relevant?'},
    'find': {'find': 'Which?'},
    'annotate': {'version': 1, 'questions': {'refund': {'decide': 'Refund?'}}},
    'recognize': {'version': 1, 'recognize': {'kinds': {'person': 'A name.'}}},
    'relate': {'version': 1, 'relate': {'relations': [{'name': 'linked', 'source': 'person', 'target': 'person'}]}},
}
with tempfile.TemporaryDirectory(prefix='cobol-session-', dir=REPO / 'libraries/cobol/checks/target') as temporary:
    root = Path(temporary)
    installed = root / 'installed'
    for directory in ('src', 'copybooks', 'examples'):
        shutil.copytree(PACKAGE / directory, installed / directory)
    shutil.copy(HEADER, installed / 'src/thinkthen.h')
    shutil.copy(LIBRARY, root / 'libthinkthen.so.0')
    entity_helper = root / 'entity.c'
    entity_helper.write_text('''#include "tt_requests_generated.h"
tt_value *TT_TEST_ENTITY(const char *name, unsigned long long length) {
    tt_value *id=NULL,*named=NULL,*kind=NULL,*object=NULL;
    const thinkthen_cobol_text names[3]={{"id",2},{"name",4},{"kind",4}};
    if(TT_VALUE_STRING("p1",2,&id) || TT_VALUE_STRING(name,length,&named) || TT_VALUE_STRING("person",6,&kind)) goto done;
    const tt_value *values[3]={id,named,kind};
    TT_VALUE_OBJECT(names,values,3,&object);
done:
    TT_VALUE_FREE(id); TT_VALUE_FREE(named); TT_VALUE_FREE(kind); return object;
}
''')
    entity_header = root / 'entity.h'
    entity_header.write_text('#include "tt_requests_generated.h"\ntt_value *TT_TEST_ENTITY(const char *, unsigned long long);\nint TT_TEST_VALUE_OWNERS(void);\n')
    home = root / 'home'
    home.mkdir()
    backend = Backend(None, key='tt-canary-273', arrivals='wire', retry_ms=True, concurrent=True)
    try:
        env = child_env(home=home, THINKTHEN_BASE_URL=f'http://127.0.0.1:{backend.server_port}/generic/v1',
                        THINKTHEN_API_KEY='tt-canary-273', THINKTHEN_CACHE=str(root / 'cache'))
        def compile_caller(source, executable):
            subprocess.run(['cobc', '-x', '-free', '-fstatic-call', '-fno-gen-c-decl-static-call',
                            '-I', str(installed / 'copybooks'), '-A', '-include ' + str(installed / 'src/tt_session.h') + ' -Wno-incompatible-pointer-types -include ' + str(entity_header) + ' -I' + str(installed / 'src'),
                            '-o', str(executable), str(source), str(installed / 'src/tt_session.c'),
                            str(installed / 'src/tt_requests_generated.c'), str(entity_helper), str(REPO / 'libraries/cobol/checks/value_owner.c'),
                            '-A', '-I' + str(installed / 'src'),
                            '-Q', str(root / 'libthinkthen.so.0') + ' -Wl,-rpath,$ORIGIN'],
                           env=env, check=True, timeout=30)
        template = (installed / 'examples/session.cob').read_text()
        # The plan caller exercises generated counted mapping without a session.
        plan = template[:template.index('    if status-code not = 0\n')]
        plan = plan.replace('01 engine usage pointer.', '01 engine usage pointer.\n01 plan-owner usage pointer.\n01 records-owner usage pointer.\n01 plan-sentinel usage pointer.\n01 null-output usage pointer value null.\n01 plan-text pic x(8192).\n01 plan-written usage binary-double unsigned.')
        plan = plan.replace('call "TT_SESSION_DECIDE" using by value engine\n       by reference tt-n-cobol-RequestCall session-owner',
                            'call "TT_PLAN" using by value engine\n       by reference "decide" by value 6\n       by reference tt-n-cobol-RequestCall plan-owner')
        plan += """    if status-code not = 0 move 20 to return-code goback end-if
    set plan-sentinel to plan-owner
    call "TT_PLAN" using by value engine
       by reference "other" by value 5
       by reference tt-n-cobol-RequestCall plan-owner returning status-code
    if status-code = 0 or plan-owner not = plan-sentinel
       move 24 to return-code goback end-if
    call "TT_PLAN" using by value engine
       by reference "decide" by value 6
       by reference tt-n-cobol-RequestCall by value null-output returning status-code
    if status-code not = tt-n-cobol-representation-k
       move 25 to return-code goback end-if
    call "thinkthen_engine_free" using by value engine returning omitted
    call "TT_VALUE_FIELD" using by value plan-owner
       by reference "records" by value 7 by reference records-owner
       returning status-code
    if status-code not = 0 move 21 to return-code goback end-if
    call "TT_VALUE_FREE" using by value plan-owner returning omitted
    call "TT_VALUE_TEXT" using by value records-owner
       by reference plan-text plan-written returning status-code
    call "TT_VALUE_FREE" using by value records-owner returning omitted
    if status-code not = 0 move 22 to return-code goback end-if
    call "TT_TEST_VALUE_OWNERS" returning status-code
    if status-code not = 0 move 23 to return-code goback end-if
    display "PLAN_RECORDS " plan-text(1:plan-written)
    move 0 to return-code goback.
"""
        plan_source = root / 'plan.cob'; plan_source.write_text(plan)
        compile_caller(plan_source, root / 'plan')
        plan_question = root / 'plan-question.json'; plan_question.write_text(json.dumps(questions['decide']))
        before = len(backend.arrivals)
        result = subprocess.run([str(root / 'plan')], env=dict(env, TT_SESSION_QUESTION_FILE=str(plan_question),
                                TT_SESSION_EVIDENCE='Refund me.', TT_SESSION_EXPECT_FAILURE='N'),
                                capture_output=True, text=True, timeout=5)
        assert result.returncode == 0 and not result.stderr, (result.returncode, result.stdout, result.stderr)
        assert result.stdout.strip() == 'PLAN_RECORDS 1', result.stdout
        assert len(backend.arrivals) == before, 'COBOL typed plan sent a request'
        for function, question in questions.items():
            source = root / (function + '.cob')
            caller = template.replace('TT_SESSION_DECIDE', 'TT_SESSION_' + function.upper())
            if function in ('find', 'relate'):
                selection = 'units' if function == 'find' else 'entities'
                caller = caller.replace('RequestInputRecords', 'RequestInput' + selection.title()).replace('requestinputrecords', 'requestinput' + selection)
            if function == 'find':
                caller = caller.replace('01 record-address usage pointer.', '01 record-address.\n 02 first-record usage pointer.\n 02 second-record usage pointer.').replace('set record-address to address of tt-n-cobol-RequestItem', 'set first-record second-record to address of tt-n-cobol-RequestItem').replace('move 1 to v-len of tt-n-cobol-RequestInputUnitsItems', 'move 2 to v-len of tt-n-cobol-RequestInputUnitsItems')
            if function == 'relate':
                caller = caller.replace('01 record-address usage pointer.', '01 record-address usage pointer.\n01 entity-value usage pointer.').replace('    call "TT_TEXT_ITEM" using', '    call "TT_TEST_ENTITY" using by reference evidence-text by value evidence-length returning entity-value\n    call "TT_TEXT_ITEM" using').replace('evidence-text by value evidence-length returning status-code', 'by value entity-value returning status-code')
                caller = caller.replace('    if status-code not = 0\n', '    call "TT_VALUE_FREE" using by value entity-value returning omitted\n    if status-code not = 0\n', 1)
                caller = caller.replace('TT_TEXT_ITEM', 'TT_JSON_ITEM').replace('RequestOriginalTextText', 'RequestOriginalJsonValue').replace('RequestOriginalText', 'RequestOriginalJson').replace('requestoriginaltext', 'requestoriginaljson').replace('v-m-text of tt-n-cobol-RequestOriginalJson', 'v-m-value of tt-n-cobol-RequestOriginalJson')
            source.write_text(caller)
            executable = root / function
            compile_caller(source, executable)
            question_file = root / 'question.json'
            question_file.write_text(json.dumps(question))
            run_env = dict(env, TT_SESSION_QUESTION_FILE=str(question_file), TT_SESSION_EVIDENCE='Refund me.', TT_SESSION_EXPECT_FAILURE='N')
            if function == 'relate':
                run_env['TT_SESSION_EVIDENCE'] = 'Sam'
            result = subprocess.run([str(executable)], env=run_env, capture_output=True, text=True, timeout=5)
            assert result.returncode == 0 and 'SESSION_PASS' in result.stdout, (function, result.stdout, result.stderr)
            primitive = {'decide': 'VALUE 0000000001', 'choose': 'CHOICE a', 'score': 'SCORE '}
            if function in primitive:
                assert primitive[function] in result.stdout, (function, result.stdout)
            if function == 'score':
                number = next(line.removeprefix('SCORE ') for line in result.stdout.splitlines() if line.startswith('SCORE '))
                assert abs(float(number) - 0.1) < 1e-12, result.stdout
        source = root / 'capacity.cob'
        source.write_text('''identification division.
program-id. CapacityCaller.
data division.
working-storage section.
copy "tt-native-generated.cpy".
01 evidence pic x(8193) value all "x".
01 output-text pic x(8192) value all "z".
01 written usage binary-double unsigned value 123.
01 status-code usage binary-long signed.
procedure division.
 allocate tt-n-complete-utf8-v1
 set v-data of tt-n-complete-utf8-v1 to address of evidence
 move 8193 to v-len of tt-n-complete-utf8-v1
 call "TT_SESSION_TEXT" using by reference tt-n-complete-utf8-v1
    output-text written returning status-code
 if status-code not = tt-n-cobol-overflow-k or written not = 123
    or output-text not = all "z" move 1 to return-code goback end-if
 move 8192 to v-len of tt-n-complete-utf8-v1
 call "TT_SESSION_TEXT" using by reference tt-n-complete-utf8-v1
    output-text written returning status-code
 if status-code not = 0 or written not = 8192 or output-text not = all "x"
    move 2 to return-code goback end-if
 free tt-n-complete-utf8-v1
 move 0 to return-code goback.
''')
        compile_caller(source, root / 'capacity')
        subprocess.run([str(root / 'capacity')], env=env, check=True, timeout=5)
        # A feed waits for caller input, so cancellation has no provider race.
        caller = template.replace('    call "TT_SESSION_DECIDE"', '''    allocate tt-n-cobol-RequestInputFeed
    allocate tt-n-cobol-RequestInputFeedName
    move low-values to tt-n-cobol-RequestInputFeed
    set v-data of tt-n-cobol-RequestInputFeedName
       to address of evidence-text
    move evidence-length to v-len of tt-n-cobol-RequestInputFeedName
    set v-m-name of tt-n-cobol-RequestInputFeed
       to address of tt-n-cobol-RequestInputFeedName
    move tt-n-cobol-requestinputfeed-kind-k
       to v-kind of tt-n-cobol-RequestInput
    set v-value of tt-n-cobol-RequestInput
       to address of tt-n-cobol-RequestInputFeed
    call "TT_SESSION_DECIDE"''')
        caller = caller.replace('    free tt-n-cobol-RequestCall', '''    if status-code = 0
       call "thinkthen_session_cancel" using by value session-owner returning omitted
    end-if
    free tt-n-cobol-RequestInputFeed tt-n-cobol-RequestInputFeedName
    free tt-n-cobol-RequestCall''')
        source = root / 'cancel.cob'
        source.write_text(caller)
        compile_caller(source, root / 'cancel')
        question_file.write_text(json.dumps(questions['decide']))
        before = len(backend.arrivals)
        run_env = dict(env, TT_SESSION_QUESTION_FILE=str(question_file), TT_SESSION_EVIDENCE='cancel-feed', TT_SESSION_EXPECT_FAILURE='Y')
        result = subprocess.run([str(root / 'cancel')], env=run_env, capture_output=True, text=True, timeout=5)
        assert result.returncode == 0 and 'ERROR_KIND 0000000004' in result.stdout, (result.stdout, result.stderr)
        assert len(backend.arrivals) == before, backend.arrivals
        before = len(backend.arrivals)
        question_file.write_text(json.dumps(questions['decide']))
        for evidence, failure, expected in [('no', 'N', 'VALUE 0000000000'), ('unsure', 'N', 'VALUE_NULL'), ('status-401', 'Y', 'ERROR_KIND 0000000002'), ('x' * 8193, 'N', 'ADMISSION +0000001002')]:
            question_file.write_text(json.dumps({'decide': 'Refund?', 'threshold': '0.4:0.6'} if evidence == 'unsure' else questions['decide']))
            run_env = dict(env, TT_SESSION_QUESTION_FILE=str(question_file), TT_SESSION_EVIDENCE=evidence, TT_SESSION_EXPECT_FAILURE=failure)
            result = subprocess.run([str(root / 'decide')], env=run_env, capture_output=True, text=True, timeout=5)
            assert result.returncode == 0 and expected in result.stdout, (evidence, result.stdout, result.stderr)
            if evidence == 'status-401':
                assert 'FAILURE_REQUESTS 00000000000000000001' in result.stdout, result.stdout
        assert len(backend.arrivals) == before + 3, backend.arrivals
        before = len(backend.arrivals)
        question_file.write_text(json.dumps({'decide': 'Refund?', 'threshold': -1}))
        run_env = dict(env, TT_SESSION_QUESTION_FILE=str(question_file), TT_SESSION_EVIDENCE='invalid', TT_SESSION_EXPECT_FAILURE='Y')
        result = subprocess.run([str(root / 'decide')], env=run_env, capture_output=True, text=True, timeout=5)
        assert result.returncode == 0 and 'ERROR_KIND 0000000003' in result.stdout, (result.stdout, result.stderr)
        assert len(backend.arrivals) == before, backend.arrivals
        source = root / 'invalid.cob'
        source.write_text(template.replace('TT_QUESTION_FILE', 'TT_QUESTION_TEXT').replace('RequestQuestionFilePath', 'RequestQuestionTextText')
                          .replace('RequestQuestionFile', 'RequestQuestionText')
                          .replace('requestquestionfile', 'requestquestiontext')
                          .replace('v-m-path', 'v-m-text'))
        compile_caller(source, root / 'invalid')
        run_env = dict(env, TT_SESSION_QUESTION_FILE='', TT_SESSION_EVIDENCE='invalid', TT_SESSION_EXPECT_FAILURE='Y')
        result = subprocess.run([str(root / 'invalid')], env=run_env, capture_output=True, text=True, timeout=5)
        assert result.returncode == 0 and ('ADMISSION +0000000001' in result.stdout or 'ERROR_KIND 0000000001' in result.stdout), (result.stdout, result.stderr)
        assert len(backend.arrivals) == before, backend.arrivals
        print('installed COBOL: ten named sessions, owned primitive values/facts, typed failure, null, cancellation and zero-send/8192-byte refusals PASS')
        assert all(agent == 'thinkthen/0.2.0 (cobol)' for agent in backend.user_agents), backend.user_agents
    finally:
        backend.close()
