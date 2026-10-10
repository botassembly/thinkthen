#include "images.hpp"
#include "json_result.hpp"
#include "bridge.hpp"
#include "files_manifest.hpp"
#include "portable.hpp"
#include "scalar_owner.hpp"
#include "scalar_settings.hpp"
#include "duckdb/common/file_system.hpp"
#include "duckdb/common/weak_ptr_ipp.hpp"
#include "duckdb/main/extension/extension_loader.hpp"
#include "duckdb/planner/expression/bound_function_expression.hpp"
#include "yyjson.hpp"

extern "C" {
ThinkThenReply thinkthen_cpp_image(ThinkThenText, ThinkThenText);
ThinkThenReply thinkthen_cpp_validate_images(ThinkThenText, int32_t, const ThinkThenImage *, size_t,
                                            ThinkThenText, ThinkThenText, int32_t);
ThinkThenReply thinkthen_cpp_images(ThinkThenText, int32_t, const ThinkThenImage *, size_t,
                                   ThinkThenText, ThinkThenText, int32_t, int64_t,
                                   ThinkThenSettings, ThinkThenStop);
ThinkThenReply thinkthen_cpp_image_read(void *, int64_t (*)(void *, uint8_t *, size_t));
}

namespace duckdb {
namespace {
constexpr size_t MAX_IMAGE_BYTES = 24 * 1024 * 1024;
constexpr size_t MAX_IMAGES = 8;
ThinkThenText View(const string &text) { return {reinterpret_cast<const uint8_t *>(text.data()), text.size()}; }
Value ImageValue(const string &media, const string &data, Value file = Value(LogicalType::VARCHAR)) {
	return Value::STRUCT({{"media", Value(media)}, {"data", Value::BLOB_RAW(data)}, {"file", std::move(file)}});
}
struct Bind : FunctionData {
	weak_ptr<ClientContext> context;
	explicit Bind(weak_ptr<ClientContext> context) : context(std::move(context)) {}
	unique_ptr<FunctionData> Copy() const override { return make_uniq<Bind>(context); }
	bool Equals(const FunctionData &other) const override { return context.lock() == other.Cast<Bind>().context.lock(); }
};
unique_ptr<FunctionData> BindImages(ClientContext &context, ScalarFunction &, vector<unique_ptr<Expression>> &) {
	context.registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	return make_uniq<Bind>(context.shared_from_this());
}
void Constructor(DataChunk &args, ExpressionState &, Vector &result) {
	for (idx_t row = 0; row < args.size(); ++row) {
		auto data = args.data[0].GetValue(row), media = args.data[1].GetValue(row);
		if (data.IsNull() || media.IsNull()) { result.SetValue(row, Value(ImageType())); continue; }
		const auto bytes = StringValue::Get(data), mime = media.GetValue<string>();
		RustReply checked(thinkthen_cpp_image(View(mime), View(bytes)));
		Checked(checked.value);
		result.SetValue(row, ImageValue(mime, bytes));
	}
}
struct Handle { FileSystem &files; unique_ptr<FileHandle> handle; };
int64_t ReadHandle(void *opaque, uint8_t *bytes, size_t count) noexcept {
	try { auto &held = *static_cast<Handle *>(opaque); return held.files.Read(*held.handle, bytes, count); }
	catch (...) { return -1; }
}
void File(DataChunk &args, ExpressionState &state, Vector &result) {
	auto context = state.expr.Cast<BoundFunctionExpression>().bind_info->Cast<Bind>().context.lock();
	if (!context) { throw OrdinaryError("thinkthen defect: the caller session ended"); }
	auto &files = FileSystem::GetFileSystem(*context);
	for (idx_t row = 0; row < args.size(); ++row) {
		auto value = args.data[0].GetValue(row);
		if (value.IsNull()) { result.SetValue(row, Value(ImageType())); continue; }
		const auto path = value.GetValue<string>();
		if (path.empty() || path.find('\0') != string::npos || path.find("://") != string::npos || files.DirectoryExists(path)) {
			throw OrdinaryError("thinkthen usage: image path must name one local regular file");
		}
		auto manifest = FileManifest(files, {path});
		if (manifest.size() != 1) { throw OrdinaryError("thinkthen usage: image path must name one local regular file"); }
		Handle held {files, files.OpenFile(manifest[0], FileOpenFlags::FILE_FLAGS_READ)};
		RustReply reply(thinkthen_cpp_image_read(&held, ReadHandle));
		Checked(reply.value);
		if (!reply.value.bytes || reply.value.len < 1) { throw OrdinaryError("thinkthen defect: empty image reader reply"); }
		const auto mime = reply.value.bytes[0] == 1 ? "image/png" : "image/jpeg";
		result.SetValue(row, ImageValue(mime, string(reinterpret_cast<char *>(reply.value.bytes + 1), reply.value.len - 1), Value(manifest[0])));
	}
}
struct Input {
	ResolvedQuestion question;
	string settings;
	std::optional<string> text;
	vector<std::pair<string, string>> images;
	vector<ThinkThenImage> Views() const { return ImageViews(images); }
	ThinkThenText Text() const { return text ? View(*text) : ThinkThenText {nullptr, 0}; }
};
struct Json {
	duckdb_yyjson::yyjson_doc *doc;
	explicit Json(const ThinkThenReply &reply) : doc(duckdb_yyjson::yyjson_read(reinterpret_cast<char *>(reply.bytes), reply.len, 0)) {
		if (!doc) { throw OrdinaryError("thinkthen defect: invalid image details reply"); }
	}
	~Json() { duckdb_yyjson::yyjson_doc_free(doc); }
};
Value Answer(const ThinkThenReply &reply, int32_t kind) {
	if (kind == 2) { return ThinkThenJSON(ReplyText(reply)); }
	Json json(reply);
	auto value = duckdb_yyjson::yyjson_obj_get(duckdb_yyjson::yyjson_doc_get_root(json.doc), "value");
	if (!value) { throw OrdinaryError("thinkthen defect: image details omitted value"); }
	if (duckdb_yyjson::yyjson_is_null(value)) { return Value(kind == 0 ? LogicalType::BOOLEAN : LogicalType::VARCHAR); }
	if (kind == 0 && duckdb_yyjson::yyjson_is_bool(value)) { return Value::BOOLEAN(duckdb_yyjson::yyjson_get_bool(value)); }
	if (kind == 4 && duckdb_yyjson::yyjson_is_str(value)) { return Value(duckdb_yyjson::yyjson_get_str(value)); }
	if (kind == 5 && duckdb_yyjson::yyjson_is_num(value)) { return Value::DOUBLE(duckdb_yyjson::yyjson_get_num(value)); }
	throw OrdinaryError("thinkthen defect: image details held another judgment");
}
void Judge(DataChunk &args, ExpressionState &state, Vector &result) {
	auto &expr = state.expr.Cast<BoundFunctionExpression>();
	auto context = expr.bind_info->Cast<Bind>().context.lock();
	if (!context) { throw OrdinaryError("thinkthen defect: the caller session ended"); }
	auto owner = context->registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	const auto name = expr.function.name;
	const int32_t kind = name == "thinkthen_native_decide_images" ? 0 : name == "thinkthen_native_choose_images" ? 4 : name == "thinkthen_native_score_images" ? 5 : 2;
	vector<std::optional<Input>> inputs(args.size());
	for (idx_t row = 0; row < args.size(); ++row) {
		auto question = args.data[0].GetValue(row), images = args.data[1].GetValue(row);
		if (question.IsNull() || images.IsNull()) { continue; }
		Input input;
		input.question = owner->Resolve(*context, question.GetValue<string>());
		auto setting = args.data[3].GetValue(row), text = args.data[2].GetValue(row);
		input.settings = setting.IsNull() ? "{}" : setting.GetValue<string>();
		if (!text.IsNull()) { input.text = text.GetValue<string>(); }
		input.images = ImageMembers(images);
		auto views = input.Views();
		RustReply checked(thinkthen_cpp_validate_images(View(input.question.text), input.question.from_file, views.data(), views.size(), input.Text(), View(input.settings), kind));
		Checked(checked.value);
		inputs[row] = std::move(input);
	}
	const auto session = Settings(*context);
	for (idx_t row = 0; row < args.size(); ++row) {
		if (!inputs[row]) { result.SetValue(row, Value(result.GetType())); continue; }
		auto &input = *inputs[row];
		auto views = input.Views();
		RustReply reply(thinkthen_cpp_images(View(input.question.text), input.question.from_file, views.data(), views.size(), input.Text(), View(input.settings), kind, owner->Remaining(*context), session.Bridge(), StopFor(*context)));
		Checked(reply.value);
		result.SetValue(row, Answer(reply.value, kind));
	}
}
void Register(ExtensionLoader &loader, const string &name, vector<LogicalType> arguments, LogicalType result,
              scalar_function_t function, bool bind) {
	ScalarFunction scalar(name, std::move(arguments), std::move(result), function, bind ? BindImages : nullptr);
	scalar.null_handling = FunctionNullHandling::SPECIAL_HANDLING;
	scalar.SetStability(FunctionStability::VOLATILE);
	loader.RegisterFunction(scalar);
}
} // namespace
LogicalType ImageType() {
	return LogicalType::STRUCT({{"media", LogicalType::VARCHAR}, {"data", LogicalType::BLOB}, {"file", LogicalType::VARCHAR}});
}
vector<std::pair<string, string>> ImageMembers(const Value &images) {
    vector<std::pair<string, string>> out;
	auto &members = ListValue::GetChildren(images);
	if (members.empty() || members.size() > MAX_IMAGES) { throw OrdinaryError("thinkthen usage: image evidence requires 1 to 8 images"); }
	size_t total = 0;
	for (auto &image : members) {
		if (image.IsNull()) { throw OrdinaryError("thinkthen usage: image list contains NULL"); }
		auto &fields = StructValue::GetChildren(image);
		if (fields[0].IsNull() || fields[1].IsNull()) { throw OrdinaryError("thinkthen usage: image media and data must be non-NULL"); }
		auto &bytes = StringValue::Get(fields[1]);
		if (bytes.size() > MAX_IMAGE_BYTES - total) { throw OrdinaryError("thinkthen usage: image evidence exceeds the 25165824 compressed byte SDK limit"); }
		total += bytes.size();
		out.emplace_back(fields[0].GetValue<string>(), bytes);
	}
    return out;
}
vector<ThinkThenImage> ImageViews(const vector<std::pair<string, string>> &images) {
    vector<ThinkThenImage> out;
    for (auto &image : images) { out.push_back({View(image.first), View(image.second)}); }
    return out;
}
void RegisterImages(ExtensionLoader &loader) {
	Register(loader, "thinkthen_image", {LogicalType::BLOB, LogicalType::VARCHAR}, ImageType(), Constructor, false);
	Register(loader, "thinkthen_image_file", {LogicalType::VARCHAR}, ImageType(), File, true);
	for (auto verb : {"decide", "choose", "score", "details"}) {
		const string name = string("thinkthen_native_") + verb + "_images";
		Register(loader, name, {LogicalType::VARCHAR, LogicalType::LIST(ImageType()), LogicalType::VARCHAR, LogicalType::VARCHAR},
		         string(verb) == "decide" ? LogicalType::BOOLEAN : string(verb) == "score" ? LogicalType::DOUBLE : string(verb) == "details" ? LogicalType::JSON() : LogicalType::VARCHAR, Judge, true);
		RegisterPortableMacro(loader, string("CREATE MACRO thinkthen_") + verb + "_images(question, images, text := NULL, settings := NULL) AS " + name + "(question, images, text, settings)");
	}
}
} // namespace duckdb
