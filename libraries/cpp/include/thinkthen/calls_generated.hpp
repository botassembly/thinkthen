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
            return static_cast<const Owner*>(this)->plan_impl(call);
        }
        UsageStatus finish_usage_status() const {
            return static_cast<const Owner*>(this)->usage_impl(true);
        }
        UsageStatus usage_persistence() const {
            return static_cast<const Owner*>(this)->usage_impl(false);
        }
    };
}
