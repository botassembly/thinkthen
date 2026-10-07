"""Independent cache/2 keys for actual SQL compatibility consumers."""
import hashlib,json,re

def question_keys(url,body,reported_model=None):
    sent=json.loads(body)
    scheme,rest=url.strip().rstrip('/').split('://',1)
    authority,separator,path=rest.partition('/')
    if not authority.startswith('['):
        host,colon,port=authority.partition(':')
        host=re.sub(r'%[0-9A-Fa-f]{2}|[A-Z]',lambda match:match[0] if match[0].startswith('%') else match[0].lower(),host)
        authority=host+colon+port
    address=scheme.lower()+'://'+authority+separator+path
    compact=lambda value:json.dumps(value,separators=(',',':'),ensure_ascii=False)
    reported=sent['model'] if reported_model is None else reported_model
    result=[]
    for _,question in sorted(sent['questions'].items(),key=lambda item:int(item[0][1:])):
        parts=('systemone',address,compact(sent['model']),compact(reported),compact(sent['state']),compact(question))
        framed=b'thinkthen.question-key/2\0'
        for part in parts:
            encoded=part.encode();framed+=len(encoded).to_bytes(8,'big')+encoded
        result.append(hashlib.sha256(framed).hexdigest())
    return result
