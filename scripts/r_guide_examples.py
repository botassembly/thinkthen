"""Run the R ideal guide literally against private named backend scenarios."""
from pathlib import Path
import json
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'scripts'))
from guide_examples import blocks, check, scenario

MESSAGES = ['Please refund my order.', 'Where is my parcel?', 'I was charged twice for one invoice.']
SETUP = '''library(thinkthen)
topic_choices <- c("billing", "shipping", "other")
urgency_levels <- c("routine", "urgent")
messages <- c("Please refund my order.", "Where is my parcel?", "I was charged twice for one invoice.")
frame <- data.frame(id=c("a1", "a2", "a3"), text=messages)
write.csv(frame, "messages.csv", row.names=FALSE)
tt_toolbox_write(tt_toolbox(list(topic=tt_choose_question("What is the message about?", choices=topic_choices))), "toolbox.json")
'''


def expected(index, name):
    failed = name == 'failed'
    unsure = name == 'unsure'
    no = name == 'no'
    route = 'inbox' if no else 'refund queue'
    if index == 0:
        return None if failed else route + '\n'
    if index in (1, 2):
        if index == 2 and failed or unsure:
            return 'a person reads it\n'
        return None if failed else route + ' \n'
    if index == 3:
        return ('could not judge it: failed: backend.status: the backend answered status 503: {"error":"unavailable"} \n'
                if failed else '[1] FALSE\n' if no else '[1] TRUE\n')
    if index == 4:
        return '[1] ' + ('NA' if failed else '0.5' if unsure else '0.05' if no else '0.95') + '\n'
    if index == 5:
        return None if failed else ('a person reads it\n' if unsure else 'warehouse \n' if no else 'billing team \n')
    if index in (6, 7):
        destination = 'try again later' if failed else 'a person reads it' if unsure else route
        return '\n'.join(destination + ': ' + text for text in MESSAGES) + '\n'
    if index in (8, 9):
        if failed:
            if index == 8:
                return None
            return 'refund queue: \n' + '\n'.join('try again later: ' + text + ' backend.status' for text in MESSAGES) + '\n'
        return '' if no else ''.join('refund queue: ' + text + ' \n' for text in MESSAGES)
    if index == 10:
        return None if failed else '' if no else 'billing team\nbilling \nshipping \n'
    if index == 11:
        return None if failed else '[1] 0\n' if name == 'low-score' else ('[1] 0.5\n' if unsure else 'answer today\n[1] 1\n')
    if index == 12:
        return '\n'.join(('NA' if failed else str(i)) + ' ' + text for i, text in enumerate(MESSAGES, 1)) + '\n'
    if index == 13:
        return None if failed else 'a person reads it\n' if unsure else 'no line answers\n' if no else 'Please refund my order. \n'
    if index in (14, 15):
        if failed:
            return None
        if index == 14 and no:
            return ''.join('person ' + word + ' \n' for word in ['Morgan', 'works', 'for', 'Example', 'Group'])
        return ('person Morgan \norganization Example Group \n' if index == 14 else 'Morgan works_for Example Group \n') if name == 'relations' else ''
    if index == 16:
        destination = ('try again later' if failed or name.endswith('-failed') else
                       'a person reads it' if unsure or name.endswith('-unsure') else None)
        if destination:
            return ''.join(destination + ': ' + text + ' \n' for text in MESSAGES)
        return '' if no else ('refund queue: billing 0.00\n' if name == 'low-score' else 'refund queue: billing 1.00\n') * 3
    if index == 23:
        return None if failed else '1 0 0 \n'
    if failed and index in (17, 18, 20, 21, 22):
        return None
    return ''


def run_r(command=None, negative=None):
    fences = blocks(ROOT / 'docs/surfaces/r.md')
    assert len(fences) == 24 and all(language == 'r' for language, _, _ in fences)
    names = ['yes', 'no', 'unsure', 'failed', 'relations', 'low-score',
             'refund-failed', 'topic-failed', 'urgency-failed', 'refund-unsure', 'topic-unsure']
    with scenario('yes') as (folder, env):
        env['R_GUIDE_NEGATIVE'] = negative or ''
        script = folder / 'refusals.R'
        script.write_text(SETUP + '\n' + (ROOT / 'sdk/r/tests/ideal-contract.R').read_text())
        result = subprocess.run(['Rscript', '--vanilla', str(script)], cwd=folder, env=env,
                                capture_output=True, text=True, timeout=120)
        check('r/ideal/public-refusals-and-storage', result, output='R ideal refusals, native vectors, packed records and canonical saving passed\n')


    # The named no scenario selects shipping; use an existing reply fixture for
    # the third native factor level and the literal switch's front-desk branch.
    other = {'replies': [{'distribution': {'billing': 0., 'shipping': 0., 'other': 1.}}]}
    with scenario('yes', case=other) as (folder, env):
        script = folder / 'other.R'
        script.write_text(SETUP + '\n' + fences[5][1] + '''
failure <- tryCatch({team_for(factor("future",levels=c(topic_choices,"future")));NULL},error=identity)
stopifnot(inherits(failure,"error"),grepl("no team for:",conditionMessage(failure),fixed=TRUE))
''')
        result = subprocess.run(['Rscript', '--vanilla', str(script)], cwd=folder, env=env,
                                capture_output=True, text=True, timeout=60)
        check('r/ideal/choose/other-and-unknown-choice-refusal', result,
              output='front desk \n', branch='front desk ')

    authentication = {'replies': [{'status': 401, 'body': {'error': 'unauthorized'}}]}
    with scenario('yes', case=authentication) as (folder, env):
        script = folder / 'authentication.R'
        script.write_text(SETUP + '\n' + fences[3][1])
        result = subprocess.run(['Rscript', '--vanilla', str(script)], cwd=folder, env=env,
                                capture_output=True, text=True, timeout=60)
        check('r/ideal/did-call-work/general-call-error', result,
              output='the call did not complete: the backend answered status 401: {"error":"unauthorized"} \n',
              branch='the call did not complete: the backend answered status 401: {"error":"unauthorized"} ')

    mismatches = []
    for name in names:
        # The targeted scenarios cover every independent form entry; other examples
        # have their complete primitive outcomes covered by the first six scenarios.
        selected = range(len(fences)) if name in names[:6] else [16]
        with scenario(name) as (folder, env):
            env['R_GUIDE_SCENARIO'] = name
            for index in selected:
                (folder / f'{index}.R').write_text(fences[index][1])
            harness = SETUP + '\n'
            harness += 'results <- list()\n'
            for index in selected:
                harness += f'''failure <- NULL
out <- capture.output(tryCatch(eval(parse(file="{index}.R")), error=function(e) {{ failure <<- e; invisible(NULL) }}))
results[["{index}"]] <- list(output=paste0(paste(out, collapse="\\n"), if(length(out)) "\\n" else ""),
 error=if(is.null(failure)) NULL else class(failure)[[1]], message=if(is.null(failure)) NULL else conditionMessage(failure))
'''
                if index == 7:
                    harness += '''stopifnot(identical(where_it_goes(is_refund), switch(as.character(is_refund$status), answered=if(is_refund$answer) "refund queue" else "inbox", unsure="a person reads it", failed="try again later")))
future <- is_refund
future$status <- factor("future")
failure <- tryCatch({where_it_goes(future);NULL},error=identity)
stopifnot(inherits(failure,"error"),grepl("no route for:",conditionMessage(failure),fixed=TRUE))
'''
                if index == 13 and name in ('no', 'unsure'):
                    harness += '''saved_find <- tt_find(message, question="Which line states the requested action?", details=TRUE)
path <- tempfile()
tt_details_write(saved_find,path)
wire <- jsonlite::fromJSON(readLines(path,encoding="UTF-8"),simplifyVector=FALSE)
if(Sys.getenv("R_GUIDE_SCENARIO")=="no") {
 stopifnot(wire$status=="answered",is.null(wire$answer),!is.na(saved_find$answer),saved_find$answer$text=="")
} else {
 stopifnot(wire$status=="unsure",is.null(wire$answer),is.na(saved_find$answer))
}
'''
                if index == 15 and name == 'relations':
                    harness += '''saved_relations <- tt_relate(message,rules=rules,details=TRUE)
path <- tempfile()
tt_details_write(saved_relations,path)
wire <- jsonlite::fromJSON(readLines(path,encoding="UTF-8"),simplifyVector=FALSE)
stopifnot(length(wire$answer)==1L,wire$answer[[1]]$source$start==0,wire$answer[[1]]$source$text=="Morgan",length(wire$detail$names)==2L)
'''
                if index == 16:
                    harness += (ROOT / 'sdk/r/tests/ideal-states.R').read_text() + '\n'
            harness += 'writeLines(jsonlite::toJSON(results, auto_unbox=TRUE, null="null"), "results.json")\n'
            script = folder / 'guide.R'
            script.write_text(harness)
            result = subprocess.run(['Rscript', '--vanilla', str(script)], cwd=folder, env=env,
                                    capture_output=True, text=True, timeout=180)
            check('r/literal-execution/' + name, result, output='', negative=negative if name == 'yes' and negative == 'exit' else None)
            actual = json.loads((folder / 'results.json').read_text())
            for index in selected:
                item = actual[str(index)]
                wanted = expected(index, name)
                label = f'r/ideal/line-{fences[index][2]}/{name}'
                try:
                    if wanted is None:
                        assert item['error'] == 'thinkthen_failed', (label, item)
                        assert item['message'].startswith('failed: '), (label, item)
                    else:
                        assert item['error'] is None, (label, item)
                        if negative in ('output', 'branch') and name == 'yes' and index == 0:
                            wanted = 'deliberately wrong output\n'
                        assert item['output'] == wanted, (label, repr(wanted), item)
                except AssertionError as error:
                    mismatches.append(str(error))
                else:
                    print('PASS ' + label, flush=True)
        if mismatches and negative:
            raise AssertionError('\n'.join(mismatches))
    if mismatches:
        raise AssertionError('\n'.join(mismatches))
