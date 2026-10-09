using System.Runtime.InteropServices;
using ThinkThen.Inputs;
using ThinkThen.Results;
namespace ThinkThen;
public sealed partial class Engine
{
    /// <summary>Preview a canonical atomic request without key or cache reads or sends.</summary>
    public Plan Plan(InputRequest request)
    {
        ArgumentNullException.ThrowIfNull(request);
        byte[] bytes = request.ToBytes();
        return Live(_ =>
        {
            NativeSession.Check(NativeSession.thinkthen_request_plan_json(ownedEngine, bytes,
                (nuint)bytes.Length, out var output, out var length));
            try { return new Plan(ReadJson(output, length)); }
            finally { Native.thinkthen_free_string(output); }
        });
    }
}
