-- Ten independent calls. The caller configures the default backend or the stub.
SELECT 'decide',position,status,answer,p,error,kept FROM thinkthen_decide('SELECT json_quote(''Please refund the duplicate charge.'') AS record','Does the message request a refund?');
SELECT 'choose',position,status,answer,p,error,kept FROM thinkthen_choose('SELECT json_quote(''Please refund the duplicate charge.'') AS record','Which team owns this?','{"options":["billing","shipping"]}');
SELECT 'tag',position,status,answer,p,error,kept FROM thinkthen_tag('SELECT json_quote(''Please refund the duplicate charge.'') AS record','Which topics apply?','{"labels":["refund","delivery"]}');
SELECT 'score',position,status,answer,p,error,kept FROM thinkthen_score('SELECT json_quote(''The account is locked and the customer cannot work.'') AS record','How urgent is this?','{"levels":["low","medium","high"]}');
SELECT 'filter',position,status,answer,p,error,kept FROM thinkthen_filter('SELECT json_quote(''Please refund the duplicate charge.'') AS record','Does the message request a refund?');
SELECT 'rank',position,status,answer,p,error,kept FROM thinkthen_rank('SELECT json_quote(''The account is locked and the customer cannot work.'') AS record','How urgently does this need a reply?');
SELECT 'find',position,status,answer,p,error,kept FROM thinkthen_find('SELECT json_quote(''The invoice is due Friday.'' || char(10) || ''The meeting is Monday.'') AS record','When is the invoice due?');
SELECT 'annotate',position,status,answer,p,error,kept FROM thinkthen_annotate('SELECT json_quote(''Please refund the duplicate charge.'') AS record','{"type":"inline","value":{"questions":{"refund":{"decide":"Does the message request a refund?"}}}}');
SELECT 'recognize',position,status,answer,p,error,kept FROM thinkthen_recognize('SELECT json_quote(''Ada works at Acme.'') AS record','{"type":"inline","value":["person","organization"]}');
SELECT 'relate',position,status,answer,p,error,kept FROM thinkthen_relate('SELECT json_quote(''Ada works at Acme.'') AS record','{"type":"inline","value":[{"name":"works_for","source":"person","target":"organization"}]}');
