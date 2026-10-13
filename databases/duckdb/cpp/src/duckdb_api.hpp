#pragma once

#include "duckdb.hpp"
#include "duckdb/catalog/catalog.hpp"
#include "duckdb/parser/parsed_data/create_info.hpp"
#include "duckdb/parser/parser.hpp"
#include "duckdb/planner/expression/bound_function_expression.hpp"

namespace duckdb {

using ThinkThenScalarBinder = unique_ptr<FunctionData> (*)(ClientContext &, const string &,
                                                         vector<unique_ptr<Expression>> &);

// Keep the host version boundary here; SQL callers and the Rust bridge share one contract.
#if defined(THINKTHEN_DUCKDB_V2) && THINKTHEN_DUCKDB_V2
using ThinkThenIdentifier = Identifier;
using ThinkThenColumnNames = vector<Identifier>;

template <ThinkThenScalarBinder Binder>
unique_ptr<FunctionData> ThinkThenBindScalar(BindScalarFunctionInput &input) {
    return Binder(input.GetClientContext(), input.GetBoundFunction().GetName().GetIdentifierName(), input.GetArguments());
}
inline const FunctionData &ThinkThenBindData(const BoundFunctionExpression &expression) {
    return *expression.BindInfo();
}
inline const string &ThinkThenFunctionName(const BoundFunctionExpression &expression) {
    return expression.Function().GetName().GetIdentifierName();
}
inline void ThinkThenSpecialNulls(ScalarFunction &function) {
    function.SetNullHandling(FunctionNullHandling::SPECIAL_HANDLING);
}
inline void ThinkThenArgument(ScalarFunction &function, idx_t index, LogicalType type) {
    function.GetSignature().GetParameter(index).SetType(std::move(type));
}
template <class Function>
vector<LogicalType> ThinkThenRegistrationTypes(const Function &function) {
    vector<LogicalType> types;
    for (const auto &parameter : function->GetSignature().GetParameters()) {
        types.push_back(parameter.GetType());
    }
    return types;
}
inline Value ThinkThenJSONValue(const string &text) {
    return Value(text).WithType(LogicalType::JSON());
}
inline Parser ThinkThenMacroParser() {
    return Parser::GetBuiltinParser();
}
inline void ThinkThenMacroSchema(CreateInfo &info) {
    info.SetSchema(Identifier::DefaultSchema());
}
template <class Result>
Value ThinkThenQueryValue(Result &result, idx_t column, idx_t row) {
    return result.Collection().GetValue(column, row);
}
inline bool ThinkThenTemporaryTable(ClientContext &context, const string &name) {
    EntryLookupInfo lookup(CatalogType::TABLE_ENTRY, QualifiedName("temp", "main", Identifier(name)));
    return bool(Catalog::GetEntry(context, lookup, OnEntryNotFound::RETURN_NULL));
}
#else
using ThinkThenIdentifier = string;
using ThinkThenColumnNames = vector<string>;

template <ThinkThenScalarBinder Binder>
unique_ptr<FunctionData> ThinkThenBindScalar(ClientContext &context, ScalarFunction &function,
                                           vector<unique_ptr<Expression>> &arguments) {
    return Binder(context, function.name, arguments);
}
inline const FunctionData &ThinkThenBindData(const BoundFunctionExpression &expression) {
    return *expression.bind_info;
}
inline const string &ThinkThenFunctionName(const BoundFunctionExpression &expression) {
    return expression.function.name;
}
inline void ThinkThenSpecialNulls(ScalarFunction &function) {
    function.null_handling = FunctionNullHandling::SPECIAL_HANDLING;
}
inline void ThinkThenArgument(ScalarFunction &function, idx_t index, LogicalType type) {
    function.arguments[index] = std::move(type);
}
template <class Function>
vector<LogicalType> ThinkThenRegistrationTypes(const Function &function) {
    return function.arguments;
}
inline Value ThinkThenJSONValue(const string &text) {
    Value value(text);
    value.GetTypeMutable() = LogicalType::JSON();
    return value;
}
inline Parser ThinkThenMacroParser() {
    return Parser();
}
inline void ThinkThenMacroSchema(CreateInfo &info) {
    info.schema = DEFAULT_SCHEMA;
}
template <class Result>
Value ThinkThenQueryValue(Result &result, idx_t column, idx_t row) {
    return result.GetValue(column, row);
}
inline bool ThinkThenTemporaryTable(ClientContext &context, const string &name) {
    EntryLookupInfo lookup(CatalogType::TABLE_ENTRY, name);
    return bool(Catalog::GetEntry(context, "temp", "main", lookup, OnEntryNotFound::RETURN_NULL));
}
#endif

} // namespace duckdb
