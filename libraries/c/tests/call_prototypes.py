"""Create throwaway Dart/PHP callers for the calling-thread producer contract."""
import os
from pathlib import Path
import subprocess
import sys
import tempfile
ROOT = Path(__file__).resolve().parents[3]
sys.path[:0] = [str(ROOT / 'conformance'), str(ROOT / 'conformance/children')]
from children import child_env
from c_parity import Backend

# These files live only in a temporary directory. They are not language package code.
PHP = r'''<?php
$ffi=FFI::cdef('typedef struct { const char *data; size_t len; } Text;
typedef struct {Text function;Text question_json;Text input_json;Text options_json;} Request;
void *thinkthen_engine_new_with(const char *);
void thinkthen_engine_free(void *);
int32_t thinkthen_run_call(void *,const Request *,uint32_t (*)(void *,Text *),void *,void *,void **,void **);
Text thinkthen_call_result_json(void *);Text thinkthen_call_error_message(void *);
void thinkthen_call_result_free(void *);void thinkthen_call_error_free(void *);', $argv[1]);
$threads=FFI::cdef('unsigned long pthread_self(void);');
$caller=$threads->pthread_self();$count=0;$buffers=[];$changed=false;
$text=function($value)use($ffi,&$buffers) {
 $bytes=$ffi->new('char['.(strlen($value)+1).']');FFI::memcpy($bytes,$value,strlen($value));$buffers[]=$bytes;
 $out=$ffi->new('Text');$out->data=FFI::addr($bytes[0]);$out->len=strlen($value);return $out;
};
$request=$ffi->new('Request');$request->function=$text('decide');
$request->question_json=$text('{"kind":"text","text":"Does it pass?"}');
$request->input_json=$text('{"kind":"feed","name":"records"}');$request->options_json=$text('{}');
$callback=function($context,$out)use(&$count,&$changed,$caller,$threads,$text) {
 if($threads->pthread_self()!=$caller){$changed=true;return 2;}
 if($count++)return 1;
 $out[0]=$text('{"original":{"kind":"text","text":"Evidence."}}');return 0;
};
$engine=$ffi->thinkthen_engine_new_with('{"cache":false,"max_retries":0}');
$result=$ffi->new('void *');$error=$ffi->new('void *');
$code=$ffi->thinkthen_run_call($engine,FFI::addr($request),$callback,null,null,FFI::addr($result),FFI::addr($error));
$ffi->thinkthen_engine_free($engine);
if($code){$message=$ffi->thinkthen_call_error_message($error);throw new Exception(FFI::string($message->data,$message->len));}
$json=$ffi->thinkthen_call_result_json($result);$call=json_decode(FFI::string($json->data,$json->len),true,512,JSON_THROW_ON_ERROR);
$ffi->thinkthen_call_result_free($result);
if($changed||$count!=2||$call['facts']['records']!=1)throw new Exception('missing producer/result');
echo "PHP producer calling thread PASS\n";
'''
DART = r'''import 'dart:ffi';
import 'dart:convert';
import 'dart:io';
final class Text extends Struct {
 external Pointer<Uint8> data;
 @Size() external int len;
}
final class Request extends Struct {
 external Text function;
 external Text questionJson;
 external Text inputJson;
 external Text optionsJson;
}
typedef Producer = Uint32 Function(Pointer<Void>, Pointer<Text>);
typedef Run = Int32 Function(Pointer<Void>, Pointer<Request>, Pointer<NativeFunction<Producer>>, Pointer<Void>, Pointer<Void>, Pointer<Pointer<Void>>, Pointer<Pointer<Void>>);
void main(List<String> args) {
 final library=DynamicLibrary.open(args[0]);final process=DynamicLibrary.process();
 final malloc=process.lookupFunction<Pointer<Void> Function(Size),Pointer<Void> Function(int)>('malloc');
 final free=process.lookupFunction<Void Function(Pointer<Void>),void Function(Pointer<Void>)>('free');
 final thread=process.lookupFunction<Uint64 Function(),int Function()>('pthread_self');
 final buffers=<Pointer<Void>>[];
 Pointer<T> allocate<T extends NativeType>(int size){final p=malloc(size);if(p==nullptr)throw StateError('allocation');buffers.add(p);return p.cast<T>();}
 Pointer<Uint8> bytes(String value){final encoded=utf8.encode(value);final p=allocate<Uint8>(encoded.length+1);p.asTypedList(encoded.length+1).setAll(0,[...encoded,0]);return p;}
 void text(Text target,String value){target.data=bytes(value);target.len=utf8.encode(value).length;}
 final request=allocate<Request>(sizeOf<Request>());
 text(request.ref.function,'decide');text(request.ref.questionJson,'{"kind":"text","text":"Does it pass?"}');
 text(request.ref.inputJson,'{"kind":"feed","name":"records"}');text(request.ref.optionsJson,'{}');
 final caller=thread();var count=0;var changed=false;
 final callback=NativeCallable<Producer>.isolateLocal((Pointer<Void> context,Pointer<Text> out){
  if(thread()!=caller){changed=true;return 2;}if(count++>0)return 1;
  text(out.ref,'{"original":{"kind":"text","text":"Evidence."}}');return 0;
 },exceptionalReturn:2);
 final engineNew=library.lookupFunction<Pointer<Void> Function(Pointer<Uint8>),Pointer<Void> Function(Pointer<Uint8>)>('thinkthen_engine_new_with');
 final engineFree=library.lookupFunction<Void Function(Pointer<Void>),void Function(Pointer<Void>)>('thinkthen_engine_free');
 final run=library.lookupFunction<Run,int Function(Pointer<Void>,Pointer<Request>,Pointer<NativeFunction<Producer>>,Pointer<Void>,Pointer<Void>,Pointer<Pointer<Void>>,Pointer<Pointer<Void>>)>('thinkthen_run_call');
 final resultJson=library.lookupFunction<Text Function(Pointer<Void>),Text Function(Pointer<Void>)>('thinkthen_call_result_json');
 final resultFree=library.lookupFunction<Void Function(Pointer<Void>),void Function(Pointer<Void>)>('thinkthen_call_result_free');
 final result=allocate<Pointer<Void>>(sizeOf<Pointer<Void>>());result.value=nullptr;
 final error=allocate<Pointer<Void>>(sizeOf<Pointer<Void>>());error.value=nullptr;
 final engine=engineNew(bytes('{"cache":false,"max_retries":0}'));
 final status=run(engine,request,callback.nativeFunction,nullptr,nullptr,result,error);
 engineFree(engine);callback.close();
 if(status!=0||changed||count!=2)throw StateError('call/callback $status $changed $count');
 final raw=resultJson(result.value);final call=jsonDecode(utf8.decode(raw.data.asTypedList(raw.len)));
 resultFree(result.value);if(call['facts']['records']!=1)throw StateError('missing result');
 for(final pointer in buffers.reversed){free(pointer);}stdout.writeln('Dart producer calling thread PASS');
}
'''


def run(native, scratch):
    shared = native / 'lib' / ('libthinkthen.0.dylib' if sys.platform == 'darwin' else 'libthinkthen.so.0')
    if not shared.exists():
        shared = native / 'lib' / ('libthinkthen.dylib' if sys.platform == 'darwin' else 'libthinkthen.so')
    shared = shared.resolve(strict=True)
    backend_binary = Path(os.environ.get('CARGO_TARGET_DIR', str(ROOT / 'target'))) / 'debug/conformance-backend'
    with tempfile.TemporaryDirectory(prefix='call-host-prototypes-', dir=scratch) as folder:
        folder = Path(folder)
        env = child_env(home=folder / 'home', PATH=os.environ.get('PATH', '/usr/bin:/bin'))
        backend = Backend(backend_binary, env)
        try:
            env.update(THINKTHEN_BASE_URL=f'http://127.0.0.1:{backend.port}/generic/v1',
                       THINKTHEN_API_KEY='sk-conformance-loopback')
            for name, source, command in (('php', PHP, ['php', '-d', 'ffi.enable=true']),
                                           ('dart', DART, ['dart'])):
                path = folder / ('caller.' + name)
                path.write_text(source)
                output = subprocess.run([*command, str(path), str(shared)], env=env, cwd=folder,
                                        capture_output=True, text=True, timeout=120)
                assert output.returncode == 0 and not output.stderr, (name, output.stdout, output.stderr)
                assert 'producer calling thread PASS' in output.stdout, output.stdout
                print(output.stdout.strip(), flush=True)
            assert int(backend.read('count')) == 2
        finally:
            backend.close()
