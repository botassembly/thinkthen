using System.Runtime.InteropServices;
using System.Text.Json;
using Microsoft.Win32.SafeHandles;
using ThinkThen.Inputs;
namespace ThinkThen;
internal sealed class QuestionHandle : SafeHandleZeroOrMinusOneIsInvalid
{
    internal QuestionHandle(IntPtr pointer) : base(true) => SetHandle(pointer);
    protected override bool ReleaseHandle() { Native.thinkthen_question_free(handle); return true; }
}
public sealed partial class Engine
{
    private readonly object authoredErrorGuard = new();
    /// <summary>Admit authored data with the native parser and return its generated typed definition.</summary>
    public InputRequestQuestionDefinition ParseQuestion(AuthoredQuestionKind kind, string authoredJson)
    {
        ArgumentNullException.ThrowIfNull(authoredJson);
        byte[] bytes = Text(authoredJson);
        lock (authoredErrorGuard) return Live(pointer => {
            var pin = GCHandle.Alloc(bytes,GCHandleType.Pinned);
            try {
                int code = Native.thinkthen_question_parse(pointer,(uint)kind,
                    new StringV1 { data = pin.AddrOfPinnedObject(), len = (nuint)bytes.Length },out var question);
                if (code != 0) throw ReadFailure(pointer,code);
                using var owned = new QuestionHandle(question);
                using var document = JsonDocument.Parse(bytes, new JsonDocumentOptions { MaxDepth = int.MaxValue });
                return new InputRequestQuestionDefinition { Value = InputRequestDefinition.Read(document.RootElement) };
            } finally { pin.Free(); }
        });
    }
}
