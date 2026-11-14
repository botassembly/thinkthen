"""Compile the literal C guide once and execute every observable branch."""
import json
import os
from pathlib import Path
import re
import shlex
import subprocess
import tempfile

from guide_examples import ROOT, IDEAL, blocks, check, scenario
from bash_guide_examples import FAILURE, cases
from rust_guide_support import rank_fixture

MESSAGES=['Please refund my order.','Where is my parcel?','I was charged twice for one invoice.']
TOPIC='enum topic { TOPIC_BILLING,TOPIC_SHIPPING,TOPIC_OTHER };\nstatic const char *const topic_words[]={"billing","shipping","other"};\n'
URGENCY='enum urgency { URGENCY_ROUTINE,URGENCY_URGENT };\nstatic const char *const urgency_words[]={"routine","urgent"};\n'
ASK='''thinkthen_question *asks_refund=NULL;
thinkthen_row is_refund=THINKTHEN_ROW_INIT;
rc=thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE,"Does the customer request a refund?",THINKTHEN_NUL_TERMINATED,&asks_refund,&error);
'''


def program(examples):
    functions=[]
    setup=examples[1][1].split('if (rc == THINKTHEN_OK) {',1)[0]
    for index,(_,body,_) in enumerate(examples):
        if index in (0,5):
            full=body.replace('int main(void)',f'static int example_{index}(void)')
            if index==5:full=full.replace('enum topic','enum example_topic').replace('topic_words','example_topic_words').replace('TOPIC_','EXAMPLE_TOPIC_')
            functions.append(full)
            continue
        prefix='thinkthen_error error=THINKTHEN_ERROR_INIT; int rc=THINKTHEN_OK;\n'
        suffix=''
        if index == 7:
            functions.append(body)
            continue
        if index not in (22,24):
            prefix+='thinkthen_client *client=NULL; rc=thinkthen_client_open(NULL,&client,&error);\n'
        if index == 2: prefix+=setup
        if index in (19,22,23,24): prefix+=ASK
        if index==10: prefix+=examples[9][1]
        if index==18:
            prefix+='thinkthen_question *asks_refund=NULL;thinkthen_question *asks_topic=NULL;thinkthen_question *asks_urgency=NULL;thinkthen_question *questions=NULL;thinkthen_row form=THINKTHEN_ROW_INIT;\n'
            prefix+='rc = thinkthen_question_new('+examples[17][1].split('rc = thinkthen_question_new(',1)[1].split('if (rc == THINKTHEN_OK)\n    rc = thinkthen_call_open',1)[0]
            suffix='if(rc==THINKTHEN_OK) puts(thinkthen_row_string(&form));'
        if index==19:
            prefix+='thinkthen_question *asks_topic=NULL;\nif(rc==THINKTHEN_OK) rc=thinkthen_question_new(THINKTHEN_FUNCTION_CHOOSE,"What is the message about?",THINKTHEN_NUL_TERMINATED,&asks_topic,&error);\nif(rc==THINKTHEN_OK) rc=thinkthen_question_set_choices(asks_topic,topic_words,3,&error);\n'
            suffix='''const thinkthen_question *saved_questions[]={asks_refund,asks_topic};
for(size_t i=0;i<2 && rc==THINKTHEN_OK;i++){thinkthen_buffer json={0};rc=thinkthen_question_to_json(saved_questions[i],&json,&error);if(rc==THINKTHEN_OK)printf("%.*s\\n",(int)json.len,json.data);thinkthen_buffer_free(&json);}'''
        if index in (20,22): suffix='if(rc==THINKTHEN_OK) puts(thinkthen_row_string(&is_refund));'
        if index==21:
            body=body.replace('thinkthen_row_free(&topic);','if(rc==THINKTHEN_OK) puts(thinkthen_row_string(&topic));\nthinkthen_row_free(&topic);')
        if index==2: suffix='if(rc==THINKTHEN_UNSURE)rc=THINKTHEN_OK;'
        # Prefixes supply only the documented surrounding main; the fence itself is unchanged.
        text=prefix+body+'\n'+suffix
        cleanup=[]
        for kind,name in re.findall(r'thinkthen_(row|question|call|settings|client|toolbox|cancel)\s*\*?\s*(\w+)\s*=',text):
            if re.search(r'thinkthen_'+kind+r'_free\(\s*&?'+name+r'\s*\)',text): continue
            cleanup.append(f'thinkthen_{kind}_free({"&" if kind=="row" else ""}{name});')
        footer='if(rc==THINKTHEN_END)rc=THINKTHEN_OK;\n'
        if index!=3:
            footer+='if(rc<0 && error.code)fprintf(stderr,"%s\\n",thinkthen_error_string(&error));\n'
        footer+='\n'.join(cleanup)+'\nthinkthen_error_free(&error);return rc==THINKTHEN_OK ? 0:1;'
        functions.append(f'static int example_{index}(void) {{\n{text}\n{footer}\n}}')
    return '#include <thinkthen.h>\n#include <stdio.h>\n#include <stdlib.h>\n#include <math.h>\n#include <inttypes.h>\n'+TOPIC+URGENCY+'\n'.join(functions)+'\nint main(int argc,char **argv){if(argc!=2)return 2;switch(atoi(argv[1])){'+''.join(f'case {i}:return example_{i}();' for i in range(len(examples)) if i!=7)+'default:return 2;}}\n'


def expected(index,name):
    failed=name=='failed' or name.endswith('-failed')
    route={'yes':'refund queue','no':'inbox','unsure':'a person reads it','failed':'try again later'}
    code=0
    output=''
    if index in (0,1,2):output='' if failed else route[name]+'\n';code=int(failed)
    elif index==3:output='' if failed else ('false' if name=='no' else 'true')+'\n';code=int(failed)
    elif index==4:output='' if failed else {'yes':'0.95\n','no':'0.05\n','unsure':'0.5\n'}[name];code=int(failed)
    elif index==5:output='' if failed else {'yes':'billing team','no':'warehouse','other':'front desk','unsure':'a person reads it'}[name]+'\n';code=int(failed)
    elif index==6:output=''.join(route[name]+': '+m+'\n' for m in MESSAGES)
    elif index==8:output=({'yes':'refund queue','no':'inbox','unsure':'a person reads it: 0.5','failed':'try again later: backend.status'}[name]+'\n')*3
    elif index in (9,10):
        if name=='yes':output=''.join('refund queue: '+m+'\n' for m in MESSAGES)
        elif failed and index==10:output=''.join('try again later: '+m+' ('+FAILURE+')\n' for m in MESSAGES)
        code=int(failed)
    elif index==11:output='' if name in ('failed','no') else 'billing team\nbilling\nshipping\n';code=int(failed)
    elif index==12:output='' if failed else '0.00\n' if name=='low-score' else 'answer today\n1.00\n';code=int(failed)
    elif index==13:
        order=[2,0,1] if name=='rank-example' else [0,1,2]
        output='' if failed else ''.join(f'{i+1} {MESSAGES[pos]}\n' for i,pos in enumerate(order));code=int(failed)
    elif index==14:output='' if failed else {'yes':'Please refund my order.','no':'no line answers','unsure':'a person reads it'}[name]+'\n';code=int(failed)
    elif index==15:output='person Morgan\norganization Example Group\n' if name=='relations' else '';code=int(failed)
    elif index==16:output='Morgan works_for Example Group\n' if name=='relations' else '';code=int(failed)
    elif index==17:
        if failed:output=''.join('try again later: '+m+'\n' for m in MESSAGES)
        elif name.endswith('unsure'):output=''.join('a person reads it: '+m+'\n' for m in MESSAGES)
        elif name=='yes':output='refund queue: billing 1.00\n'*3
    elif index==18:
        if failed:code=1
        else:output='refund='+('false' if name=='no' else 'true')+'\ttopic='+('shipping' if name=='no' else 'unsure' if name.endswith('unsure') else 'billing')+'\turgency=1\n'
    elif index in (20,22):output='' if failed else ('unsure' if name=='unsure' else 'false' if name=='no' else 'true')+'\n';code=int(failed)
    elif index==21:output='' if failed else ('unsure' if name=='unsure' else 'billing')+'\n';code=int(failed)
    elif index==23:output='1 0 0\n'
    elif index==24:pass
    return output,code


def run_c(command=None,negative=None,flags=None):
    examples=blocks(IDEAL/'c.md')
    assert len(examples)==25 and all(lang=='c' for lang,_,_ in examples),'C guide inventory changed'
    target=Path(os.environ.get('CARGO_TARGET_DIR',ROOT/'target')).resolve()
    flags=shlex.split(flags) if flags else ['-I',str(ROOT/'crates/thinkthen-ffi/include'),'-L',str(target/'debug'),'-lthinkthen_ffi','-Wl,-rpath,'+str(target/'debug')]
    scripted=cases();scripted['rank-example']=rank_fixture()[0]
    with tempfile.TemporaryDirectory(prefix='c-ideal-guide-') as directory:
        folder=Path(directory);source=folder/'guide.c';source.write_text(program(examples));executable=folder/'guide'
        subprocess.run(['cc','-std=c11','-Wall','-Wextra','-Werror',str(source),*flags,'-o',str(executable)],check=True)
        for index,(_,_,line) in enumerate(examples):
            if index==7:
                print(f'PASS c.md:{line}/helper-called-by-next-bulk-block')
                continue
            names=['yes','failed']
            if index in (0,9,10,11):names=['yes','no','failed']
            if index in (1,2,3,4,5,6,8,14,20):names=['yes','no','unsure','failed']
            if index==5:names.append('other')
            if index==12:names=['yes','low-score','failed']
            if index==13:names=['yes','rank-example','failed']
            if index in (15,16):names=['relations','empty','failed']
            if index in (17,18):names=['yes','no','failed','form-refund-failed','form-topic-failed','form-urgency-failed','form-topic-unsure']
            if index in (19,23,24):names=['yes']
            if index==21:names=['yes','unsure','failed']
            for name in names:
                with scenario(name.removeprefix('form-') if name not in scripted else 'yes',case=scripted.get(name)) as (working,env):
                    (working/'toolbox.json').write_text(json.dumps({'entries':{'topic':{'question':{'function':'choose','question':'What is the message about?','choices':['billing','shipping','other'],'field':'/text'}}}}))
                    result=subprocess.run([str(executable),str(index)],cwd=working,env=env,capture_output=True,text=True)
                    output,status=expected(index,name)
                    if index==19:
                        values=[json.loads(line) for line in result.stdout.splitlines()];assert len(values)==2,values
                        value,topic=values
                        assert value['yes_means']=='the customer wants money back, including for a duplicate charge' and value['no_means']=='the customer wants anything else',value
                        descriptions={entry['name']:entry['description'] for entry in topic['choices'] if isinstance(entry,dict)}
                        assert descriptions=={'billing':'charges, invoices and refunds','shipping':'parcels and delivery'},topic
                        check(f'c.md:{line}/{name}',result,output=result.stdout,negative=negative)
                    else:
                        kwargs=dict(output=output,exit_code=status)
                        if index in (0,1,2,5,6,8,17,18) and output:kwargs['branch']=output.splitlines()[0]
                        if status:kwargs['refusal']=r'backend\.(status|reply)'
                        if index==3 and name=='failed':
                            assert result.stderr=='the call did not complete: '+FAILURE.removeprefix('failed: ')+'\n',result.stderr
                        check(f'c.md:{line}/{name}',result,negative=negative,**kwargs)
        refusals(examples,folder,flags,negative)


def refusals(examples,folder,flags,negative):
    snippets={
        'if-row':'thinkthen_row row=THINKTHEN_ROW_INIT;if(row)return 1;',
        'bare-answer-equality':'thinkthen_row row=THINKTHEN_ROW_INIT;if(row==true)return 1;',
        'row-equality':'thinkthen_row row=THINKTHEN_ROW_INIT,other=THINKTHEN_ROW_INIT;if(row==other)return 1;',
        'missing-state-case':'thinkthen_row row=THINKTHEN_ROW_INIT;switch(row.status){case THINKTHEN_STATUS_ANSWERED:break;case THINKTHEN_STATUS_UNSURE:break;}',
        'missing-enum-case':'enum topic{BILLING,SHIPPING,OTHER};enum topic topic=BILLING;switch(topic){case BILLING:break;case SHIPPING:break;}',
        'pointer-address':'thinkthen_row row=THINKTHEN_ROW_INIT;if(&row)return 1;',
        'ignored-status':'thinkthen_row row=THINKTHEN_ROW_INIT;bool yes=false;thinkthen_row_answer_bool(&row,&yes,NULL);',
        'quiet-answer-field':'thinkthen_row row=THINKTHEN_ROW_INIT;return row.answer_bool;',
    }
    for name,body in snippets.items():
        source=folder/(name+'.c');source.write_text('#include <thinkthen.h>\nint main(void){'+body+'return 0;}\n')
        result=subprocess.run(['cc','-std=c11','-Wall','-Wextra','-Werror',str(source),*flags,'-o',str(folder/name)],capture_output=True,text=True)
        if negative=='refusal':result.returncode=0
        diagnostic={'if-row':r'scalar|struct type', 'bare-answer-equality':r'invalid operands|incompatible',
                    'row-equality':r'invalid operands|incompatible', 'missing-state-case':r'not handled',
                    'missing-enum-case':r'not handled', 'pointer-address':r'always.*true|always evaluate',
                    'ignored-status':r'ignoring return value', 'quiet-answer-field':r'no member'}[name]
        assert result.returncode!=0 and re.search(diagnostic,result.stderr),name+': promised compile refusal accepted or wrong diagnostic\n'+result.stderr
        print('PASS c/refusal/'+name)
    source=folder/'pointer-parameter.c'
    source.write_text('#include <thinkthen.h>\nstatic int truth(const thinkthen_row *row){return row ? 1:0;}\nint main(void){return truth(NULL);}')
    subprocess.run(['cc','-std=c11','-Wall','-Wextra','-Werror',str(source),*flags,'-o',str(folder/'pointer-parameter')],check=True)
    print('PASS c/pointer-parameter-is-allowed')
    executable=folder/'ideal-rows'
    subprocess.run(['cc','-std=c11','-Wall','-Wextra','-Werror','-UNDEBUG',str(ROOT/'sdk/c/tests/ideal_rows.c'),*flags,'-o',str(executable)],check=True)
    for mode,names in [('states',['yes','no','unsure','failed']),('mismatches',['yes','unsure','failed']),('ownership',['yes']),('selected',['yes']),('details',['relations','failed']),('forms',['yes','failed','form-refund-failed','form-topic-failed','form-urgency-failed','form-topic-unsure','form-refund-unsure']),('admission',['yes']),('filter',['yes']),('cancel',['yes'])]:
        for name in names:
            with scenario(name.removeprefix('form-')) as (working,env):
                result=subprocess.run([str(executable),mode,name],cwd=working,env=env,capture_output=True,text=True)
                check('c/runtime/'+mode+'/'+name,result,output=mode+'\n',negative=negative)
