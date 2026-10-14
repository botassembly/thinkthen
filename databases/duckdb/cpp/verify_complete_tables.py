"""Authorized DuckDB handles decode shared CSV/TSV grammar and keep physical rows."""
import argparse
import json
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'tools'))
from harness import Backend, run
from native_queries import call as native_call


def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('--extension',required=True,type=Path)
    extension=parser.parse_args().extension.resolve(strict=True)
    with tempfile.TemporaryDirectory(prefix='thinkthen-duckdb-tables-') as tmp, Backend() as backend:
        folder=Path(tmp)
        for format,delimiter in [('csv',','),('tsv','\t')]:
            source=folder/('valid.'+format)
            source.write_text('body'+delimiter+'policy\n"Alpha.\nBeta."'+delimiter+'Refund policy\n')
            inputs={'files':{'paths':[str(source)],'format':format},'reading':{'fields':['/body'],'context':'/policy'}}
            def call(inputs):
                question=json.dumps({'decide':'Fits?','threshold':0.95}).replace("'","''")
                query="SELECT value,result,failure,observations FROM "+native_call("filter", "'"+question+"'", inputs)
                result=run([query],backend.base(),extension=extension)
                assert 'rows' in result[0],result
                value,completed,failure,observations=result[0]['rows'][0]
                return {'value':value,'result':completed,'failure':failure,'observations':observations}
            value=call(inputs)
            assert value['value']==[] and value['result']['value'] is False,value
            assert value['observations'][0]['inputs'][0]['source']=={'file':str(source),'first_line':2,'last_line':3},value
            assert json.loads(value['observations'][0]['inputs'][0]['input'])=={'body':'Alpha.\nBeta.','policy':'Refund policy'},value
            for text,options in [('body'+delimiter+'body\nAlpha'+delimiter+'Beta\n',{}),
                                 ('body\nAlpha\n',{'reading':{'unit':'file'}})]:
                source.write_text(text)
                value=call({'files':{'paths':[str(source)],'format':format,'options':options}})
                assert value['failure']['kind']=='usage',value
        assert backend.count()==2,'invalid table readers sent a request'
    print('DuckDB authorized CSV/TSV handles preserve originals, context, multiline locations and false results; invalid tables send nothing')


if __name__=='__main__':
    main()
