using System.Runtime.CompilerServices;
namespace ThinkThen;
/// <summary>Provides result value.</summary>
public sealed partial class ThinkThenClient
{
    private static async IAsyncEnumerable<T> Keep<T>(IAsyncEnumerable<Row<bool,T>> rows,[EnumeratorCancellation] CancellationToken cancellationToken)
    {
        var failures=new List<IFailed>();
        await foreach(var row in rows.WithCancellation(cancellationToken).ConfigureAwait(false))
        {
            if(row is Answered<bool,T> { Answer:true }) yield return row.Item;
            else if(row is IFailed failed) failures.Add(failed);
        }
        if(failures.Count>0) throw new FailedException(failures);
    }
}
