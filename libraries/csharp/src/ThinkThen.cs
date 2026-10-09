using System.Runtime.InteropServices;
using System.Text;
using System.Text.Json;
using ThinkThen.Inputs;
namespace ThinkThen;
internal static class Native
{
 const string Library="thinkthen";
 [DllImport(Library,CallingConvention=CallingConvention.Cdecl)] internal static extern IntPtr thinkthen_engine_new_with(byte[] settings);
 [DllImport(Library,CallingConvention=CallingConvention.Cdecl)] internal static extern void thinkthen_engine_free(IntPtr engine);
 [DllImport(Library,CallingConvention=CallingConvention.Cdecl)] internal static extern int thinkthen_error_code(IntPtr engine);
 [DllImport(Library,CallingConvention=CallingConvention.Cdecl)] internal static extern int thinkthen_error_retryable(IntPtr engine);
 [DllImport(Library,CallingConvention=CallingConvention.Cdecl)] internal static extern IntPtr thinkthen_error_message(IntPtr engine);
 [DllImport(Library,CallingConvention=CallingConvention.Cdecl)] internal static extern IntPtr thinkthen_error_facts_json(IntPtr engine);
 [DllImport(Library,CallingConvention=CallingConvention.Cdecl)] internal static extern void thinkthen_free_string(IntPtr value);
 [DllImport(Library,CallingConvention=CallingConvention.Cdecl)] internal static extern int thinkthen_question_parse(IntPtr engine,uint role,StringV1 value,out IntPtr question);
 [DllImport(Library,CallingConvention=CallingConvention.Cdecl)] internal static extern void thinkthen_question_free(IntPtr question);
}
public sealed class Failure : Exception
{
 public int Code {get;}
 public FailureKind Kind {get;}
 public bool Retryable {get;}
 public string? FactsJson {get;}
 public Failure(int code,bool retryable,string message,string? factsJson):base(message) {
  if(!Enum.IsDefined(typeof(FailureKind),code))throw new InvalidOperationException("invalid native failure kind");
  Code=code;Kind=(FailureKind)code;Retryable=retryable;FactsJson=factsJson;
 }
}
public sealed partial class Engine : IDisposable
{
 private readonly ReaderWriterLockSlim lifetime=new();
 private IntPtr engine;
 private readonly EngineHandle ownedEngine;
 private static readonly Encoding StrictUtf8=new UTF8Encoding(false,true);
 private Engine(IntPtr pointer){engine=pointer;ownedEngine=new(pointer);}
 public static Engine Open(InputEngineSettings? settings=null) {
  NativeLoader.Initialize();byte[] bytes=(settings??new InputEngineSettings()).ToBytes();Array.Resize(ref bytes,bytes.Length+1);
  IntPtr pointer=Native.thinkthen_engine_new_with(bytes);
  if(pointer==IntPtr.Zero)throw ReadFailure(pointer,Native.thinkthen_error_code(pointer));
  return new Engine(pointer);
 }
 private static byte[] Text(string value)=>StrictUtf8.GetBytes(value);
 private static Failure ReadFailure(IntPtr pointer,int code) {
  int observed=Native.thinkthen_error_code(pointer);bool retryable=Native.thinkthen_error_retryable(pointer)!=0;
  string message=Marshal.PtrToStringUTF8(Native.thinkthen_error_message(pointer))??"missing native error";
  string? facts=Marshal.PtrToStringUTF8(Native.thinkthen_error_facts_json(pointer));
  if(observed!=code)throw new InvalidOperationException($"native error mismatch {code}/{observed}");
  return new Failure(code,retryable,message,facts);
 }
 private static JsonElement ReadJson(IntPtr pointer,nuint length) {
  if(pointer==IntPtr.Zero)throw new InvalidOperationException("missing native output");
  byte[] bytes=new byte[checked((int)length)];Marshal.Copy(pointer,bytes,0,bytes.Length);
  using var document=JsonDocument.Parse(bytes);return document.RootElement.Clone();
 }
 private TResult Live<TResult>(Func<IntPtr,TResult> body) {
  lifetime.EnterReadLock();bool pinned=false;
  try {if(engine==IntPtr.Zero)throw new ObjectDisposedException(nameof(Engine));ownedEngine.DangerousAddRef(ref pinned);return body(engine);}
  finally {if(pinned)ownedEngine.DangerousRelease();lifetime.ExitReadLock();}
 }
 public void Dispose() {
  lifetime.EnterWriteLock();try{if(engine!=IntPtr.Zero){ownedEngine.Dispose();engine=IntPtr.Zero;}}
  finally{lifetime.ExitWriteLock();}
 }
}
