/* Counted descriptors clone buffers; immutable image views outlive engines. */
#include <thinkthen.h>
#include <assert.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#define STR(s) (thinkthen_string_v1){s, sizeof(s)-1}
#define TEXT(s) (thinkthen_content_v1){THINKTHEN_CONTENT_TEXT_V1, STR(s)}
static void image(thinkthen_engine *e) {
    const char *path=getenv("TYPED_IMAGE"); assert(path);
    FILE *f=fopen(path,"rb"); assert(f);
    uint8_t bytes[1024]; size_t n=fread(bytes,1,sizeof bytes,f); assert(fclose(f)==0);
    assert(n>0 && n<sizeof bytes);
    uint8_t original[1024]; memcpy(original,bytes,n);
    char filename[]="private-image-name.png";
    thinkthen_optional_string_v1 name={1,{filename,sizeof filename-1}};
    thinkthen_image *owner=NULL;
    assert(thinkthen_image_clone(e,bytes,n,THINKTHEN_IMAGE_PNG_V1,name,&owner)==0);
    memset(bytes,0,n); memset(filename,'x',sizeof filename-1);
    thinkthen_image_view_v1 view;
    assert(thinkthen_image_view(owner,&view)==0);
    assert(view.media==THINKTHEN_IMAGE_PNG_V1 && view.width==1 && view.height==1);
    assert(view.bytes_len==n && memcmp(view.bytes,original,n)==0);
    assert(view.filename.present && view.filename.value.len==22);
    assert(memcmp(view.filename.value.data,"private-image-name.png",22)==0);
    const char *jpeg_path=getenv("TYPED_JPEG"); assert(jpeg_path);
    f=fopen(jpeg_path,"rb"); assert(f);
    size_t jpeg_n=fread(bytes,1,sizeof bytes,f); assert(fclose(f)==0);
    assert(jpeg_n>0 && jpeg_n<sizeof bytes);
    uint8_t jpeg_original[1024]; memcpy(jpeg_original,bytes,jpeg_n);
    thinkthen_image *jpeg=NULL;
    assert(thinkthen_image_clone(e,bytes,jpeg_n,THINKTHEN_IMAGE_JPEG_V1,(thinkthen_optional_string_v1){0},&jpeg)==0);
    memset(bytes,0,jpeg_n);
    thinkthen_image_view_v1 jpeg_view;
    assert(thinkthen_image_view(jpeg,&jpeg_view)==0);
    assert(jpeg_view.media==THINKTHEN_IMAGE_JPEG_V1 && jpeg_view.width==32 && jpeg_view.height==32 && jpeg_view.filename.present==0);
    assert(jpeg_view.bytes_len==jpeg_n && memcmp(jpeg_view.bytes,jpeg_original,jpeg_n)==0);
    thinkthen_record_v1 record={0};
    const thinkthen_image *ordered[]={owner,owner}; record.images=(thinkthen_images_v1){ordered,2};
    char context[]="private context";
    record.context=(thinkthen_optional_content_v1){1,{1,{context,sizeof context-1}}};
    thinkthen_choice_v1 choices[2]={0}; choices[0].name=STR("duplicate"); choices[1].name=STR("duplicate");
    choices[0].description=(thinkthen_optional_content_v1){1,TEXT("first")};
    choices[1].description=(thinkthen_optional_content_v1){1,{THINKTHEN_CONTENT_JSON_V1,STR("{\"what\":\"second\"}")}};
    record.options=(thinkthen_choices_v1){choices,2};
    thinkthen_source *source=NULL; assert(thinkthen_source_records(e,&record,1,&source)==0);
    const thinkthen_image *many[9]={owner,owner,owner,owner,owner,owner,owner,owner,owner};
    record.images=(thinkthen_images_v1){many,9};
    thinkthen_source *unchanged_source=(thinkthen_source *)(uintptr_t)23;
    assert(thinkthen_source_records(e,&record,1,&unchanged_source)==THINKTHEN_EUSAGE);
    assert(unchanged_source==(thinkthen_source *)(uintptr_t)23);
    memset(context,'x',sizeof context-1);
    thinkthen_source_free(source);
    assert(thinkthen_source_records(e,NULL,0,&source)==0); thinkthen_source_free(source);
    thinkthen_image *unchanged=(thinkthen_image *)(uintptr_t)17;
    assert(thinkthen_image_clone(e,NULL,1,2,(thinkthen_optional_string_v1){0},&unchanged)==THINKTHEN_EUSAGE);
    assert(thinkthen_image_clone(e,original,SIZE_MAX,2,(thinkthen_optional_string_v1){0},&unchanged)==THINKTHEN_EUSAGE);
    assert(thinkthen_image_clone(e,original,n,79,(thinkthen_optional_string_v1){0},&unchanged)==THINKTHEN_EUSAGE);
    assert(thinkthen_image_clone(e,original,n,2,(thinkthen_optional_string_v1){2,{NULL,0}},&unchanged)==THINKTHEN_EUSAGE);
    assert(thinkthen_image_clone(e,original,n,1,(thinkthen_optional_string_v1){0},&unchanged)==THINKTHEN_EUSAGE);
    assert(thinkthen_image_clone(e,original,8,2,(thinkthen_optional_string_v1){0},&unchanged)==THINKTHEN_EUSAGE);
    assert(thinkthen_image_clone(e,original,n,2,(thinkthen_optional_string_v1){0},NULL)==THINKTHEN_EUSAGE);
    assert(unchanged==(thinkthen_image *)(uintptr_t)17);
    const char *failure=thinkthen_error_message(e);
    thinkthen_image_view_v1 sentinel={0}; sentinel.width=79;
    assert(thinkthen_image_view(NULL,&sentinel)==THINKTHEN_EUSAGE && sentinel.width==79);
    assert(thinkthen_image_view(owner,NULL)==THINKTHEN_EUSAGE);
    assert(failure==thinkthen_error_message(e) && thinkthen_error_code(e)==THINKTHEN_EUSAGE);
    thinkthen_engine_free(e);
    assert(thinkthen_image_view(owner,&view)==0 && memcmp(view.bytes,original,n)==0);
    assert(thinkthen_image_view(jpeg,&jpeg_view)==0 && memcmp(jpeg_view.bytes,jpeg_original,jpeg_n)==0);
    thinkthen_image_free(jpeg);
    thinkthen_image_free(owner); thinkthen_image_free(NULL); thinkthen_source_free(NULL); thinkthen_question_free(NULL);
}
int main(void) {
    thinkthen_engine *e=thinkthen_engine_new_with("{\"cache\":false}"); assert(e);
    image(e); return 0;
}
