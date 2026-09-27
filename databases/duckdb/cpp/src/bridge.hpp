#pragma once

#include "duckdb.hpp"
#include "duckdb/common/exception.hpp"
#include "duckdb/main/client_context.hpp"

#include <cstddef>
#include <cstdint>

static_assert(sizeof(double) == 8, "the Rust bridge returns eight-byte probabilities");

extern "C" {
struct ThinkThenReply {
	int32_t status;
	uint8_t *bytes;
	size_t len;
};
struct ThinkThenText {
	const uint8_t *bytes;
	size_t len;
};
struct ThinkThenSettings {
	int64_t throttle;
	int64_t max_requests;
	int64_t max_requests_total;
	const uint8_t *cache_bytes;
	size_t cache_len;
	int32_t cache_allowed;
};
struct ThinkThenStop {
	void *context;
	int32_t (*interrupted)(void *);
};
int32_t thinkthen_cpp_init();
void *thinkthen_cpp_query_begin();
int32_t thinkthen_cpp_query_stopped(const void *scope);
void thinkthen_cpp_query_end(void *scope);
ThinkThenReply thinkthen_cpp_validate_question(const uint8_t *bytes, size_t len, int32_t from_file);
ThinkThenReply thinkthen_cpp_validate_set(const uint8_t *bytes, size_t len, int32_t from_file);
ThinkThenReply thinkthen_cpp_validate_listed(const uint8_t *question, size_t question_len,
                                             const ThinkThenText *members, size_t member_count, int32_t kind);
ThinkThenReply thinkthen_cpp_scalar_group(const uint8_t *question, size_t question_len, const ThinkThenText *texts,
                                          size_t count, int64_t deadline_ms, int32_t kind,
                                          ThinkThenSettings settings, int32_t from_file, ThinkThenStop stop);
ThinkThenReply thinkthen_cpp_try_details_row(const uint8_t *question, size_t question_len,
                                             const uint8_t *evidence, size_t evidence_len,
                                             int64_t deadline_ms, ThinkThenSettings settings, int32_t from_file,
                                             ThinkThenStop stop);
ThinkThenReply thinkthen_cpp_listed_group(const uint8_t *question, size_t question_len,
                                          const ThinkThenText *members, size_t member_count,
                                          const ThinkThenText *texts, size_t text_count,
                                          int64_t deadline_ms, int32_t kind, ThinkThenSettings settings, ThinkThenStop stop);
ThinkThenReply thinkthen_cpp_validate_nested(const uint8_t *argument, size_t argument_len,
                                             const ThinkThenText *members, size_t member_count,
                                             int32_t kind, int32_t from_file);
ThinkThenReply thinkthen_cpp_nested_group(const uint8_t *argument, size_t argument_len,
                                          const ThinkThenText *members, size_t member_count,
                                          const ThinkThenText *texts, size_t text_count,
                                          int64_t deadline_ms, int32_t kind, ThinkThenSettings settings,
                                          int32_t from_file, ThinkThenStop stop);
void thinkthen_cpp_free(uint8_t *bytes, size_t len);
ThinkThenReply thinkthen_cpp_usage();
ThinkThenReply thinkthen_cpp_warm(const uint8_t *question, size_t question_len,
                                 const ThinkThenText *texts, size_t count, int32_t from_file, ThinkThenStop stop);
}

namespace duckdb {

bool QueryInterrupted(ClientContext &context) noexcept;

inline int32_t Interrupted(void *opaque) noexcept {
	try {
		return QueryInterrupted(*static_cast<ClientContext *>(opaque)) ? 1 : 0;
	} catch (...) {
		return 1;
	}
}

inline ThinkThenStop StopFor(ClientContext &context) {
	return {&context, Interrupted};
}

struct RustReply {
	explicit RustReply(ThinkThenReply value) : value(value) {
	}
	~RustReply() {
		thinkthen_cpp_free(value.bytes, value.len);
	}
	RustReply(const RustReply &) = delete;
	RustReply &operator=(const RustReply &) = delete;
	ThinkThenReply value;
};

inline string ReplyText(const ThinkThenReply &reply) {
	if (!reply.bytes) {
		return "thinkthen defect: the bridge returned no error buffer";
	}
	return string(reinterpret_cast<const char *>(reply.bytes), reply.len);
}

inline void Checked(const ThinkThenReply &reply) {
	if (reply.status != 0) {
		throw InvalidInputException("%s", ReplyText(reply).c_str());
	}
}

} // namespace duckdb
