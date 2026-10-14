// Generated from RequestFunction and the compiler-derived ABI; do not edit.
#pragma once
namespace tt {
    template<class Owner> class NamedCalls {
        public:
        ResultCall<results::detail::project_decide<false>> decide(const inputs::RequestQuestion& question,
        const inputs::RequestInput& input, const inputs::RequestOptions& options={
        }) const {
            return ResultCall<results::detail::project_decide<false>>(static_cast<const Owner*>(this)->start("decide",
            question, input, options));
        }
        ResultCall<results::detail::project_decide<true>> decide(const inputs::RequestQuestion& question,
        const inputs::RequestInputText& input, const inputs::RequestOptions& options={
        }) const {
            return ResultCall<results::detail::project_decide<true>>(static_cast<const Owner*>(this)->start("decide",
            question, input, options));
        }
        ResultCall<results::detail::project_decide<true>> decide(const inputs::RequestQuestion& question,
        const inputs::RequestInputJson& input, const inputs::RequestOptions& options={
        }) const {
            return ResultCall<results::detail::project_decide<true>>(static_cast<const Owner*>(this)->start("decide",
            question, input, options));
        }
        ResultCall<results::detail::project_choose<false>> choose(const inputs::RequestQuestion& question,
        const inputs::RequestInput& input, const inputs::RequestOptions& options={
        }) const {
            return ResultCall<results::detail::project_choose<false>>(static_cast<const Owner*>(this)->start("choose",
            question, input, options));
        }
        ResultCall<results::detail::project_choose<true>> choose(const inputs::RequestQuestion& question,
        const inputs::RequestInputText& input, const inputs::RequestOptions& options={
        }) const {
            return ResultCall<results::detail::project_choose<true>>(static_cast<const Owner*>(this)->start("choose",
            question, input, options));
        }
        ResultCall<results::detail::project_choose<true>> choose(const inputs::RequestQuestion& question,
        const inputs::RequestInputJson& input, const inputs::RequestOptions& options={
        }) const {
            return ResultCall<results::detail::project_choose<true>>(static_cast<const Owner*>(this)->start("choose",
            question, input, options));
        }
        ResultCall<results::detail::project_tag<false>> tag(const inputs::RequestQuestion& question,
        const inputs::RequestInput& input, const inputs::RequestOptions& options={
        }) const {
            return ResultCall<results::detail::project_tag<false>>(static_cast<const Owner*>(this)->start("tag",
            question, input, options));
        }
        ResultCall<results::detail::project_tag<true>> tag(const inputs::RequestQuestion& question,
        const inputs::RequestInputText& input, const inputs::RequestOptions& options={
        }) const {
            return ResultCall<results::detail::project_tag<true>>(static_cast<const Owner*>(this)->start("tag",
            question, input, options));
        }
        ResultCall<results::detail::project_tag<true>> tag(const inputs::RequestQuestion& question,
        const inputs::RequestInputJson& input, const inputs::RequestOptions& options={
        }) const {
            return ResultCall<results::detail::project_tag<true>>(static_cast<const Owner*>(this)->start("tag",
            question, input, options));
        }
        ResultCall<results::detail::project_score<false>> score(const inputs::RequestQuestion& question,
        const inputs::RequestInput& input, const inputs::RequestOptions& options={
        }) const {
            return ResultCall<results::detail::project_score<false>>(static_cast<const Owner*>(this)->start("score",
            question, input, options));
        }
        ResultCall<results::detail::project_score<true>> score(const inputs::RequestQuestion& question,
        const inputs::RequestInputText& input, const inputs::RequestOptions& options={
        }) const {
            return ResultCall<results::detail::project_score<true>>(static_cast<const Owner*>(this)->start("score",
            question, input, options));
        }
        ResultCall<results::detail::project_score<true>> score(const inputs::RequestQuestion& question,
        const inputs::RequestInputJson& input, const inputs::RequestOptions& options={
        }) const {
            return ResultCall<results::detail::project_score<true>>(static_cast<const Owner*>(this)->start("score",
            question, input, options));
        }
        ResultCall<results::detail::project_filter> filter(const inputs::RequestQuestion& question,
        const inputs::RequestInput& input, const inputs::RequestOptions& options={
        }) const {
            return ResultCall<results::detail::project_filter>(static_cast<const Owner*>(this)->start("filter",
            question, input, options));
        }
        ResultCall<results::detail::project_rank> rank(const inputs::RequestQuestion& question, const inputs::RequestInput& input,
        const inputs::RequestOptions& options={
        }) const {
            return ResultCall<results::detail::project_rank>(static_cast<const Owner*>(this)->start("rank",
            question, input, options));
        }
        ResultCall<results::detail::project_find> find(const inputs::RequestQuestion& question, const inputs::RequestInput& input,
        const inputs::RequestOptions& options={
        }) const {
            return ResultCall<results::detail::project_find>(static_cast<const Owner*>(this)->start("find",
            question, input, options));
        }
        ResultCall<results::detail::project_annotate<false>> annotate(const inputs::RequestQuestion& question,
        const inputs::RequestInput& input, const inputs::RequestOptions& options={
        }) const {
            return ResultCall<results::detail::project_annotate<false>>(static_cast<const Owner*>(this)->start("annotate",
            question, input, options));
        }
        ResultCall<results::detail::project_annotate<true>> annotate(const inputs::RequestQuestion& question,
        const inputs::RequestInputText& input, const inputs::RequestOptions& options={
        }) const {
            return ResultCall<results::detail::project_annotate<true>>(static_cast<const Owner*>(this)->start("annotate",
            question, input, options));
        }
        ResultCall<results::detail::project_annotate<true>> annotate(const inputs::RequestQuestion& question,
        const inputs::RequestInputJson& input, const inputs::RequestOptions& options={
        }) const {
            return ResultCall<results::detail::project_annotate<true>>(static_cast<const Owner*>(this)->start("annotate",
            question, input, options));
        }
        ResultCall<results::detail::project_recognize<false>> recognize(const inputs::RequestQuestion& question,
        const inputs::RequestInput& input, const inputs::RequestOptions& options={
        }) const {
            return ResultCall<results::detail::project_recognize<false>>(static_cast<const Owner*>(this)->start("recognize",
            question, input, options));
        }
        ResultCall<results::detail::project_recognize<true>> recognize(const inputs::RequestQuestion& question,
        const inputs::RequestInputText& input, const inputs::RequestOptions& options={
        }) const {
            return ResultCall<results::detail::project_recognize<true>>(static_cast<const Owner*>(this)->start("recognize",
            question, input, options));
        }
        ResultCall<results::detail::project_recognize<true>> recognize(const inputs::RequestQuestion& question,
        const inputs::RequestInputJson& input, const inputs::RequestOptions& options={
        }) const {
            return ResultCall<results::detail::project_recognize<true>>(static_cast<const Owner*>(this)->start("recognize",
            question, input, options));
        }
        ResultCall<results::detail::project_relate> relate(const inputs::RequestQuestion& question,
        const inputs::RequestInput& input, const inputs::RequestOptions& options={
        }) const {
            return ResultCall<results::detail::project_relate>(static_cast<const Owner*>(this)->start("relate",
            question, input, options));
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
