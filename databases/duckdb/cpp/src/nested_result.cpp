#include "nested_result.hpp"
#include "duckdb/common/exception.hpp"

#include <cstring>

namespace duckdb {

LogicalType NestedType(int32_t kind) {
	if (kind == 8) {
		return LogicalType::LIST(LogicalType::STRUCT({{"text", LogicalType::VARCHAR}, {"start", LogicalType::BIGINT},
		                                             {"end", LogicalType::BIGINT}, {"length", LogicalType::BIGINT},
		                                             {"kind", LogicalType::VARCHAR}, {"strength", LogicalType::DOUBLE}}));
	}
	return LogicalType::LIST(LogicalType::STRUCT({{"relation", LogicalType::VARCHAR}, {"source", LogicalType::VARCHAR},
	                                         {"source_kind", LogicalType::VARCHAR}, {"target", LogicalType::VARCHAR},
	                                         {"target_kind", LogicalType::VARCHAR}, {"probability", LogicalType::DOUBLE}}));
}

namespace {

struct Cursor {
	const uint8_t *bytes;
	size_t length;
	size_t at = 0;

	void Need(size_t amount) const {
		if (amount > length - at) {
			throw InvalidInputException("thinkthen defect: the bridge returned truncated nested values");
		}
	}
	template <class T> T Read() {
		Need(sizeof(T));
		T value;
		std::memcpy(&value, bytes + at, sizeof(T));
		at += sizeof(T);
		return value;
	}
	string Text() {
		auto size = Read<uint32_t>();
		Need(size);
		string value(reinterpret_cast<const char *>(bytes + at), size);
		at += size;
		return value;
	}
	Value Entity() {
		return Value::STRUCT({{"text", Value(Text())}, {"start", Value::BIGINT(Read<int64_t>())},
		                      {"end", Value::BIGINT(Read<int64_t>())}, {"length", Value::BIGINT(Read<int64_t>())},
		                      {"kind", Value(Text())}, {"strength", Value::DOUBLE(Read<double>())}});
	}
	Value Relation() {
		return Value::STRUCT({{"relation", Value(Text())}, {"source", Value(Text())},
		                      {"source_kind", Value(Text())}, {"target", Value(Text())},
		                      {"target_kind", Value(Text())}, {"probability", Value::DOUBLE(Read<double>())}});
	}
	Value Row(int32_t kind) {
		const auto count = Read<uint32_t>();
		Need(count); // Every value consumes at least one byte, before any allocation.
		vector<Value> values;
		values.reserve(count);
		for (uint32_t index = 0; index < count; ++index) {
			values.push_back(kind == 8 ? Entity() : Relation());
		}
		return Value::LIST(ListType::GetChildType(NestedType(kind)), values);
	}
};

} // namespace

vector<Value> DecodeNested(const uint8_t *bytes, size_t length, idx_t count, int32_t kind) {
	if (!bytes || (kind != 8 && kind != 9)) {
		throw InvalidInputException("thinkthen defect: the bridge returned no nested values");
	}
	Cursor cursor {bytes, length};
	vector<Value> values;
	values.reserve(count);
	for (idx_t index = 0; index < count; ++index) {
		values.push_back(cursor.Row(kind));
	}
	if (cursor.at != length) {
		throw InvalidInputException("thinkthen defect: the bridge returned extra nested bytes");
	}
	return values;
}

} // namespace duckdb
