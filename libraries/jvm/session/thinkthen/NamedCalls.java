// Generated from the shared Rust request graph. Do not edit.
package thinkthen;
import java.util.concurrent.CompletableFuture;
public abstract class NamedCalls {
protected abstract CompletableFuture<Engine.OwnedCall> named(Inputs.RequestFunction function, Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options);
public final CompletableFuture<Engine.OwnedCall> decide(Inputs.RequestQuestion question, Inputs.RequestInput input) { return decide(question, input, null); }
public final CompletableFuture<Engine.OwnedCall> decide(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return named(Inputs.RequestFunction.DECIDE, question, input, options); }
public final CompletableFuture<Engine.OwnedCall> choose(Inputs.RequestQuestion question, Inputs.RequestInput input) { return choose(question, input, null); }
public final CompletableFuture<Engine.OwnedCall> choose(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return named(Inputs.RequestFunction.CHOOSE, question, input, options); }
public final CompletableFuture<Engine.OwnedCall> tag(Inputs.RequestQuestion question, Inputs.RequestInput input) { return tag(question, input, null); }
public final CompletableFuture<Engine.OwnedCall> tag(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return named(Inputs.RequestFunction.TAG, question, input, options); }
public final CompletableFuture<Engine.OwnedCall> score(Inputs.RequestQuestion question, Inputs.RequestInput input) { return score(question, input, null); }
public final CompletableFuture<Engine.OwnedCall> score(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return named(Inputs.RequestFunction.SCORE, question, input, options); }
public final CompletableFuture<Engine.OwnedCall> filter(Inputs.RequestQuestion question, Inputs.RequestInput input) { return filter(question, input, null); }
public final CompletableFuture<Engine.OwnedCall> filter(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return named(Inputs.RequestFunction.FILTER, question, input, options); }
public final CompletableFuture<Engine.OwnedCall> rank(Inputs.RequestQuestion question, Inputs.RequestInput input) { return rank(question, input, null); }
public final CompletableFuture<Engine.OwnedCall> rank(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return named(Inputs.RequestFunction.RANK, question, input, options); }
public final CompletableFuture<Engine.OwnedCall> find(Inputs.RequestQuestion question, Inputs.RequestInput input) { return find(question, input, null); }
public final CompletableFuture<Engine.OwnedCall> find(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return named(Inputs.RequestFunction.FIND, question, input, options); }
public final CompletableFuture<Engine.OwnedCall> annotate(Inputs.RequestQuestion question, Inputs.RequestInput input) { return annotate(question, input, null); }
public final CompletableFuture<Engine.OwnedCall> annotate(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return named(Inputs.RequestFunction.ANNOTATE, question, input, options); }
public final CompletableFuture<Engine.OwnedCall> recognize(Inputs.RequestQuestion question, Inputs.RequestInput input) { return recognize(question, input, null); }
public final CompletableFuture<Engine.OwnedCall> recognize(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return named(Inputs.RequestFunction.RECOGNIZE, question, input, options); }
public final CompletableFuture<Engine.OwnedCall> relate(Inputs.RequestQuestion question, Inputs.RequestInput input) { return relate(question, input, null); }
public final CompletableFuture<Engine.OwnedCall> relate(Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) { return named(Inputs.RequestFunction.RELATE, question, input, options); }
}
