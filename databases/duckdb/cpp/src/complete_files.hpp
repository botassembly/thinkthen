#pragma once
#include "bridge.hpp"
namespace duckdb {
bool CompleteFileCall(ClientContext &,const string &,const string &,const string &,const string &,const void *,int64_t,ThinkThenSettings,string &,const string &native_reading = "");
string CompleteAdmissionError(const std::exception &error);
}
