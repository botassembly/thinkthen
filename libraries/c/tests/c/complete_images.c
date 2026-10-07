/* Original ordered image inputs execute in native routes; other verbs send nothing. */
#include <thinkthen.h>
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define STR(s) (thinkthen_string_v1){s,sizeof(s)-1}
#define TEXT(s) (thinkthen_content_v1){1,STR(s)}
static thinkthen_image *image(thinkthen_engine *e,const char *path) {
    FILE *f=fopen(path,"rb"); assert(f); uint8_t bytes[1024]; size_t n=fread(bytes,1,sizeof bytes,f); assert(fclose(f)==0 && n>0 && n<sizeof bytes);
    thinkthen_image *out=NULL; assert(thinkthen_image_clone(e,bytes,n,2,(thinkthen_optional_string_v1){0},&out)==0); return out;
}
static thinkthen_question *question(thinkthen_engine *e,unsigned kind) {
    thinkthen_question_spec_v1 spec={0}; spec.kind=kind;
    if(kind<8) spec.text=kind==1 ? TEXT("Is red visible?"):kind==2 ? TEXT("Which color?"):TEXT("How red?");
    thinkthen_choice_v1 options[2]={0}; options[0].name=kind==4?STR("none"):STR("red"); options[1].name=kind==4?STR("all"):STR("blue");
    if(kind==2 || kind==3 || kind==4) spec.choices=(thinkthen_choices_v1){options,2};
    thinkthen_question *member=NULL; thinkthen_member_spec_v1 m={0};
    if(kind==8) { member=question(e,1); m.name=STR("visible"); m.question=member; spec.members=(thinkthen_member_specs_v1){&m,1}; }
    thinkthen_relation_v1 relation={0};
    if(kind==10) { relation.name=STR("supports"); relation.source=STR("*"); relation.target=STR("*"); spec.relations=(thinkthen_relations_v1){&relation,1}; }
    thinkthen_question *q=NULL; assert(thinkthen_question_new(e,&spec,&q)==0); thinkthen_question_free(member); return q;
}
typedef int (*complete_fn)(const thinkthen_engine *,const thinkthen_question *,const thinkthen_source *,const thinkthen_controls_v1 *,thinkthen_result **);
int main(void) {
    const char *red_path=getenv("TYPED_IMAGE"),*blue_path=getenv("TYPED_BLUE"); assert(red_path && blue_path);
    const char *base=getenv("THINKTHEN_BASE_URL"); assert(base && strncmp(base,"http://127.0.0.1:",17)==0);
    char settings[2048]; assert(snprintf(settings,sizeof settings,"{\"cache\":false,\"backend\":\"liquid\",\"base_url\":\"%s\",\"model\":\"d1\",\"max_retries\":0}",base)>0);
    thinkthen_engine *e=thinkthen_engine_new_with(settings); assert(e);
    thinkthen_image *red=image(e,red_path),*blue=image(e,blue_path); const thinkthen_image *images[]={red,blue,red};
    thinkthen_record_v1 record={0}; record.original=(thinkthen_optional_content_v1){1,TEXT("Compare originals.")}; record.images=(thinkthen_images_v1){images,3};
    thinkthen_source *source=NULL; assert(thinkthen_source_records(e,&record,1,&source)==0);
    thinkthen_image_free(red); thinkthen_image_free(blue);
    const complete_fn functions[]={thinkthen_decide_complete,thinkthen_choose_complete,thinkthen_tag_complete,thinkthen_score_complete,thinkthen_filter_complete,thinkthen_rank_complete,thinkthen_find_complete,thinkthen_annotate_complete,thinkthen_recognize_complete,thinkthen_relate_complete};
    const char *refuse=getenv("TYPED_REFUSE");
    for(unsigned kind=1;kind<=10;++kind) {
        int admitted=kind==1 || kind==2 || kind==4;
        if((refuse!=NULL)==admitted) continue;
        thinkthen_question *q=question(e,kind); thinkthen_result *r=NULL;
        int rc=functions[kind-1](e,q,source,NULL,&r);
        if(!admitted) { assert(rc==THINKTHEN_EUSAGE && r==NULL); }
        else {
            if(rc) { fprintf(stderr,"image %u: %s\n",kind,thinkthen_error_message(e)); abort(); }
            thinkthen_summary_v1 summary={0}; assert(thinkthen_result_summary(r,&summary)==0);
            assert(summary.facts.value.requests_sent==1);
            thinkthen_row_v1 common={0};
            if(kind==1) { thinkthen_decide_view_v1 v={0}; assert(thinkthen_result_decide(r,0,&v)==0); assert(v.value.kind==1 && v.value.data.boolean==1); common=v.common; }
            if(kind==2) { thinkthen_choose_view_v1 v={0}; assert(thinkthen_result_choose(r,0,&v)==0); assert(v.value.present && v.value.value.len==3 && memcmp(v.value.value.data,"red",3)==0); common=v.common; }
            if(kind==4) { thinkthen_score_view_v1 v={0}; assert(thinkthen_result_score(r,0,&v)==0); assert(v.value==0.8); common=v.common; }
            assert(common.images.present && common.images.value.len==3);
            const thinkthen_image_view_v1 *v=common.images.value.data;
            assert(v[0].bytes_len==v[2].bytes_len && memcmp(v[0].bytes,v[2].bytes,v[0].bytes_len)==0);
            assert(v[0].bytes_len!=v[1].bytes_len || memcmp(v[0].bytes,v[1].bytes,v[0].bytes_len)!=0);
            thinkthen_details_v1 d={0}; assert(thinkthen_result_details(r,0,&d)==0 && d.inputs.data[0].images.value.len==3);
        }
        thinkthen_result_free(r); thinkthen_question_free(q);
    }
    if(!refuse) {
        thinkthen_string_v1 path={red_path,strlen(red_path)}; thinkthen_source_spec_v1 files={{&path,1},THINKTHEN_SOURCE_IMAGE_FILE_V1,0};
        thinkthen_source *file=NULL; assert(thinkthen_source_files(e,&files,&file)==0); thinkthen_question *q=question(e,1); thinkthen_result *r=NULL;
        assert(thinkthen_decide_complete(e,q,file,NULL,&r)==0);
        thinkthen_decide_view_v1 v={0}; assert(thinkthen_result_decide(r,0,&v)==0);
        assert(!v.common.input.present && v.common.images.present && v.common.images.value.len==1);
        assert(v.common.position.present && v.common.position.value.file.present && !v.common.position.value.first_line.present && !v.common.position.value.last_line.present);
        thinkthen_details_v1 d={0}; assert(thinkthen_result_details(r,0,&d)==0 && d.inputs.data[0].position.present);
        thinkthen_result_free(r); thinkthen_question_free(q); thinkthen_source_free(file);
    }
    thinkthen_source_free(source); thinkthen_engine_free(e); return 0;
}
