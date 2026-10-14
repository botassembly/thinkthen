// Generated from RequestFunction and the compiler-derived ABI; do not edit.
#pragma once
namespace tt {
    template<class Owner> class NamedCalls {
        public:
        AnswerCall<results::read_decide> decide(const inputs::RequestQuestion& question, const inputs::RequestInput& input,
        const inputs::RequestOptions& options={
        }) const {
            return AnswerCall<results::read_decide>(static_cast<const Owner*>(this)->start("decide", question,
            input, options));
        }
        AnswerCall<results::read_choose> choose(const inputs::RequestQuestion& question, const inputs::RequestInput& input,
        const inputs::RequestOptions& options={
        }) const {
            return AnswerCall<results::read_choose>(static_cast<const Owner*>(this)->start("choose", question,
            input, options));
        }
        AnswerCall<results::read_tag> tag(const inputs::RequestQuestion& question, const inputs::RequestInput& input,
        const inputs::RequestOptions& options={
        }) const {
            return AnswerCall<results::read_tag>(static_cast<const Owner*>(this)->start("tag", question,
            input, options));
        }
        AnswerCall<results::read_score> score(const inputs::RequestQuestion& question, const inputs::RequestInput& input,
        const inputs::RequestOptions& options={
        }) const {
            return AnswerCall<results::read_score>(static_cast<const Owner*>(this)->start("score", question,
            input, options));
        }
        AnswerCall<results::read_filter> filter(const inputs::RequestQuestion& question, const inputs::RequestInput& input,
        const inputs::RequestOptions& options={
        }) const {
            return AnswerCall<results::read_filter>(static_cast<const Owner*>(this)->start("filter", question,
            input, options));
        }
        AnswerCall<results::read_rank> rank(const inputs::RequestQuestion& question, const inputs::RequestInput& input,
        const inputs::RequestOptions& options={
        }) const {
            return AnswerCall<results::read_rank>(static_cast<const Owner*>(this)->start("rank", question,
            input, options));
        }
        AnswerCall<results::read_find> find(const inputs::RequestQuestion& question, const inputs::RequestInput& input,
        const inputs::RequestOptions& options={
        }) const {
            return AnswerCall<results::read_find>(static_cast<const Owner*>(this)->start("find", question,
            input, options));
        }
        AnswerCall<results::read_annotate> annotate(const inputs::RequestQuestion& question, const inputs::RequestInput& input,
        const inputs::RequestOptions& options={
        }) const {
            return AnswerCall<results::read_annotate>(static_cast<const Owner*>(this)->start("annotate",
            question, input, options));
        }
        AnswerCall<results::read_recognize> recognize(const inputs::RequestQuestion& question, const inputs::RequestInput& input,
        const inputs::RequestOptions& options={
        }) const {
            return AnswerCall<results::read_recognize>(static_cast<const Owner*>(this)->start("recognize",
            question, input, options));
        }
        AnswerCall<results::read_relate> relate(const inputs::RequestQuestion& question, const inputs::RequestInput& input,
        const inputs::RequestOptions& options={
        }) const {
            return AnswerCall<results::read_relate>(static_cast<const Owner*>(this)->start("relate", question,
            input, options));
        }
        results::Plan plan(const inputs::RequestCall& call) const {
            const auto& value=call.document();
            auto function=value.at("function").get<std::string>();
            auto question=value.at("question").dump(), input=value.at("input").dump();
            auto options=value.contains("options") ? value.at("options").dump() : std::string("{}");
            thinkthen_named_request_v1 request{
                {
                    function.data(),function.size()
                },{
                    question.data(),question.size()
                },{
                    input.data(),input.size()
                },{
                    options.data(),options.size()
                }
            };
            thinkthen_value* raw=nullptr;
            detail::check_session(thinkthen_request_plan_named_value(static_cast<const Owner*>(this)->native_engine(),
            &request,&raw));
            std::unique_ptr<thinkthen_value,decltype(&thinkthen_value_free)> owned(raw,&thinkthen_value_free);
            const thinkthen_complete_json_v1* view=nullptr;
            detail::check_session(thinkthen_value_view(owned.get(),&view));
            return results::Plan(results::detail::native_value(view));
        }
        UsageStatus finish_usage_status() const {
            auto engine=static_cast<const Owner*>(this)->native_engine();
            thinkthen_complete_usage_persistence_v1 state{
            };
            thinkthen_complete_utf8_v1 advice{
            };
            detail::check_session(thinkthen_engine_finish_usage_status_v1(engine,&state,&advice));
            return {
                static_cast<UsagePersistenceState>(state.kind),advice.data ? std::optional<std::string>(std::string(advice.data,
                advice.len)) : std::nullopt
            };
        }
        UsageStatus usage_persistence() const {
            auto engine=static_cast<const Owner*>(this)->native_engine();
            thinkthen_complete_usage_persistence_v1 state{
            };
            thinkthen_complete_utf8_v1 advice{
            };
            detail::check_session(thinkthen_engine_usage_persistence_v1(engine,&state,&advice));
            return {
                static_cast<UsagePersistenceState>(state.kind),advice.data ? std::optional<std::string>(std::string(advice.data,
                advice.len)) : std::nullopt
            };
        }
    };
}
