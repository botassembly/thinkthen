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
	const uint8_t *batch_bytes;
	size_t batch_len;
	int64_t throttle;
	int64_t max_requests;
	int64_t max_request_bytes;
	int64_t max_requests_total;
	const uint8_t *backend_bytes;
	size_t backend_len;
	const uint8_t *cache_bytes;
	size_t cache_len;
	int32_t cache_allowed;
	const uint8_t *model_bytes;
	size_t model_len;
	int64_t timeout;
	int64_t max_retries;
	const uint8_t *profile_bytes;
	size_t profile_len;
	const uint8_t *record_bytes;
	size_t record_len;
	const uint8_t *replay_bytes;
	size_t replay_len;
	int32_t record_allowed;
	int32_t replay_allowed;
    const uint8_t *base_url_bytes; size_t base_url_len;
    const uint8_t *input_price_bytes; size_t input_price_len;
    const uint8_t *output_price_bytes; size_t output_price_len;
    int32_t refresh_cache;
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
ThinkThenReply thinkthen_cpp_validate_portable_decide(const uint8_t *question, size_t question_len,
                                                      int32_t from_file, const uint8_t *settings, size_t settings_len,
                                                      const uint8_t *threshold, size_t threshold_len);
ThinkThenReply thinkthen_cpp_portable_decide_group(const uint8_t *question, size_t question_len,
                                                   int32_t from_file, const ThinkThenText *texts, size_t count,
                                                   const uint8_t *settings, size_t settings_len,
                                                   const uint8_t *threshold, size_t threshold_len,
                                                   int64_t query_deadline_ms, ThinkThenSettings session,
                                                   ThinkThenStop stop);
ThinkThenReply thinkthen_cpp_validate_portable_many(const uint8_t *question, size_t question_len,
                                                   int32_t from_file, const uint8_t *keyed, size_t keyed_len,
                                                   const uint8_t *settings, size_t settings_len, int32_t kind);
ThinkThenReply thinkthen_cpp_validate_plan(const uint8_t *question, size_t question_len,
                                          int32_t from_file, const uint8_t *keyed, size_t keyed_len,
                                          const uint8_t *settings, size_t settings_len);
ThinkThenReply thinkthen_cpp_plan(const uint8_t *question, size_t question_len,
                                 int32_t from_file, const uint8_t *keyed, size_t keyed_len,
                                 const uint8_t *settings, size_t settings_len, ThinkThenSettings session);
ThinkThenReply thinkthen_cpp_portable_many(const uint8_t *question, size_t question_len,
                                          int32_t from_file, const uint8_t *keyed, size_t keyed_len,
                                          const uint8_t *settings, size_t settings_len, int32_t kind,
                                          int64_t query_deadline_ms, ThinkThenSettings session, ThinkThenStop stop);
ThinkThenReply thinkthen_cpp_validate_portable_listed(const uint8_t *question, size_t question_len,
                                                     int32_t from_file, const uint8_t *members, size_t members_len,
                                                     const uint8_t *settings, size_t settings_len, int32_t kind);
ThinkThenReply thinkthen_cpp_portable_listed_group(const uint8_t *question, size_t question_len,
                                                  int32_t from_file, const ThinkThenText *texts, size_t count,
                                                  const uint8_t *members, size_t members_len,
                                                  const uint8_t *settings, size_t settings_len, int32_t kind,
                                                  int64_t query_deadline_ms, ThinkThenSettings session, ThinkThenStop stop);
ThinkThenReply thinkthen_cpp_validate_portable_scalar(const uint8_t *question, size_t question_len,
                                                     int32_t from_file, const uint8_t *settings, size_t settings_len,
                                                     int32_t kind);
ThinkThenReply thinkthen_cpp_portable_scalar_group(const uint8_t *question, size_t question_len,
                                                  int32_t from_file, const ThinkThenText *texts, size_t count,
                                                  const uint8_t *settings, size_t settings_len, int32_t kind,
                                                  int64_t query_deadline_ms, ThinkThenSettings session, ThinkThenStop stop);
ThinkThenReply thinkthen_cpp_validate_portable_annotate(const uint8_t *argument, size_t argument_len,
                                                       int32_t from_file, const uint8_t *settings, size_t settings_len);
ThinkThenReply thinkthen_cpp_portable_annotate_group(const uint8_t *argument, size_t argument_len,
                                                    int32_t from_file, const ThinkThenText *texts, size_t count,
                                                    const uint8_t *settings, size_t settings_len,
                                                    int64_t query_deadline_ms, ThinkThenSettings session,
                                                    ThinkThenStop stop);
ThinkThenReply thinkthen_cpp_portable_try_details_group(const uint8_t *argument, size_t argument_len,
                                                       int32_t from_file, const ThinkThenText *texts, size_t count,
                                                       const uint8_t *settings, size_t settings_len,
                                                       int64_t query_deadline_ms, ThinkThenSettings session,
                                                       ThinkThenStop stop);
ThinkThenReply thinkthen_cpp_validate_portable_nested(const uint8_t *argument, size_t argument_len,
                                                     int32_t from_file, const ThinkThenText *members,
                                                     size_t member_count, const uint8_t *settings,
                                                     size_t settings_len, int32_t kind);
ThinkThenReply thinkthen_cpp_portable_nested_group(const uint8_t *argument, size_t argument_len,
                                                  int32_t from_file, const ThinkThenText *members,
                                                  size_t member_count, const ThinkThenText *texts,
                                                  size_t text_count, const uint8_t *settings,
                                                  size_t settings_len, int32_t kind, int64_t query_deadline_ms,
                                                  ThinkThenSettings session, ThinkThenStop stop);
ThinkThenReply thinkthen_cpp_validate_set(const uint8_t *bytes, size_t len, int32_t from_file);
ThinkThenReply thinkthen_cpp_validate_listed(const uint8_t *question, size_t question_len,
                                             const ThinkThenText *members, size_t member_count, int32_t kind);
ThinkThenReply thinkthen_cpp_validate_portable_find(const uint8_t *question, size_t question_len,
                                                   const ThinkThenText *units, size_t unit_count,
                                                   const uint8_t *settings, size_t settings_len);
ThinkThenReply thinkthen_cpp_portable_find(const uint8_t *question, size_t question_len,
                                          const ThinkThenText *units, size_t unit_count,
                                          const uint8_t *settings, size_t settings_len,
                                          int64_t query_deadline_ms, ThinkThenSettings session, ThinkThenStop stop);
ThinkThenReply thinkthen_cpp_scalar_group(const uint8_t *question, size_t question_len, const ThinkThenText *texts,
                                          size_t count, int64_t deadline_ms, int32_t kind,
                                          ThinkThenSettings settings, int32_t from_file,
                                          const uint8_t *context, size_t context_len, ThinkThenStop stop);
ThinkThenReply thinkthen_cpp_try_details_row(const uint8_t *question, size_t question_len,
                                             const uint8_t *evidence, size_t evidence_len,
                                             int64_t deadline_ms, ThinkThenSettings settings, int32_t from_file,
                                             ThinkThenStop stop);
ThinkThenReply thinkthen_cpp_try_details_group(const uint8_t *question, size_t question_len,
                                               const ThinkThenText *texts, size_t count, int64_t deadline_ms,
                                               ThinkThenSettings settings, int32_t from_file,
                                               const uint8_t *context, size_t context_len, ThinkThenStop stop);
ThinkThenReply thinkthen_cpp_listed_group(const uint8_t *question, size_t question_len,
                                          const ThinkThenText *members, size_t member_count,
                                          const ThinkThenText *texts, size_t text_count,
                                          int64_t deadline_ms, int32_t kind, ThinkThenSettings settings,
                                          const uint8_t *context, size_t context_len, ThinkThenStop stop);
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
ThinkThenReply thinkthen_cpp_usage_status();
ThinkThenReply thinkthen_cpp_relate_validate(const uint8_t *rule, size_t rule_len,
                                            const ThinkThenText *members, size_t member_count,
                                            int32_t list, int32_t from_file,
                                            const uint8_t *call_settings, size_t call_settings_len,
                                            ThinkThenSettings settings);
ThinkThenReply thinkthen_cpp_relate_rows(const uint8_t *rule, size_t rule_len,
                                        const ThinkThenText *members, size_t member_count,
                                        int32_t list, int32_t from_file,
                                        const ThinkThenText *ids, const ThinkThenText *names,
                                        const ThinkThenText *kinds, size_t count, int64_t deadline_ms,
                                        const uint8_t *call_settings, size_t call_settings_len,
                                        ThinkThenSettings settings, ThinkThenStop stop);
ThinkThenReply thinkthen_cpp_relate_plan(const uint8_t *plan, size_t len, uint64_t holding);
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

// Host-side messages may contain caller SQL text, so only this boundary's
// known nonretryable classification is appended, before any SQL hint lines.
inline string HostErrorText(string text) {
	const auto newline = text.find('\n');
	text.insert(newline == string::npos ? text.size() : newline, " (retryable: no)");
	return text;
}

// A Rust bridge reply may already carry its typed retryability. That label
// never comes from a host query error passed to OrdinaryError.
inline string BridgeErrorText(string text) {
	const auto first = text.substr(0, text.find('\n'));
	if ((first.size() >= 15 && first.compare(first.size() - 15, 15, "(retryable: no)") == 0) ||
	    (first.size() >= 16 && first.compare(first.size() - 16, 16, "(retryable: yes)") == 0)) {
		return text;
	}
	return HostErrorText(std::move(text));
}

template <typename... Args>
inline InvalidInputException OrdinaryError(const string &format, Args... args) {
	const auto text = HostErrorText(StringUtil::Format(format, args...));
	return InvalidInputException("%s", text.c_str());
}

inline void Checked(const ThinkThenReply &reply) {
	if (reply.status != 0) {
		throw InvalidInputException("%s", BridgeErrorText(ReplyText(reply)).c_str());
	}
}

} // namespace duckdb
