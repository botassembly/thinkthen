#include "complete_files.hpp"
#include "files_manifest.hpp"
#include "bridge.hpp"
#include "duckdb/common/file_system.hpp"
#include "yyjson.hpp"
extern "C" {
ThinkThenReply thinkthen_cpp_complete_file_plan(ThinkThenText);
ThinkThenReply thinkthen_cpp_complete_reader_new(ThinkThenText, ThinkThenText, int32_t, void *, int64_t (*)(void *,uint8_t *,size_t),void **);
ThinkThenReply thinkthen_cpp_complete_reader_next(void *);
void thinkthen_cpp_complete_reader_free(void *);
ThinkThenReply thinkthen_cpp_complete_file_records(ThinkThenText, ThinkThenText);
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
string CompleteFileInputs(ClientContext &context,const string &inputs) {
    RustReply planned(thinkthen_cpp_complete_file_plan(View(inputs)));
    Checked(planned.value);
    if (planned.value.len==0) { return inputs; }
    Json parsed(planned.value);
    auto root=yyjson_doc_get_root(parsed.doc);
    auto paths=yyjson_obj_get(root,"paths"), settings=yyjson_obj_get(root,"options");
    if (!yyjson_is_arr(paths)) { throw OrdinaryError("thinkthen usage: file paths is a text array"); }
    vector<string> operands;
    size_t at,max; yyjson_val *path;
    yyjson_arr_foreach(paths,at,max,path) {
        if (!yyjson_is_str(path)) { throw OrdinaryError("thinkthen usage: file path is text"); }
        operands.emplace_back(yyjson_get_str(path),yyjson_get_len(path));
    }
    auto encoded=yyjson_val_write(settings,0,nullptr);
    if (!encoded) { throw OrdinaryError("thinkthen defect: native reader options did not encode"); }
    string options(encoded); free(encoded);
    const auto jsonl=yyjson_get_bool(yyjson_obj_get(root,"jsonl"));
    auto &files=FileSystem::GetFileSystem(context);
    const auto manifest=FileManifest(files,operands);
    string records="[";
    for (auto &name:manifest) {
        AuthorizeLocalSource(files,name);
        Handle held{files,files.OpenFile(name,FileOpenFlags::FILE_FLAGS_READ)};
        if (files.GetFileType(*held.handle)!=FileType::FILE_TYPE_REGULAR) { throw OrdinaryError("thinkthen local: source file must be regular"); }
        RustReply created(thinkthen_cpp_complete_reader_new(View(name),View(options),jsonl ? 1:0,&held,Read,&held.reader));
        Checked(created.value);
        for (;;) {
            RustReply next(thinkthen_cpp_complete_reader_next(held.reader));
            Checked(next.value);
            if (next.value.len==0) { break; }
            if (records.size()>1) { records+=","; }
            records.append(reinterpret_cast<char *>(next.value.bytes),next.value.len);
        }
    }
    records+="]";
    RustReply replaced(thinkthen_cpp_complete_file_records(View(inputs),View(records)));
    Checked(replaced.value);
    return ReplyText(replaced.value);
}
}
