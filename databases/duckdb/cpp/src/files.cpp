#include "descriptions.hpp"
#include "files.hpp"
#include "files_manifest.hpp"
#include "bridge.hpp"
#include "duckdb/common/file_system.hpp"
#include "duckdb/function/table_function.hpp"
#include "duckdb/function/scalar_function.hpp"
#include "duckdb/common/types/vector.hpp"
#include "yyjson.hpp"

#include <limits>

extern "C" {
ThinkThenReply thinkthen_cpp_reader_options(const uint8_t *, size_t);
ThinkThenReply thinkthen_cpp_reader_new(const uint8_t *, size_t, const uint8_t *, size_t, void *,
                                      int64_t (*)(void *, uint8_t *, size_t), void **);
ThinkThenReply thinkthen_cpp_reader_next(void *);
void thinkthen_cpp_reader_free(void *);
ThinkThenReply thinkthen_cpp_span_lines(const uint8_t *, size_t, size_t, size_t, size_t);
}

namespace duckdb {
namespace {
constexpr size_t BYTE_CAP = 16 * 1024 * 1024;
const uint8_t *Bytes(const string &text) { return reinterpret_cast<const uint8_t *>(text.data()); }

struct FilesBind : FunctionData {
	vector<string> paths;
	string options;
	unique_ptr<FunctionData> Copy() const override {
		auto copy = make_uniq<FilesBind>();
		copy->paths = paths;
		copy->options = options;
		return std::move(copy);
	}
	bool Equals(const FunctionData &other) const override {
		auto &value = other.Cast<FilesBind>();
		return paths == value.paths && options == value.options;
	}
};

struct FilesState : GlobalTableFunctionState {
	vector<string> manifest;
	idx_t at = 0;
	int64_t ordinal = 0;
	FileSystem *files = nullptr;
	unique_ptr<FileHandle> handle;
	void *reader = nullptr;
	~FilesState() override { Close(); }
	void Close() {
		if (reader) {
			thinkthen_cpp_reader_free(reader);
			reader = nullptr;
		}
		handle.reset();
	}
};

// This callback executes synchronously on the scan thread. Rust owns the
// writable range only for this call; exceptions never cross the C ABI.
int64_t ReadHandle(void *pointer, uint8_t *buffer, size_t length) noexcept {
	try {
		auto &state = *static_cast<FilesState *>(pointer);
		return state.files->Read(*state.handle, buffer, static_cast<int64_t>(length));
	} catch (...) {
		return -1;
	}
}

unique_ptr<FunctionData> BindFiles(ClientContext &, TableFunctionBindInput &input,
                                  vector<LogicalType> &types, vector<string> &names) {
	auto bind = make_uniq<FilesBind>();
	if (input.inputs[0].IsNull()) {
		throw OrdinaryError("thinkthen usage: source path must be text or a list of paths");
	}
	if (input.inputs[0].type().id() == LogicalTypeId::LIST) {
		for (auto &path : ListValue::GetChildren(input.inputs[0])) {
			if (path.IsNull()) {
				throw OrdinaryError("thinkthen usage: source paths must be nonempty file or folder names");
			}
			bind->paths.push_back(path.GetValue<string>());
		}
	} else {
		bind->paths.push_back(input.inputs[0].GetValue<string>());
	}
	size_t bytes = 0;
	for (auto &path : bind->paths) {
		if (path.empty() || path.find('\0') != string::npos) {
			throw OrdinaryError("thinkthen usage: source paths must be nonempty file or folder names");
		}
		if (path.size() >= BYTE_CAP - bytes) {
			throw OrdinaryError("thinkthen usage: source operands exceed 16 MiB");
		}
		bytes += path.size() + 1;
	}
	if (bind->paths.empty()) {
		throw OrdinaryError("thinkthen usage: source paths must be nonempty file or folder names");
	}
	bind->options = input.inputs.size() > 1 && !input.inputs[1].IsNull() ? input.inputs[1].GetValue<string>() : "{}";
	RustReply checked(thinkthen_cpp_reader_options(Bytes(bind->options), bind->options.size()));
	Checked(checked.value);
	types = {LogicalType::BIGINT, LogicalType::VARCHAR, LogicalType::VARCHAR, LogicalType::BIGINT, LogicalType::BIGINT};
	names = {"ordinal", "record", "file", "first_line", "last_line"};
	return std::move(bind);
}

unique_ptr<GlobalTableFunctionState> InitFiles(ClientContext &context, TableFunctionInitInput &input) {
    auto &bind = input.bind_data->Cast<FilesBind>();
    auto state = make_uniq<FilesState>();
    state->files = &FileSystem::GetFileSystem(context);
    state->manifest = FileManifest(*state->files, bind.paths);
    return std::move(state);
}

struct Json {
	duckdb_yyjson::yyjson_doc *doc;
	explicit Json(ThinkThenReply &reply) : doc(duckdb_yyjson::yyjson_read(reinterpret_cast<char *>(reply.bytes), reply.len, 0)) {
		if (!doc) { throw OrdinaryError("thinkthen defect: invalid reader reply"); }
	}
	~Json() { duckdb_yyjson::yyjson_doc_free(doc); }
	duckdb_yyjson::yyjson_val *Member(const char *name) {
		return duckdb_yyjson::yyjson_obj_get(duckdb_yyjson::yyjson_doc_get_root(doc), name);
	}
	Value Text(const char *name) {
		auto value = Member(name);
		if (!duckdb_yyjson::yyjson_is_str(value)) { throw OrdinaryError("thinkthen defect: invalid source text"); }
		return Value(string(duckdb_yyjson::yyjson_get_str(value), duckdb_yyjson::yyjson_get_len(value)));
	}
	Value Integer(const char *name) {
		auto value = Member(name);
		if (!duckdb_yyjson::yyjson_is_uint(value)) { throw OrdinaryError("thinkthen defect: invalid source line"); }
		auto line = duckdb_yyjson::yyjson_get_uint(value);
		if (line > static_cast<uint64_t>(std::numeric_limits<int64_t>::max())) {
			throw OrdinaryError("thinkthen usage: source line exceeds SQL BIGINT");
		}
		return Value::BIGINT(static_cast<int64_t>(line));
	}
};

void ScanFiles(ClientContext &, TableFunctionInput &input, DataChunk &output) {
	auto &bind = input.bind_data->Cast<FilesBind>();
	auto &state = input.global_state->Cast<FilesState>();
	// Return one record per demand: LIMIT does not open an unread next file.
	for (;;) {
		if (!state.reader) {
			if (state.at == state.manifest.size()) { return; }
			auto &path = state.manifest[state.at++];
			AuthorizeLocalSource(*state.files, path);
			state.handle = state.files->OpenFile(path, FileOpenFlags::FILE_FLAGS_READ);
			if (state.files->GetFileType(*state.handle) != FileType::FILE_TYPE_REGULAR) {
				throw OrdinaryError("thinkthen local: source file must be regular");
			}
			RustReply created(thinkthen_cpp_reader_new(Bytes(path), path.size(), Bytes(bind.options), bind.options.size(),
			                                             &state, ReadHandle, &state.reader));
			Checked(created.value);
		}
		RustReply record(thinkthen_cpp_reader_next(state.reader));
		Checked(record.value);
		if (record.value.len == 0) { state.Close(); continue; }
		Json json(record.value);
		output.SetValue(0, 0, Value::BIGINT(++state.ordinal));
		output.SetValue(1, 0, json.Text("record"));
		output.SetValue(2, 0, json.Text("file"));
		output.SetValue(3, 0, json.Integer("first_line"));
		output.SetValue(4, 0, json.Integer("last_line"));
		output.SetCardinality(1);
		return;
	}
}

void SpanLines(DataChunk &input, ExpressionState &, Vector &output) {
	output.SetVectorType(VectorType::FLAT_VECTOR);
	for (idx_t row = 0; row < input.size(); ++row) {
		bool null = false;
		for (idx_t col = 0; col < 4; ++col) { null |= input.GetValue(col, row).IsNull(); }
		if (null) { FlatVector::SetNull(output, row, true); continue; }
		auto record = input.GetValue(0, row).GetValue<string>();
		auto first = input.GetValue(1, row).GetValue<int64_t>();
		auto start = input.GetValue(2, row).GetValue<int64_t>();
		auto end = input.GetValue(3, row).GetValue<int64_t>();
		if (first <= 0 || start < 0 || end < 0) {
			throw OrdinaryError("thinkthen usage: source span positions are invalid");
		}
		RustReply reply(thinkthen_cpp_span_lines(Bytes(record), record.size(), first, start, end));
		Checked(reply.value);
		Json json(reply.value);
		output.SetValue(row, Value::STRUCT({{"first_line", json.Integer("first_line")}, {"last_line", json.Integer("last_line")}}));
	}
}
} // namespace

void RegisterFiles(ExtensionLoader &loader) {
	TableFunctionSet readers("thinkthen_read_files");
	for (auto &path : vector<LogicalType> {LogicalType::VARCHAR, LogicalType::LIST(LogicalType::VARCHAR)}) {
		for (bool options : {false, true}) {
			vector<LogicalType> arguments {path};
			if (options) { arguments.push_back(LogicalType::VARCHAR); }
			readers.AddFunction(TableFunction("thinkthen_read_files", arguments, ScanFiles, BindFiles, InitFiles));
		}
	}
	RegisterDescribedTable(loader, readers, "Read authorized local files or folders as ordered text records with file and line positions.");
	RegisterDescribedScalar(loader, ScalarFunction("thinkthen_span_lines", {LogicalType::VARCHAR, LogicalType::BIGINT, LogicalType::BIGINT, LogicalType::BIGINT},
	    LogicalType::STRUCT({{"first_line", LogicalType::BIGINT}, {"last_line", LogicalType::BIGINT}}), SpanLines), "Map Unicode scalar offsets in a record to inclusive physical line positions.");
}
} // namespace duckdb
