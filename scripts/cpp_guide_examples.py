"""Compile literal C++20 guide programs once, then assert their scenario branches."""
import json
import os
from pathlib import Path
import shlex
import subprocess
import tempfile
from guide_examples import ROOT, IDEAL, blocks, check, scenario
from bash_guide_examples import FAILURE, cases
from rust_guide_support import rank_fixture

MESSAGES=['Please refund my order.','Where is my parcel?','I was charged twice for one invoice.']
HEAD='''#include <thinkthen/thinkthen.hpp>
#include <array>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <iterator>
#include <string_view>
#include <utility>
namespace tt = thinkthen;
'''
ASK='const auto asks_refund = tt::decide{"Does the customer request a refund?"}.threshold(tt::threshold::band(0.2,0.8));\n'

def program(examples):
    choice=examples[5][1]
    domain=choice[choice.index('enum class Topic'):choice.index('int main()')]
    result=[HEAD,domain,examples[13][1].replace('Topic','TagTopic'),examples[15][1],examples[9][1]]
    many=examples[6][1].split('auto refund_checks')[0]
    for i,(_,body,_) in enumerate(examples):
        if i in (13,15):
            result.append(f'static int example_{i}() {{ return 0; }}')
            continue
        if i in (0,5):
            body=body[body.index('int main()'):]
            result.append(body.replace('int main()',f'static int example_{i}()').replace('} catch (const std::exception& error)', 'return 0;\n} catch (const std::exception& error)'))
            continue
        prefix='tt::client client;\n' if i not in (26,28) else ''
        suffix=''
        if i==2:prefix+=examples[1][1].split('if (is_refund.answered())')[0]
        if i in (7,8,9,10):prefix+=many
        if i==9:body=examples[10][1]
        if i in (26,27,28):prefix+=ASK
        if i==12:prefix+=examples[11][1].split('for (const auto& refund_request')[0]
        if i==14:body=body.replace('Topic','TagTopic')
        if i==22:prefix+=examples[21][1].split('auto forms =')[0]
        if i==23:suffix="""
for(const auto* question : {asks_refund.native_handle(),asks_topic.native_handle()}) {
    thinkthen_buffer value{}; tt::detail::invoke(thinkthen_question_to_json,question,&value);
    std::cout.write(reinterpret_cast<const char*>(value.data),value.len);std::cout << '\\n';thinkthen_buffer_free(&value);
}"""
        if i in (24,26,28):suffix='std::cout << is_refund << "\\n";'
        if i==25:suffix='std::cout << topic << "\\n";'
        result.append(f"""static int example_{i}() try {{
{prefix}{body}
{suffix}
return 0;
}} catch(const std::exception& error) {{ std::cerr << error.what() << '\\n'; return 1; }}
""")
    result.append('int main(int argc,char** argv) {if(argc!=2)return 2; switch(std::atoi(argv[1])){'+''.join(f'case {i}:return example_{i}();' for i in range(len(examples)))+'default:return 2;}}')
    return '\n'.join(result)


def expected(i,name):
    failed=name=='failed' or name.endswith('-failed')
    route={'yes':'refund queue','no':'inbox','unsure':'a person reads it','failed':'try again later'}
    output='';status=0
    if i==0:output='' if failed else ('inbox' if name=='no' else 'refund queue')+'\n';status=int(failed)
    elif i in (1,2):output='' if failed else route[name]+'\n';status=int(failed)
    elif i==3:output='could not judge it: '+FAILURE.removeprefix('failed: ')+'\n' if failed else ('false' if name=='no' else 'true')+'\n'
    elif i==4:output='' if failed else {'yes':'0.95\n','no':'0.05\n','unsure':'0.5\n'}[name];status=int(failed)
    elif i==5:output='' if failed else {'yes':'billing team','no':'warehouse','other':'front desk','unsure':'a person reads it'}[name]+'\n';status=int(failed)
    elif i==6:output=''.join(route[name]+': '+m+'\n' for m in MESSAGES)
    elif i==7:output='' if failed else ('unsure' if name=='unsure' else 'false' if name=='no' else 'true')+'\n';output*=3;status=int(failed)
    elif i==8:output=(route[name]+'\n')*3
    elif i in (9,10):
        text={'yes':'refund queue','no':'inbox','unsure':'a person reads it: 0.5','failed':'try again later: backend.status'}[name]
        output=''.join(text+': '+m+'\n' for m in MESSAGES)
    elif i in (11,12):
        if name=='yes':output=''.join('refund queue: '+m+'\n' for m in MESSAGES)
        elif failed and i==12:output=''.join('try again later: '+m+' ('+FAILURE+')\n' for m in MESSAGES)
        status=int(failed and i==11)
    elif i==14:output='' if name in ('failed','no') else 'billing team\nbilling\nshipping\n';status=int(failed)
    elif i==16:output='' if failed else '0.00\n' if name=='low-score' else 'answer today\n1.00\n';status=int(failed)
    elif i==17:
        order=[2,0,1] if name=='rank-example' else [0,1,2]
        output='' if failed else ''.join(f'{n+1} {MESSAGES[pos]}\n' for n,pos in enumerate(order));status=int(failed)
    elif i==18:output='' if failed else {'yes':'Please refund my order.','no':'no line answers','unsure':'a person reads it'}[name]+'\n';status=int(failed)
    elif i==19:output='person Morgan\norganization Example Group\n' if name=='relations' else '';status=int(failed)
    elif i==20:output='Morgan works_for Example Group\n' if name=='relations' else '';status=int(failed)
    elif i==21:
        if failed:output=''.join('try again later: '+m+'\n' for m in MESSAGES)
        elif name.endswith('unsure'):output=''.join('a person reads it: '+m+'\n' for m in MESSAGES)
        elif name=='yes':output='refund queue: billing 1.00\n'*3
    elif i==22:status=int(failed)
    elif i in (24,26,28):output='' if failed else ('unsure' if name=='unsure' else 'false' if name=='no' else 'true')+'\n';status=int(failed)
    elif i==25:output='' if failed else ('unsure' if name=='unsure' else 'shipping' if name=='no' else 'billing')+'\n';status=int(failed)
    elif i==27:output='1 0 0\n'
    return output,status


def run_cpp(command=None,negative=None,contract_only=False):
    examples=blocks(IDEAL/'cpp.md')
    assert len(examples)==29 and all(lang=='cpp' for lang,_,_ in examples),'C++ guide inventory changed'
    target=Path(os.environ.get('CARGO_TARGET_DIR',ROOT/'target')).resolve()
    flags=['-I',str(ROOT/'sdk/cpp/include'),'-I',str(ROOT/'crates/thinkthen-ffi/include'),'-L',str(target/'debug'),'-lthinkthen_ffi','-Wl,-rpath,'+str(target/'debug')]
    if os.environ.get('THINKTHEN_CPP_GUIDE_FLAGS'):flags=shlex.split(os.environ['THINKTHEN_CPP_GUIDE_FLAGS'])
    compiler=shlex.split(os.environ.get('CXX','c++'))
    scripted=cases();scripted['rank-example']=rank_fixture()[0]
    with tempfile.TemporaryDirectory(prefix='cpp-ideal-guide-') as directory:
        folder=Path(directory)
        if not contract_only:
            source=folder/'guide.cpp';source.write_text(program(examples));executable=folder/'guide'
            subprocess.run([*compiler,'-std=c++20','-Wall','-Wextra','-Werror',str(source),*flags,'-o',str(executable)],check=True)
            for i,(_,_,line) in enumerate(examples):
                names=['yes','failed']
                if i in (0,1,2,3,4,5,6,7,8,9,10,18,24,25):names=['yes','no','unsure','failed']
                if i==5:names.append('other')
                if i in (11,12,14):names=['yes','no','failed']
                if i==16:names=['yes','low-score','failed']
                if i==17:names=['yes','rank-example','failed']
                if i in (19,20):names=['relations','empty','failed']
                if i in (21,22):names=['yes','no','failed','form-refund-failed','form-topic-failed','form-urgency-failed','form-refund-unsure','form-topic-unsure']
                if i in (13,15,23,27):names=['yes']
                for name in names:
                    with scenario(name.removeprefix('form-') if name not in scripted else 'yes',case=scripted.get(name)) as (working,env):
                        (working/'toolbox.json').write_text(json.dumps({'entries':{'topic':{'question':{'function':'choose','question':'What is the message about?','choices':['billing','shipping','other'],'field':'/text'}}}}))
                        result=subprocess.run([str(executable),str(i)],cwd=working,env=env,capture_output=True,text=True,timeout=60)
                        output,status=expected(i,name)
                        if i==23:
                            values=[json.loads(line) for line in result.stdout.splitlines()];assert len(values)==2,values
                            assert values[0]['yes_means']=='the customer wants money back, including for a duplicate charge',values
                            assert values[0]['no_means']=='the customer wants anything else',values
                            descriptions={entry['name']:entry['description'] for entry in values[1]['choices'] if isinstance(entry,dict)}
                            assert descriptions=={'billing':'charges, invoices and refunds','shipping':'parcels and delivery'},values
                            output=result.stdout
                        check(f'cpp.md:{line}/{name}',result,output=output,exit_code=status,
                              refusal=r'backend\.(status|reply)' if status and (name=='failed' or name.endswith('-failed')) else None,
                              negative=negative)
                        if i==0 and name=='yes' and negative is None:
                            for control in ('output','branch','exit'):
                                try:
                                    check('cpp/control/'+control,result,output=output,negative=control)
                                except AssertionError as error:
                                    assert {'output':'deliberately wrong output','branch':'deliberately wrong branch','exit':'expected 99'}[control] in str(error)
                                else:raise AssertionError('runner accepted wrong '+control)
                                print('PASS cpp runner rejects wrong '+control)
        witness=folder/'retained-rows'
        json_include=Path.home()/'.local/share/thinkthen-build-tools/nlohmann-json/3.11.3/include'
        subprocess.run([*compiler,'-std=c++20','-Wall','-Wextra','-Werror','-UNDEBUG','-I',str(json_include),str(ROOT/'sdk/cpp/tests/ideal_rows.cpp'),*flags,'-o',str(witness)],check=True)
        for name in ('yes','unsure','failed'):
            with scenario(name) as (working,env):
                result=subprocess.run([str(witness),name],cwd=working,env=env,capture_output=True,text=True,timeout=60)
                output='retained original records, form refusals, native print and cancellation\n' if name=='yes' else 'safe state and loud answer: '+name+'\n'
                check('cpp/retained-and-refusals/'+name,result,output=output,negative=negative)
        for name in ('form-refund-failed','form-topic-failed','form-urgency-failed','form-refund-unsure','form-topic-unsure'):
            with scenario(name.removeprefix('form-')) as (working,env):
                result=subprocess.run([str(witness),name],cwd=working,env=env,capture_output=True,text=True,timeout=60)
                check('cpp/form-failure/'+name,result,output=name+': aggregate failure '+str(name.endswith('failed')).lower()+'\n',negative=negative)
        if not contract_only:
            refusals(folder,compiler,flags,negative)
            if negative is None:
                try:refusals(folder,compiler,flags,'refusal')
                except AssertionError as error:assert 'promised compile refusal accepted' in str(error)
                else:raise AssertionError('runner accepted a missing refusal')
                print('PASS cpp runner rejects wrong refusal')


def refusals(folder,compiler,flags,negative):
    snippets={
        'removed-failure-name':'tt::record_error* value=nullptr;(void)value;',
        'choose-bool':'tt::client c; auto r=c.choose(tt::choose{"Q",{"a","b"}},"text"); if(r)return 0;',
        'row-equality':'tt::client c;auto a=c.decide(tt::decide{"Q"},"text");if(a==a)return 0;',
        'bare-equality':'tt::client c;auto a=c.decide(tt::decide{"Q"},"text");if(a==true)return 0;',
        'copy-row':'tt::client c;auto a=c.decide(tt::decide{"Q"},"text");auto b=a;(void)b;',
        'quiet-value':'tt::client c;auto a=c.decide(tt::decide{"Q"},"text");(void)a.value();',
        'filter-band':'auto q=tt::filter{"Q"}.threshold(tt::threshold::band(.2,.8));(void)q;',
        'filter-one':'tt::client c;(void)c.decide(tt::filter{"Q"},"text");',
        'rank-one':'tt::client c;(void)c.score(tt::rank{"Q",{"a","b"}},"text");',
        'list-one':'tt::client c;(void)c.decide(tt::decide{"Q"},std::vector<std::string>{"text"});',
        'missing-words':'enum class Choice{a,b};tt::choose<Choice> q{"Q"};(void)q;',
        'missing-state':'tt::status s=tt::status::answered;switch(s){case tt::status::answered:break;case tt::status::unsure:break;}',
        'missing-enum':'enum class Choice{a,b,c};Choice x=Choice::a;switch(x){case Choice::a:break;case Choice::b:break;}',
        'ignored-call':'tt::client c;c.decide(tt::decide{"Q"},"text");',
        'pointer-address':'tt::client c;auto a=c.decide(tt::decide{"Q"},"text");if(&a)return 0;',
        'tag-shortcut':'tt::client c;auto r=c.tag(tt::tag{"Q",{"a","b"}},"text");(void)r.contains("a");',
        'filter-kept-shortcut':'auto witness=[](const tt::row<tt::filter,std::string>& row){(void)row.kept();};(void)witness;',
        'filter-every-row':'tt::client c;(void)c.filter_rows_each(tt::filter{"Q"},std::vector<std::string>{"text"});',
        'filter-one-removed':'tt::client c;(void)c.filter(tt::filter{"Q"},"text");',
        'rank-one-removed':'tt::client c;(void)c.rank(tt::rank{"Q",{"a","b"}},"text");',
        'settings-order':'tt::settings s{.batch=16,.model="m"};(void)s;',
        'filter-form':'tt::annotate q{{"filter",tt::filter{"Q"}}};(void)q;',
        'filter-form-get':'tt::client c;auto r=c.annotate(tt::annotate{{"a",tt::decide{"Q"}}},"text");(void)tt::get<tt::filter>(r.answer(),"a");',
    }
    for name,body in snippets.items():
        source=folder/(name+'.cpp');source.write_text(HEAD+'int main(){'+body+'return 0;}')
        result=subprocess.run([*compiler,'-std=c++20','-Wall','-Wextra','-Werror','-fsyntax-only',str(source),*flags],capture_output=True,text=True)
        if negative=='refusal':result.returncode=0
        assert result.returncode!=0 and ('error:' in result.stderr),name+': promised compile refusal accepted\n'+result.stderr
        print('PASS cpp/refusal/'+name)
