"""Owning native SQL call costs, missing usage, overflow, cache and replay."""
import json,os,sys,tempfile,threading
from http.server import BaseHTTPRequestHandler,ThreadingHTTPServer
from pathlib import Path
from parity import execute
from c_parity import ERRORS

class Listener(ThreadingHTTPServer):
    count=0
    usage={'input_tokens':1,'output_tokens':1}
    wrong=False
class Reply(BaseHTTPRequestHandler):
    def log_message(self,*args):pass
    def do_POST(self):
        request=json.loads(self.rfile.read(int(self.headers['Content-Length'])))
        self.server.count+=1
        answer={'model':request['model'],'answers':{key:{'type':'noul','noul':0.9} for key in request['questions']}}
        if self.server.wrong:answer['answers']={}
        if self.server.usage is not None:answer['usage']=self.server.usage
        body=json.dumps(answer,separators=(',',':')).encode()
        self.send_response(200);self.send_header("Connection", "close");self.send_header('Content-Length',str(len(body)));self.end_headers();self.wfile.write(body)

def main():
    consumer=sys.argv[1]
    with tempfile.TemporaryDirectory(prefix='thinkthen-sql-facts-') as tmp:
        home=Path(tmp)
        env={'PATH':os.environ['PATH'],'HOME':tmp,'XDG_CONFIG_HOME':tmp+'/config','XDG_CACHE_HOME':tmp+'/cache','XDG_STATE_HOME':tmp+'/state','LANG':'C.UTF-8','LD_LIBRARY_PATH':os.environ.get('LD_LIBRARY_PATH',''),'THINKTHEN_API_KEY':'sk-conformance-loopback'}
        server=Listener(('127.0.0.1',0),Reply)
        thread=threading.Thread(target=server.serve_forever,daemon=True);thread.start()
        base=f'http://127.0.0.1:{server.server_port}/v1'
        host=None
        if consumer=='postgresql':
            from postgresql_host import PostgresqlHost
            host=PostgresqlHost();host.start({**env,'THINKTHEN_BASE_URL':base})
        try:
            defaults={'base_url':base,'model':'jev-1.13.0','cache':False,'batch':1,'max_retries':0,'usd_per_million_input':'0.25','usd_per_million_output':'0.25'}
            def call(settings=None,records=1,inputs=None):
                before=server.count
                frame={'held_cancel':False,'verb':'decide','question':'Does this ask for a refund?','inputs':inputs if inputs is not None else {'records':[{'text':f'Refund {i}.'} for i in range(records)],'attempts':True},'controls':{},'engine_settings':{**defaults,**(settings or {})}}
                got=execute(consumer,frame,env,home,None)
                assert got.get('facts',{}).get('requests_sent',0)==server.count-before,got
                return got
            live=call({'cache':str(home/'saved')})
            assert 'facts' in live,live
            assert live['facts']['estimated_cost_usd']=='0.000001'
            assert live['facts']['input_tokens']==live['facts']['output_tokens']==1
            assert len(live['facts']['attempts'])==1
            cached=call({'cache':str(home/'saved')})
            recorded=call({'record':str(home/'recorded')})
            assert recorded['facts']['estimated_cost_usd']=='0.000001'
            replay=call({'replay':str(home/'recorded')})
            for held in (cached,replay):
                assert held['facts']['estimated_cost_usd']=='0.000000' and held['facts']['requests_sent']==0
                assert held['facts']['attempts']==[]
            assert cached['rows'][0]['answer_id']==live['rows'][0]['answer_id']
            assert replay['rows'][0]['answer_id']==recorded['rows'][0]['answer_id']
            assert len({item['call_id'] for item in (live,cached,recorded,replay)})==4
            server.usage=None
            missing=call();assert 'estimated_cost_usd' not in missing['facts']
            server.usage={'input_tokens':18446744073709551615,'output_tokens':1}
            overflow=call(records=2)
            assert overflow['requests_sent']==2 and 'estimated_cost_usd' not in overflow['facts']
            assert 'input_tokens' not in overflow['facts']
            server.usage={'input_tokens':1,'output_tokens':1};server.wrong=True
            failure=call()
            assert failure['code']==ERRORS['backend'] and failure['facts']['requests_sent']==1
            assert failure['facts']['estimated_cost_usd']=='0.000001' and len(failure['facts']['attempts'])==1
            assert server.count==6,server.count
            # Caller-supplied terminal errors may stop reading, never supply started facts.
            terminal={'error':{'kind':'local','message':'owned reader stopped','retryable':False,'stopped':{'cause':'local','retryable':False}}}
            descriptors=[{'text':'Refund before stop.'},{'read_error':terminal},{'text':'Never sent after stop.'}]
            eager=call(inputs={'records':descriptors,'attempts':True})
            assert eager['code']==ERRORS['local'] and 'facts' not in eager,eager
            server.wrong=False
            incremental=call(inputs={'records':descriptors,'attempts':True,'incremental':True})
            assert incremental['code']==ERRORS['local'] and incremental['requests_sent']==1,incremental
            assert incremental['stopped_at']==2 and len(incremental['completed'])==1,incremental
            assert incremental['facts']['estimated_cost_usd']=='0.000001' and len(incremental['facts']['attempts'])==1
            forged=[]
            for extra in ({'facts':live['facts']},{'call_id':'forged'},{'attempts':[]}):forged.append({**terminal,**extra})
            for key,value in (('retryable',True),('call_id','forged'),('kind','backend')):
                forged.append({'error':{**terminal['error'],key:value}})
            for key,value in (('at',1),('status',200),('retryable',True),('cause','backend')):
                forged.append({'error':{**terminal['error'],'stopped':{**terminal['error']['stopped'],key:value}}})
            for error in forged:
                refusal=call(inputs={'records':[{'read_error':error}]})
                assert refusal['code']==ERRORS['usage'] and 'facts' not in refusal,refusal
            assert server.count==7,server.count
            print(consumer+': exact owning costs, incomplete usage, overflow, cache/replay, started facts and terminal reader boundary pass')
        finally:
            if host:host.stop()
            server.shutdown();server.server_close();thread.join()
if __name__=='__main__':main()
