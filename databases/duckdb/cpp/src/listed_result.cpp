#include "listed_result.hpp"
#include "duckdb/common/exception.hpp"

#include <cstring>

namespace duckdb {
namespace {

struct Cursor {
	const uint8_t *bytes;
	size_t length;
	size_t at = 0;

	void Need(size_t count) const {
		if (count > length - at) {
			throw InvalidInputException("thinkthen defect: the bridge returned truncated listed values");
		}
	}
	uint8_t Tag() {
		Need(1);
		return bytes[at++];
	}
	uint32_t Count() {
		Need(sizeof(uint32_t));
		uint32_t result;
		std::memcpy(&result, bytes + at, sizeof(result));
		at += sizeof(result);
		return result;
	}
	string Text() {
		auto length = Count();
		Need(length);
		string result(reinterpret_cast<const char *>(bytes + at), length);
		at += length;
		return result;
	}
	Value Next(int32_t kind) {
		switch (Tag()) {
		case 0:
			return Value(kind == 5 ? LogicalType::DOUBLE : kind == 6 ? LogicalType::LIST(LogicalType::VARCHAR)
			                                                                   : LogicalType::VARCHAR);
		case 1:
			if (kind == 4) {
				return Value(Text());
			}
			break;
		case 2:
			if (kind == 5) {
				Need(sizeof(double));
				double value;
				std::memcpy(&value, bytes + at, sizeof(value));
				at += sizeof(value);
				return Value::DOUBLE(value);
			}
			break;
		case 3:
			if (kind == 6) {
				const auto count = Count();
				vector<Value> values;
				values.reserve(count);
				for (uint32_t index = 0; index < count; ++index) {
					values.emplace_back(Text());
				}
				return Value::LIST(LogicalType::VARCHAR, values);
			}
			break;
		}
		throw InvalidInputException("thinkthen defect: the bridge returned another listed kind");
	}
};

} // namespace

vector<Value> DecodeListed(const uint8_t *bytes, size_t length, idx_t count, int32_t kind) {
	if (!bytes) {
		throw InvalidInputException("thinkthen defect: the bridge returned no listed values");
	}
	Cursor cursor {bytes, length};
	vector<Value> values;
	values.reserve(count);
	for (idx_t index = 0; index < count; ++index) {
		values.push_back(cursor.Next(kind));
	}
	if (cursor.at != length) {
		throw InvalidInputException("thinkthen defect: the bridge returned extra listed bytes");
	}
	return values;
}

} // namespace duckdb
