// Generated from canonical RequestFunction; do not edit.
#nullable enable
using ThinkThen.Inputs;
namespace ThinkThen;
public sealed partial class Engine {
    public Task<OwnedCall> DecideAsync(InputRequestQuestion question, InputRequestInput input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("decide", question, input, options), null, cancellation);
    public Task<OwnedCall> DecideAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("decide", question, FeedInput, options), input,
    cancellation);
    public Task<OwnedCall> ChooseAsync(InputRequestQuestion question, InputRequestInput input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("choose", question, input, options), null, cancellation);
    public Task<OwnedCall> ChooseAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("choose", question, FeedInput, options), input,
    cancellation);
    public Task<OwnedCall> TagAsync(InputRequestQuestion question, InputRequestInput input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("tag", question, input, options), null, cancellation);
    public Task<OwnedCall> TagAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("tag", question, FeedInput, options), input, cancellation);
    public Task<OwnedCall> ScoreAsync(InputRequestQuestion question, InputRequestInput input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("score", question, input, options), null, cancellation);
    public Task<OwnedCall> ScoreAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("score", question, FeedInput, options), input, cancellation);
    public Task<OwnedCall> FilterAsync(InputRequestQuestion question, InputRequestInput input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("filter", question, input, options), null, cancellation);
    public Task<OwnedCall> FilterAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("filter", question, FeedInput, options), input,
    cancellation);
    public Task<OwnedCall> RankAsync(InputRequestQuestion question, InputRequestInput input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("rank", question, input, options), null, cancellation);
    public Task<OwnedCall> RankAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("rank", question, FeedInput, options), input, cancellation);
    public Task<OwnedCall> FindAsync(InputRequestQuestion question, InputRequestInput input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("find", question, input, options), null, cancellation);
    public Task<OwnedCall> FindAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("find", question, FeedInput, options), input, cancellation);
    public Task<OwnedCall> AnnotateAsync(InputRequestQuestion question, InputRequestInput input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("annotate", question, input, options), null, cancellation);
    public Task<OwnedCall> AnnotateAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("annotate", question, FeedInput, options), input,
    cancellation);
    public Task<OwnedCall> RecognizeAsync(InputRequestQuestion question, InputRequestInput input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("recognize", question, input, options), null, cancellation);
    public Task<OwnedCall> RecognizeAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("recognize", question, FeedInput, options), input,
    cancellation);
    public Task<OwnedCall> RelateAsync(InputRequestQuestion question, InputRequestInput input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("relate", question, input, options), null, cancellation);
    public Task<OwnedCall> RelateAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("relate", question, FeedInput, options), input,
    cancellation);
}
