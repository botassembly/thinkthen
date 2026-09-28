#pragma once

#include "duckdb.hpp"
#include "duckdb/main/client_context.hpp"
#include "scalar_owner.hpp"

namespace duckdb {

void ValidateCompleteListed(const ResolvedQuestion &question, int32_t kind);
void CompleteListed(DataChunk &args, ClientContext &context, int32_t kind, Vector &result);

} // namespace duckdb
