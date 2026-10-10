#include "complete_files.hpp"
#include "files_manifest.hpp"
#include "bridge.hpp"
#include "duckdb/common/file_system.hpp"
#include "yyjson.hpp"
#include "duckdb/common/error_data.hpp"
#include "scalar_owner.hpp"
#include <thread>
#include <chrono>
extern "C" {
ThinkThenReply thinkthen_cpp_complete_file_plan(ThinkThenText);
ThinkThenReply thinkthen_cpp_complete_admission_error(ThinkThenText,ThinkThenText);
ThinkThenReply thinkthen_cpp_complete_reader_new(ThinkThenText, ThinkThenText, int32_t, void *, int64_t (*)(void *,uint8_t *,size_t),void **);
ThinkThenReply thinkthen_cpp_complete_reader_next(void *);
void thinkthen_cpp_complete_reader_free(void *);
ThinkThenReply thinkthen_cpp_complete_feed_new(ThinkThenText,ThinkThenText,ThinkThenText,ThinkThenText,const void *,int64_t,ThinkThenSettings,void **);
ThinkThenReply thinkthen_cpp_complete_feed_push(void *,ThinkThenText);
ThinkThenReply thinkthen_cpp_complete_feed_read(void *);
ThinkThenReply thinkthen_cpp_complete_feed_finish(void *,void *,int32_t);
void thinkthen_cpp_complete_feed_free(void *);
ThinkThenReply thinkthen_cpp_complete_feed_render(ThinkThenText,ThinkThenText);
ThinkThenReply thinkthen_cpp_complete_failure_envelope(ThinkThenText);
}
namespace duckdb {
namespace {
using namespace duckdb_yyjson;
ThinkThenText View(const string &text) { return {reinterpret_cast<const uint8_t *>(text.data()),text.size()}; }
struct Json {
    yyjson_doc *doc;
    explicit Json(const ThinkThenReply &reply): doc(yyjson_read(reinterpret_cast<char *>(reply.bytes),reply.len,0)) {
        if (!doc) { throw OrdinaryError("thinkthen defect: invalid native source plan"); }
    }
    ~Json() { yyjson_doc_free(doc); }
};
struct Handle {
    FileSystem &files;
    unique_ptr<FileHandle> handle;
    void *reader=nullptr;
    ~Handle() { if (reader) { thinkthen_cpp_complete_reader_free(reader); } }
};
int64_t Read(void *opaque,uint8_t *bytes,size_t count) noexcept {
    try { auto &held=*static_cast<Handle *>(opaque); return held.files.Read(*held.handle,bytes,count); }
    catch (...) { return -1; }
}
}
string CompleteAdmissionError(const std::exception &error) {
    auto message=ErrorData(error).RawMessage();
    string kind="local";
    for (auto name:{"usage","local","backend","deadline","cancelled","defect"}) {
        const auto prefix=string("thinkthen ")+name+": ";
        if (message.rfind(prefix,0)==0) { kind=name;message=message.substr(prefix.size());break; }
    }
    const string suffix=" (retryable: no)";
    const auto end=message.find('\n'), at=message.rfind(suffix,end);
    if (at!=string::npos && at+suffix.size()==(end==string::npos ? message.size():end)) { message.erase(at,suffix.size()); }
    RustReply reply(thinkthen_cpp_complete_admission_error(View(message),View(kind)));
    Checked(reply.value);
    return ReplyText(reply.value);
}
bool CompleteFileCall(ClientContext &context,const string &verb,const string &question,const string &inputs,const string &controls,const void *reference,int64_t deadline,ThinkThenSettings settings,string &result) {
    RustReply planned(thinkthen_cpp_complete_file_plan(View(inputs)));
    if (planned.value.status!=0) {
        RustReply failed(thinkthen_cpp_complete_failure_envelope(View(ReplyText(planned.value))));
        Checked(failed.value); result=ReplyText(failed.value); return true;
    }
    if (planned.value.len==0) { return false; }
    Json parsed(planned.value);
    auto root=yyjson_doc_get_root(parsed.doc);
    auto paths=yyjson_obj_get(root,"paths"), reader_options=yyjson_obj_get(root,"options");
    vector<string> operands;
    size_t at,max; yyjson_val *path;
    yyjson_arr_foreach(paths,at,max,path) { operands.emplace_back(yyjson_get_str(path),yyjson_get_len(path)); }
    auto encoded=yyjson_val_write(reader_options,0,nullptr);
    if (!encoded) { throw OrdinaryError("thinkthen defect: native reader options did not encode"); }
    string options(encoded); free(encoded);
    const auto framing=yyjson_get_int(yyjson_obj_get(root,"framing"));
    struct Feed {
        void *value=nullptr;
        ~Feed() { thinkthen_cpp_complete_feed_free(value); }
    } feed;
    RustReply started(thinkthen_cpp_complete_feed_new(View(verb),View(question),View(inputs),View(controls),reference,deadline,settings,&feed.value));
    Checked(started.value);
    if (!feed.value) { result=ReplyText(started.value); return true; }
    string packets="[";
    bool ended=false,closed=false,finished=false;
    auto check_stop=[&]() { if (QueryInterrupted(context)) { throw InvalidInputException("%s",BridgeErrorText("thinkthen cancelled: the call was cancelled").c_str()); } };
    auto drain=[&]() {
        check_stop();
        for (;;) {
            RustReply next(thinkthen_cpp_complete_feed_read(feed.value)); Checked(next.value);
            if (next.value.len==0) { break; }
            auto packet=ReplyText(next.value);
            if (packet=="E") { ended=true;closed=true;break; }
            if (packets.size()>1) { packets+=","; }
            packets+=packet;
            check_stop();
        }
    };
    auto finish=[&](void *reader,int32_t failed) {
        if (!finished) {
            RustReply reply(thinkthen_cpp_complete_feed_finish(feed.value,reader,failed)); Checked(reply.value); finished=true;
        }
    };
    auto pause=[&]() { check_stop();std::this_thread::sleep_for(std::chrono::milliseconds(5));check_stop(); };
    auto &files=FileSystem::GetFileSystem(context);
    try {
        check_stop();
        const auto manifest=FileManifest(files,operands);
        for (auto &name:manifest) {
            drain(); if (closed) { break; } check_stop();
            AuthorizeLocalSource(files,name);
            Handle held{files,files.OpenFile(name,FileOpenFlags::FILE_FLAGS_READ)};
            if (files.GetFileType(*held.handle)!=FileType::FILE_TYPE_REGULAR) { throw OrdinaryError("thinkthen local: source file must be regular"); }
            check_stop();
            RustReply created(thinkthen_cpp_complete_reader_new(View(name),View(options),static_cast<int32_t>(framing),&held,Read,&held.reader));
            if (created.value.status!=0) { finish(held.reader,1);closed=true;break; }
            for (;;) {
                drain(); if (closed) { break; } check_stop();
                RustReply next(thinkthen_cpp_complete_reader_next(held.reader));
                if (next.value.status!=0) { finish(held.reader,0);closed=true;break; }
                if (next.value.len==0) { break; }
                const auto descriptor=ReplyText(next.value);
                for (;;) {
                    check_stop();
                    RustReply pushed(thinkthen_cpp_complete_feed_push(feed.value,View(descriptor))); Checked(pushed.value);
                    const auto status=ReplyText(pushed.value);
                    drain();
                    if (status=="C") { closed=true;break; }
                    if (status=="A" || closed) { break; }
                    pause();
                }
                if (closed) { break; }
            }
            if (closed) { break; }
        }
        if (!closed) { finish(nullptr,0); }
    } catch (const Exception &error) {
        if (ErrorData(error).Type()==ExceptionType::INTERRUPT || QueryInterrupted(context)) { throw; }
        finish(nullptr,1);
    }
    while (!ended) { drain(); if (!ended) { pause(); } }
    packets+="]";
    RustReply rendered(thinkthen_cpp_complete_feed_render(View(packets),View(verb))); Checked(rendered.value);
    result=ReplyText(rendered.value); return true;
}
}
