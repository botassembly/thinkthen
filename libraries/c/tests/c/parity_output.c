/* Shared fixture consumers print only values read from typed public C views. */
#include <thinkthen.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
static void quoted(thinkthen_string_v1 s) {
    putchar('"');
    for(size_t i=0;i<s.len;++i) { unsigned char c=(unsigned char)s.data[i];
        if(c=='"' || c=='\\') { putchar('\\'); putchar(c); }
        else if(c=='\n') fputs("\\n",stdout); else if(c=='\r') fputs("\\r",stdout); else if(c=='\t') fputs("\\t",stdout); else if(c<32) printf("\\u%04x",c); else putchar(c);
    }
    putchar('"');
}
static void content(thinkthen_content_v1 c) { if(c.kind==2) fwrite(c.data.data,1,c.data.len,stdout); else quoted(c.data); }
static void strings(thinkthen_strings_v1 s) { putchar('['); for(size_t i=0;i<s.len;++i) { if(i) putchar(','); quoted(s.data[i]); } putchar(']'); }
static void decision(thinkthen_decide_value_v1 v) { if(v.kind==0) fputs("null",stdout); else if(v.kind==1) fputs(v.data.boolean?"true":"false",stdout); else content(v.data.authored); }
static void member(thinkthen_member_value_v1 v) {
    if(v.kind==1) decision(v.data.decide);
    else if(v.kind==2) { if(v.data.choose.present) quoted(v.data.choose.value); else fputs("null",stdout); }
    else if(v.kind==3) strings(v.data.tag); else printf("%.17g",v.data.score);
}
static void probabilities(thinkthen_probabilities_v1 v) { putchar('{'); for(size_t i=0;i<v.len;++i) { if(i) putchar(','); quoted(v.data[i].name); printf(":%.17g",v.data[i].probability); } putchar('}'); }
static void endpoint(thinkthen_endpoint_v1 v) { fputs("{\"name\":",stdout); quoted(v.name); fputs(",\"kind\":",stdout); quoted(v.kind); putchar('}'); }
static void entity(thinkthen_entity_v1 v) { fputs("{\"text\":",stdout); quoted(v.text); printf(",\"start\":%zu,\"end\":%zu,\"length\":%zu,\"strength\":%.17g,\"kind\":",v.start,v.end,v.length,v.strength); quoted(v.kind); putchar('}'); }
static void edges(thinkthen_edges_v1 v) { putchar('['); for(size_t i=0;i<v.len;++i) { if(i) putchar(','); fputs("{\"relation\":",stdout); quoted(v.data[i].relation); fputs(",\"source\":",stdout); endpoint(v.data[i].source); fputs(",\"target\":",stdout); endpoint(v.data[i].target); printf(",\"probability\":%.17g",v.data[i].probability); if(v.data[i].either) fputs(",\"either\":true",stdout); putchar('}'); } putchar(']'); }
static void recognized(thinkthen_recognize_value_v1 v) {
    fputs("{\"entities\":[",stdout); for(size_t i=0;i<v.entities.len;++i) { if(i) putchar(','); entity(v.entities.data[i]); } putchar(']');
    if(v.relations.present) { fputs(",\"relations\":[",stdout); for(size_t i=0;i<v.relations.value.len;++i) { if(i) putchar(','); thinkthen_entity_edge_v1 e=v.relations.value.data[i]; fputs("{\"relation\":",stdout); quoted(e.relation); fputs(",\"source\":",stdout); entity(e.source); fputs(",\"target\":",stdout); entity(e.target); printf(",\"probability\":%.17g",e.probability); if(e.either) fputs(",\"either\":true",stdout); putchar('}'); } putchar(']'); }
    putchar('}');
}
static size_t ordinal(thinkthen_result *r,thinkthen_row_v1 row) {
    thinkthen_summary_v1 s={0}; if(thinkthen_result_summary(r,&s)) abort();
    for(size_t i=0;i<s.observation_count;++i) { thinkthen_observation_v1 e={0}; if(thinkthen_result_observation(r,i,&e)) abort(); if(e.kind==2) { thinkthen_row_v1 v={0}; switch(e.data.row.function) { case 1:v=e.data.row.data.decide.common;break;case 2:v=e.data.row.data.choose.common;break;case 3:v=e.data.row.data.tag.common;break;case 4:v=e.data.row.data.score.common;break;case 5:v=e.data.row.data.filter.common;break;case 6:v=e.data.row.data.rank.common;break;case 8:v=e.data.row.data.annotate.common;break;case 9:v=e.data.row.data.recognize.common;break;default:break; } if(v.answer_id.len==row.answer_id.len && !memcmp(v.answer_id.data,row.answer_id.data,v.answer_id.len)) return e.data.row.index; } }
    return 0;
}
static void details(thinkthen_result *r,size_t at) {
    thinkthen_details_v1 d={0}; if(thinkthen_result_details(r,at,&d)) abort();
    fputs(",\"detail_inputs\":[",stdout);
    for(size_t i=0;i<d.inputs.len;++i) {
        if(i) putchar(',');
        thinkthen_input_view_v1 input=d.inputs.data[i];
        fputs("{\"input\":",stdout); if(input.original.present) content(input.original.value); else fputs("null",stdout);
        if(input.position.present) { fputs(",\"file\":",stdout); quoted(input.position.value.file.value); if(input.position.value.first_line.present) printf(",\"first_line\":%zu",input.position.value.first_line.value); if(input.position.value.last_line.present) printf(",\"last_line\":%zu",input.position.value.last_line.value); }
        putchar('}');
    }
    putchar(']');
}
static void row(thinkthen_result *r,unsigned kind,size_t at) {
    thinkthen_row_v1 common={0}; size_t index=0;
    fputs("{\"value\":",stdout);
    switch(kind) {
        case 1:{ thinkthen_decide_view_v1 v={0}; if(thinkthen_result_decide(r,at,&v)) abort(); common=v.common; decision(v.value); break; }
        case 2:{ thinkthen_choose_view_v1 v={0}; if(thinkthen_result_choose(r,at,&v)) abort(); common=v.common; if(v.value.present) quoted(v.value.value); else fputs("null",stdout); break; }
        case 3:{ thinkthen_tag_view_v1 v={0}; if(thinkthen_result_tag(r,at,&v)) abort(); common=v.common; strings(v.value); break; }
        case 4:{ thinkthen_score_view_v1 v={0}; if(thinkthen_result_score(r,at,&v)) abort(); common=v.common; printf("%.17g",v.value); break; }
        case 5:{ thinkthen_filter_view_v1 v={0}; if(thinkthen_result_filter(r,at,&v)) abort(); common=v.common; fputs(v.value?"true":"false",stdout); break; }
        case 6:{ thinkthen_rank_view_v1 v={0}; if(thinkthen_result_rank(r,at,&v)) abort(); common=v.common; printf("%zu",v.value.value); break; }
        case 7:{ thinkthen_find_view_v1 v={0}; if(thinkthen_result_find(r,at,&v)) abort(); common=v.common; if(v.value.present) content(v.value.value); else fputs("null",stdout); index=v.index.present?v.index.value:SIZE_MAX; break; }
        case 8:{ thinkthen_annotate_view_v1 v={0}; if(thinkthen_result_annotate(r,at,&v)) abort(); common=v.common; putchar('{'); for(size_t i=0;i<v.answers.len;++i) { if(i) putchar(','); quoted(v.answers.data[i].name); putchar(':'); if(v.answers.data[i].state==1) member(v.answers.data[i].data.success.value); else { const char *causes[]={"","missing_answer","wrong_kind","missing_probability","invalid_probability","invalid_distribution","unexpected_probability"}; unsigned cause=v.answers.data[i].data.failure.cause; if(cause>6) abort(); printf("{\"failed\":{\"kind\":\"backend\",\"cause\":\"%s\"}}",causes[cause]); } } putchar('}'); break; }
        case 9:{ thinkthen_recognize_view_v1 v={0}; if(thinkthen_result_recognize(r,at,&v)) abort(); common=v.common; recognized(v.value); break; }
        case 10:{ thinkthen_relate_view_v1 v={0}; if(thinkthen_result_relate(r,at,&v)) abort(); common=v.common; edges(v.value); break; }
        default:abort();
    }
    if(kind!=7) index=ordinal(r,common);
    printf(",\"index\":"); if(index==SIZE_MAX) fputs("null",stdout); else printf("%zu",index);
    fputs(",\"answer_id\":",stdout); quoted(common.answer_id);
    fputs(",\"origin\":",stdout); if(common.meta.origin.present) printf("%u",common.meta.origin.value); else fputs("null",stdout);
    fputs(",\"answered_by\":",stdout); if(common.meta.answered_by.present) quoted(common.meta.answered_by.value); else fputs("null",stdout);
    printf(",\"observations\":%zu,\"sources\":%zu",common.meta.observations.len,common.meta.question_sources.len);
    fputs(",\"observation_ids\":[",stdout); for(size_t i=0;i<common.meta.observations.len;++i) { if(i) putchar(','); quoted(common.meta.observations.data[i].data.observation_id); } putchar(']');
    fputs(",\"input\":",stdout); if(common.input.present) content(common.input.value); else fputs("null",stdout);
    if(common.position.present) { fputs(",\"file\":",stdout); quoted(common.position.value.file.value); if(common.position.value.first_line.present) printf(",\"first_line\":%zu",common.position.value.first_line.value); if(common.position.value.last_line.present) printf(",\"last_line\":%zu",common.position.value.last_line.value); }
    if(common.images.present) { fputs(",\"images\":[",stdout); for(size_t i=0;i<common.images.value.len;++i) { if(i) putchar(','); putchar('"'); for(size_t j=0;j<common.images.value.data[i].bytes_len;++j) printf("%02x",common.images.value.data[i].bytes[j]); putchar('"'); } putchar(']'); }
    if(common.images.present) { fputs(",\"image_properties\":[",stdout); for(size_t i=0;i<common.images.value.len;++i) { if(i) putchar(','); thinkthen_image_view_v1 image=common.images.value.data[i]; printf("[%u,%u,%u]",image.media,image.width,image.height); } putchar(']'); }
    if(common.answer.present) { thinkthen_answer_v1 a=common.answer.value; if(a.kind==1) printf(",\"probability\":%.17g",a.data.probability); else { fputs(",\"probabilities\":",stdout); if(a.kind==2) probabilities(a.data.choice.probabilities); else if(a.kind==3) probabilities(a.data.tag); else if(a.kind==4) probabilities(a.data.score.probabilities); else probabilities(a.data.find.probabilities); } }
    if(kind==8) {
        thinkthen_annotate_view_v1 v={0}; if(thinkthen_result_annotate(r,at,&v)) abort();
        fputs(",\"member_authors\":[",stdout);
        for(size_t i=0;i<v.answers.len;++i) {
            if(i) putchar(',');
            thinkthen_question_author_v1 member_author={0}; if(thinkthen_result_member_author(r,at,i,&member_author)) abort();
            putchar('{'); if(member_author.name.present) { fputs("\"name\":",stdout); quoted(member_author.name.value); }
            if(member_author.wording_version.present) { if(member_author.name.present) putchar(','); printf("\"wording_version\":%llu",(unsigned long long)member_author.wording_version.value); }
            putchar('}');
        }
        putchar(']');
    }
    details(r,at);
    thinkthen_question_author_v1 author={0}; if(thinkthen_result_question_author(r,at,&author)) abort();
    if(author.name.present) { fputs(",\"name\":",stdout); quoted(author.name.value); }
    if(author.wording_version.present) printf(",\"wording_version\":%llu",(unsigned long long)author.wording_version.value);
    putchar('}');
}
static void output(thinkthen_engine *e,unsigned kind,int code,thinkthen_result *r) {
    printf("{\"code\":%d",code);
    if(code) {
        fputs(",\"message\":",stdout); const char *message=thinkthen_error_message(e); quoted((thinkthen_string_v1){message,strlen(message)});
        thinkthen_result *snapshot=NULL; if(thinkthen_error_complete(e,&snapshot)) abort();
        if(snapshot) {
            thinkthen_summary_v1 s={0}; if(thinkthen_result_summary(snapshot,&s)) abort();
            if(s.facts.present) printf(",\"requests_sent\":%llu,\"records\":%llu",(unsigned long long)s.facts.value.requests_sent,(unsigned long long)s.facts.value.records);
            if(s.error.value.stopped.present && s.error.value.stopped.value.at.present) printf(",\"stopped_at\":%zu",s.error.value.stopped.value.at.value);
            thinkthen_result_free(snapshot);
        }
    }
    else { thinkthen_summary_v1 s={0}; if(thinkthen_result_summary(r,&s)) abort(); fputs(",\"schema\":",stdout); quoted(s.schema); printf(",\"requests_sent\":%llu,\"cache_answers\":%llu,\"observations\":%zu,\"call_id\":",(unsigned long long)s.facts.value.requests_sent,(unsigned long long)s.facts.value.cache_answers,s.observation_count); quoted(s.facts.value.call_id); fputs(",\"rows\":[",stdout); for(size_t at=0;at<s.count;++at) { if(at) putchar(','); row(r,kind,at); } putchar(']'); }
    puts("}");
}
