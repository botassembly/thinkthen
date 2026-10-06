/* Located Unicode/CRLF text uses the native reader and never sends paths. */
#include <thinkthen.h>
#include <assert.h>
#include <stdlib.h>
#include <string.h>
#define STR(s) (thinkthen_string_v1){s, sizeof(s)-1}
int main(void) {
    const char *path=getenv("TYPED_FILE"); assert(path);
    thinkthen_engine *e=thinkthen_engine_new_with("{\"cache\":false}"); assert(e);
    thinkthen_question_spec_v1 spec={0}; spec.kind=1;
    spec.text=(thinkthen_content_v1){THINKTHEN_CONTENT_TEXT_V1,STR("Q?")};
    thinkthen_question *q=NULL; assert(thinkthen_question_new(e,&spec,&q)==0);
    thinkthen_string_v1 name={path,strlen(path)};
    thinkthen_source_spec_v1 selection={{&name,1},THINKTHEN_SOURCE_LINE_V1,0};
    thinkthen_source *source=NULL; assert(thinkthen_source_files(e,&selection,&source)==0);
    thinkthen_result *result=NULL; assert(thinkthen_decide_current(e,q,source,NULL,&result)==0);
    thinkthen_source_free(source); thinkthen_question_free(q); thinkthen_engine_free(e);
    thinkthen_current_summary_v1 summary; assert(thinkthen_result_current_summary(result,&summary)==0);
    assert(summary.count==2 && summary.facts.records==2 && summary.facts.requests_sent==1);
    const size_t lines[]={1,3},lengths[]={2,9};
    const char *texts[]={"\xc3\xa9","\xf0\x9f\x98\x80 tail"};
    for(size_t i=0;i<2;++i) {
        thinkthen_current_atomic_v1 row; assert(thinkthen_result_current_atomic(result,i,&row)==0);
        assert(row.common.position.present && row.common.position.value.first_line.value==lines[i] && row.common.position.value.last_line.value==lines[i]);
        assert(row.common.position.value.file.present && row.common.position.value.file.value.len==strlen(path));
        assert(row.common.input.value.data.len==lengths[i] && memcmp(row.common.input.value.data.data,texts[i],lengths[i])==0);
    }
    thinkthen_result_free(result); return 0;
}
