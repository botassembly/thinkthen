/* Every admitted question constructor, descriptor bounds and explicit readers. */
#include <thinkthen.h>
#include <assert.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#define STR(s) (thinkthen_string_v1){s, sizeof(s)-1}
#define TEXT(s) (thinkthen_content_v1){1, STR(s)}
static thinkthen_question *make(thinkthen_engine *e,unsigned kind) {
    thinkthen_question_spec_v1 spec={0}; spec.kind=kind;
    if(kind<8) spec.text=TEXT("Does this need attention?");
    thinkthen_choice_v1 choices[2]={0}; choices[0].name=STR("z-first"); choices[1].name=STR("a-second");
    if(kind>=2 && kind<=4) spec.choices=(thinkthen_choices_v1){choices,2};
    thinkthen_member_spec_v1 members[2]={0};
    if(kind==8) {
        members[0].name=STR("z_first"); members[0].question=make(e,1);
        members[1].name=STR("a_second"); members[1].question=make(e,2);
        spec.members=(thinkthen_member_specs_v1){members,2};
    }
    thinkthen_relation_v1 rule={0};
    if(kind==10) { rule.name=STR("supports"); rule.source=STR("*"); rule.target=STR("*"); spec.relations=(thinkthen_relations_v1){&rule,1}; }
    thinkthen_question *q=NULL; assert(thinkthen_question_new(e,&spec,&q)==0 && q);
    if(kind==8) { thinkthen_question_free((thinkthen_question *)members[0].question); thinkthen_question_free((thinkthen_question *)members[1].question); }
    return q;
}
static void refusal(thinkthen_engine *e) {
    thinkthen_question_spec_v1 spec={0}; spec.kind=1; spec.text=TEXT("Q?");
    thinkthen_question *out=(thinkthen_question *)(uintptr_t)17;
    assert(thinkthen_question_new(e,NULL,&out)==THINKTHEN_EUSAGE);
    assert(thinkthen_question_new(NULL,&spec,&out)==THINKTHEN_EUSAGE);
    assert(thinkthen_question_new(e,&spec,NULL)==THINKTHEN_EUSAGE);
    spec.text.data=(thinkthen_string_v1){NULL,1}; assert(thinkthen_question_new(e,&spec,&out)==THINKTHEN_EUSAGE);
    spec.text.data=(thinkthen_string_v1){"x",SIZE_MAX}; assert(thinkthen_question_new(e,&spec,&out)==THINKTHEN_EUSAGE);
    const char invalid[]={(char)0xff}; spec.text.data=(thinkthen_string_v1){invalid,1}; assert(thinkthen_question_new(e,&spec,&out)==THINKTHEN_EUSAGE);
    assert(out==(thinkthen_question *)(uintptr_t)17);
    spec=(thinkthen_question_spec_v1){0}; spec.kind=9; spec.relation_threshold.kind=THINKTHEN_RULE_NULL_V1;
    assert(thinkthen_question_new(e,&spec,&out)==THINKTHEN_EUSAGE && out==(thinkthen_question *)(uintptr_t)17);
    thinkthen_source *source=(thinkthen_source *)(uintptr_t)23;
    assert(thinkthen_source_records(e,NULL,1,&source)==THINKTHEN_EUSAGE);
    assert(thinkthen_source_records(e,(const thinkthen_record_v1 *)"x",SIZE_MAX,&source)==THINKTHEN_EUSAGE);
    thinkthen_record_v1 record={0}; assert(thinkthen_source_records(e,&record,1,&source)==THINKTHEN_EUSAGE);
    record.original=(thinkthen_optional_content_v1){2,TEXT("text")}; assert(thinkthen_source_records(e,&record,1,&source)==THINKTHEN_EUSAGE);
    record.original=(thinkthen_optional_content_v1){1,TEXT("text")}; record.context.present=2;
    assert(thinkthen_source_records(e,&record,1,&source)==THINKTHEN_EUSAGE);
    record.context.present=0; record.images=(thinkthen_images_v1){NULL,1};
    assert(thinkthen_source_records(e,&record,1,&source)==THINKTHEN_EUSAGE);
    const thinkthen_image *null_image=NULL; record.images=(thinkthen_images_v1){&null_image,1};
    assert(thinkthen_source_records(e,&record,1,&source)==THINKTHEN_EUSAGE);
    assert(source==(thinkthen_source *)(uintptr_t)23);
    thinkthen_string_v1 path=STR("/path-that-does-not-exist");
    thinkthen_source_spec_v1 files={{&path,1},THINKTHEN_SOURCE_IMAGE_FILE_V1,1};
    assert(thinkthen_source_files(e,&files,&source)==THINKTHEN_EUSAGE && source==(thinkthen_source *)(uintptr_t)23);
    files.window=0; files.unit=99; assert(thinkthen_source_files(e,&files,&source)==THINKTHEN_EUSAGE);
    files.unit=THINKTHEN_SOURCE_IMAGE_FILE_V1; assert(thinkthen_source_files(e,&files,&source)==0); thinkthen_source_free(source);
    files.unit=THINKTHEN_SOURCE_LINE_V1; assert(thinkthen_source_files(e,&files,&source)==0); thinkthen_source_free(source);
}
int main(void) {
    thinkthen_engine *e=thinkthen_engine_new_with("{\"cache\":false}"); assert(e);
    for(unsigned kind=1;kind<=10;++kind) thinkthen_question_free(make(e,kind));
    const char *path=getenv("TYPED_QUESTION"); assert(path);
    thinkthen_question *loaded=NULL; assert(thinkthen_question_load(e,(thinkthen_string_v1){path,strlen(path)},&loaded)==0); thinkthen_question_free(loaded);
    refusal(e); thinkthen_engine_free(e); return 0;
}
