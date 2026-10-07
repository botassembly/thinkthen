#pragma once
#include "duckdb/main/client_context.hpp"
namespace duckdb {
string CompleteFileInputs(ClientContext &context, const string &inputs, string &failure);
string CompleteAdmissionError(const std::exception &error);
}
