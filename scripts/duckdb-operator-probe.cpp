#include "duckdb.hpp"
#include "duckdb/main/client_context.hpp"
#include "duckdb/parser/parsed_data/create_table_function_info.hpp"
#include "duckdb/parser/parsed_data/create_scalar_function_info.hpp"
#include "duckdb/planner/operator/logical_extension_operator.hpp"
#include "duckdb/planner/operator/logical_get.hpp"
#include "duckdb/execution/physical_plan_generator.hpp"
#include <atomic>
#include <iostream>
#include <mutex>
#include <set>
#include <thread>
using namespace duckdb;
static std::atomic<int> calls{0}, finals{0}, chunks{0};
static std::mutex seen_mutex;
static std::set<std::thread::id> seen;
struct Local : LocalTableFunctionState { int64_t next=0; Local(){++calls;} };
static unique_ptr<LocalTableFunctionState> Init(ExecutionContext &, TableFunctionInitInput &, GlobalTableFunctionState *) { return make_uniq<Local>(); }
static OperatorResultType Run(ExecutionContext &, TableFunctionInput &data, DataChunk &input, DataChunk &output) {
  auto &state=data.local_state->Cast<Local>(); ++chunks;
  for(idx_t i=0;i<input.size();++i) { if(input.GetValue(0,i).GetValue<int64_t>()!=state.next++) throw InvalidInputException("unordered input"); }
  output.Reference(input); return OperatorResultType::NEED_MORE_INPUT;
}
static OperatorFinalizeResultType Final(ExecutionContext &, TableFunctionInput &data, DataChunk &output) {
  if(++finals!=1) throw InvalidInputException("more than one final drain");
  output.SetValue(0,0,Value::BIGINT(data.local_state->Cast<Local>().next)); output.SetCardinality(1);
  return OperatorFinalizeResultType::FINISHED;
}
struct Sequential : PhysicalOperator {
 Sequential(PhysicalPlan &plan, vector<LogicalType> types) : PhysicalOperator(plan, PhysicalOperatorType::EXTENSION, std::move(types), 1) {}
 OperatorResultType Execute(ExecutionContext &, DataChunk &input, DataChunk &output, GlobalOperatorState &, OperatorState &) const override { output.Reference(input); return OperatorResultType::NEED_MORE_INPUT; }
 bool ParallelOperator() const override { return false; }
 OrderPreservationType OperatorOrder() const override { return OrderPreservationType::FIXED_ORDER; }
};
struct LogicalSequential : LogicalExtensionOperator {
 explicit LogicalSequential(unique_ptr<LogicalOperator> child) { children.push_back(std::move(child)); }
 vector<ColumnBinding> GetColumnBindings() override { return children[0]->GetColumnBindings(); }
 void ResolveTypes() override { types=children[0]->types; }
 string GetExtensionName() const override { return "probe"; }
 bool SupportSerialization() const override { return false; }
 PhysicalOperator &CreatePlan(ClientContext &, PhysicalPlanGenerator &planner) override { auto &child=planner.CreatePlan(*children[0]); auto &op=planner.Make<Sequential>(types); op.children.push_back(child); return op; }
};
static unique_ptr<LogicalOperator> Bind(ClientContext &, TableFunctionBindInput &input, idx_t index, vector<string> &names) {
 names={"record"}; auto fn=input.table_function; fn.bind_operator=nullptr;
 auto get=make_uniq<LogicalGet>(index,std::move(fn),nullptr,vector<LogicalType>{LogicalType::BIGINT},names);
 get->input_table_types=input.input_table_types; get->input_table_names=input.input_table_names; get->AddColumnId(0);
 return make_uniq<LogicalSequential>(std::move(get));
}
static void Check(Connection &con, const string &sql) { auto result=con.Query(sql); if(result->HasError()) throw std::runtime_error(result->GetError()); }
int main() {
 try {
 DuckDB db(nullptr); Connection con(db); Check(con,"SET threads=4");
 TableFunction fn("probe",{LogicalType::TABLE},nullptr); fn.bind_operator=Bind; fn.in_out_function=Run; fn.in_out_function_final=Final; fn.init_local=Init;
 CreateTableFunctionInfo info(fn); con.context->RegisterFunction(info);
 ScalarFunction mark("mark",{LogicalType::BIGINT},LogicalType::BIGINT,[](DataChunk &args,ExpressionState &,Vector &out){ {std::lock_guard<std::mutex> lock(seen_mutex);seen.insert(std::this_thread::get_id());} out.Reference(args.data[0]); });
 CreateScalarFunctionInfo sinfo(mark);con.context->RegisterFunction(sinfo);
 Check(con,"CREATE TABLE input AS SELECT range AS i FROM range(2000000)");
 auto result=con.Query("SELECT count(*) AS n, min(record), max(record) FROM probe((SELECT mark(i) AS record FROM input ORDER BY i))");
 if(result->HasError()) throw std::runtime_error(result->GetError());
 if(result->GetValue(0,0).GetValue<int64_t>()!=2000001 || calls!=1 || finals!=1 || chunks<2 || seen.size()<2) throw std::runtime_error("proof counts failed");
 std::cout<<"calls="<<calls<<" ordered_chunks="<<chunks<<" final_drains="<<finals<<" upstream_threads="<<seen.size()<<" rows="<<result->GetValue(0,0).ToString()<<"\n";
 return 0;
 }catch(const std::exception &e){std::cerr<<e.what()<<"\n";return 1;}
}
