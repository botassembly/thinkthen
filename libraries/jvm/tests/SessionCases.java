package thinkthen;
import java.nio.file.*;
import java.util.*;
import java.util.concurrent.*;
import java.math.*;
import thinkthen.Inputs.*;
import thinkthen.Results.*;

/** Shared fixtures enter named typed calls; generated getters read native-owned packets. */
public final class SessionCases {
    static Map<String,Object> map(Object value) { return value instanceof Map<?,?> ? Values.object(value) : Map.of(); }
    static List<?> list(Object value) { return value instanceof List<?> values ? values : List.of(); }
    static boolean yes(Map<String,Object> data,String key) { return Boolean.TRUE.equals(data.get(key)); }
    static String text(Map<String,Object> data,String key) { return data.get(key) instanceof String value ? value : ""; }
    public static Map<String,Object> frame(String[] args) throws Exception {
        if(args.length > 1) return Json.parseObject(args[1]);
        var bytes = new java.io.ByteArrayOutputStream(); int next;
        while((next=System.in.read())!=-1 && next!='\n') bytes.write(next);
        return Json.parseObject(bytes.toString(java.nio.charset.StandardCharsets.UTF_8));
    }
    public static Engine engine(Map<String,Object> data, Engine.Surface surface) {
        return new Engine(EngineSettings.fromJson(Json.parse(text(data,"engine_settings"))),surface);
    }
    public static RequestQuestion question(Map<String,Object> data) throws Exception {
        String reference=text(data,"reference");
        return switch(text(data,"loader")) {
            case "named","load_named" -> new RequestQuestionName().name(reference);
            case "reference","load_reference" -> new RequestQuestionReference().reference(reference);
            case "file","load" -> new RequestQuestionFile().path(reference);
            default -> {
                if(text(data,"question_form").equals("file")) yield new RequestQuestionFile().path("fixture-question.json");
                // Raw malformed authored files must still reach the native parser unchanged.
                if(data.get("raw") instanceof String raw) {
                    Path file=Files.createTempFile("authored-question-",".json");
                    Files.writeString(file,raw); file.toFile().deleteOnExit();
                    yield new RequestQuestionFile().path(file.toString());
                }
                var given=new LinkedHashMap<>(map(data.get("question")));
                if(text(data,"verb").equals("find")) given.remove("none");
                yield new RequestQuestionDefinition().value(RequestDefinition.fromJson(given));
            }
        };
    }
    static Map<String,Object> original(Object value,boolean text) {
        var result=new LinkedHashMap<String,Object>(); result.put("kind",text && value instanceof String ? "text" : "json");
        result.put(text && value instanceof String ? "text" : "value",value); return result;
    }
    public static RequestInput input(Map<String,Object> data) throws Exception {
        var paths=list(data.get("paths"));
        if(!paths.isEmpty() && !yes(data,"owned_jsonl")) {
            int unit=((Number)data.get("source_unit")).intValue();
            var reading=new LinkedHashMap<String,Object>(); reading.put("unit",unit==1?"line":unit==2?"window":"file");
            if(unit==2 && data.containsKey("window")) reading.put("window",data.get("window"));
            var source=new LinkedHashMap<String,Object>(); source.put("paths",paths); source.put("reading",reading);
            if(unit==4 || yes(data,"image_reader")) source.put("media","image");
            return new RequestInputSource().source(RequestSource.fromJson(source));
        }
        var images=new ArrayList<Object>();
        for(Object path:list(data.get("image_paths"))) images.add(Map.of("kind","bytes","bytes",Base64.getEncoder().encodeToString(Files.readAllBytes(Path.of((String)path))),"media",data.getOrDefault("media","image/png")));
        if(yes(data,"context_present") && data.get("context")==null && !(data.get("contexts") instanceof List<?>)) {
            var value=new LinkedHashMap<String,Object>();value.put("item",list(data.get("items")).getFirst());value.put("context",null);
            return new RequestInputJson().value(value);
        }
        var records=new ArrayList<RequestItem>(); int index=0;
        for(Object value:list(data.get("items"))) {
            var item=new LinkedHashMap<String,Object>();
            if(!yes(data,"image_only")) item.put("original",original(yes(data,"caption_files")?Files.readString(Path.of("caption-"+index+".txt")):value,yes(data,"text")));
            if(data.get("contexts") instanceof List<?> contexts) item.put("context",contexts.get(index));
            else if(yes(data,"context_present")) item.put("context",data.get("context"));
            if(data.get("candidate_orders") instanceof List<?> orders) item.put("options",list(orders.get(index)).stream().map(name->Map.of("name",name)).toList());
            if(!images.isEmpty()) item.put("images",images);
            records.add(RequestItem.fromJson(item)); index++;
        }
        return new RequestInputRecords().items(records);
    }
    public static RequestOptions options(Map<String,Object> data) {
        var options=new RequestOptions().attempts(true);
        if(yes(data,"context_present") && data.get("context")==null && !(data.get("contexts") instanceof List<?>)) options.field(List.of("/item")).contextField("/context");
        if(data.get("shared_context") instanceof String context) options.context(context);
        if(text(data,"verb").equals("find")) options.none(yes(map(data.get("question")),"none"));
        if(text(map(data.get("operation")),"injection").equals("expired_deadline")) options.deadlineMs(0);
        return options;
    }
    public static CompletableFuture<Engine.OwnedCall> named(Engine engine,String function,RequestQuestion question,RequestInput input,RequestOptions options) {
        return switch(function) {
            case "decide" -> engine.decide(question,input,options); case "choose" -> engine.choose(question,input,options);
            case "tag" -> engine.tag(question,input,options); case "score" -> engine.score(question,input,options);
            case "filter" -> engine.filter(question,input,options); case "rank" -> engine.rank(question,input,options);
            case "find" -> engine.find(question,input,options); case "annotate" -> engine.annotate(question,input,options);
            case "recognize" -> engine.recognize(question,input,options); case "relate" -> engine.relate(question,input,options);
            default -> throw new IllegalArgumentException("unknown fixture function");
        };
    }
    public static Object optional(Presence<?> value) { return value.state()==Presence.State.VALUE ? value.value() : null; }
    static Object property(Object value,String name) {
        if(value==null) return null;
        try { Object result=value.getClass().getMethod(name).invoke(value); return result instanceof Presence<?> p ? optional(p) : result; }
        catch(NoSuchMethodException missing) { return null; }
        catch(ReflectiveOperationException failure) { throw new IllegalStateException(failure); }
    }
    static Object scalar(Object value) { return value instanceof Values.Value typed ? typed.json() : value; }
    static void location(Map<String,Object> row,Object source) { for(String field:List.of("file","firstLine","lastLine")) { Object value=property(source,field); if(value!=null) row.put(field.equals("firstLine")?"first_line":field.equals("lastLine")?"last_line":field,value); } }
    static Map<String,Object> author(Object question) { var result=new LinkedHashMap<String,Object>(); Object name=property(question,"name"),version=property(question,"wordingVersion"); if(name!=null)result.put("name",name);if(version!=null)result.put("wording_version",version);return result; }
    static Object entity(Entity entity) { return Map.of("text",entity.text(),"start",entity.start(),"end",entity.end(),"length",entity.length(),"kind",entity.kind(),"strength",entity.strength()); }
    static Object endpoint(Object value) { return Map.of("name",property(value,"name"),"kind",property(value,"kind")); }
    static Object edge(Object value,boolean recognized) {var result=new LinkedHashMap<String,Object>();result.put("relation",property(value,"relation"));result.put("source",recognized?entity((Entity)property(value,"source")):endpoint(property(value,"source")));result.put("target",recognized?entity((Entity)property(value,"target")):endpoint(property(value,"target")));result.put("probability",property(value,"probability"));if(Boolean.TRUE.equals(property(value,"either")))result.put("either",true);return result;}
    static Object recognized(Recognize value) {var result=new LinkedHashMap<String,Object>();result.put("entities",((List<?>)property(value,"entities")).stream().map(v->entity((Entity)v)).toList());Object relations=property(value,"relations");if(relations!=null)result.put("relations",((List<?>)relations).stream().map(v->edge(v,true)).toList());return result;}
    static Object annotated(AnnotatedField value) { return value.json(); }
    static List<SessionObservationQuestion> questions(Engine.OwnedCall call) {return call.packets().stream().filter(p->p instanceof SessionPacketObservation).map(p->((SessionPacketObservation)p).value()).filter(v->v instanceof SessionObservationQuestion).map(v->(SessionObservationQuestion)v).toList();}
    static Map<String,Object> row(Object value,Object reading,Engine.OwnedCall call) {
        Meta meta=(Meta)property(value,"meta"); var row=new LinkedHashMap<String,Object>();
        row.put("value",scalar(reading));row.put("index",property(value,"index"));row.put("answer_id",scalar(property(value,"answerId")));
        Object origin=optional(meta.origin());row.put("origin",origin==null?null:switch(((Origin)origin).value()){case "live"->1;case "cache"->2;case "replay"->3;default->throw new IllegalStateException("native origin");});
        row.put("answered_by",optional(meta.answeredBy()));row.put("observations",meta.observations().size());row.put("sources",meta.questionSources().size());row.put("input",property(value,"input"));
        row.put("observation_ids",meta.observations().stream().filter(v->v instanceof ObservationObservationId).map(v->((ObservationObservationId)v).observationId().value()).toList());
        if(row.get("index")==null && !(value instanceof Find)) row.put("index",questions(call).stream().filter(q->Objects.equals(scalar(optional(q.detail().answerId())),row.get("answer_id"))).map(SessionObservationQuestion::index).findFirst().orElse(BigInteger.ZERO));
        location(row,property(value,"source"));location(row,value);row.putAll(author(property(value,"question")));
        Object images=property(value,"images");if(images instanceof List<?> given){row.put("image_properties",given.stream().map(v->{Image image=(Image)v;return List.of(image.media().value().equals("image/jpeg")?1:2,image.width(),image.height());}).toList());row.put("images",given.stream().map(v->HexFormat.of().formatHex(Base64.getDecoder().decode(((Image)v).base64()))).toList());}
        Object answer=property(value,"answer");if(answer instanceof Answer a){row.put("answer_kind",a instanceof AnswerYesNo?"yes_no":a instanceof AnswerChoice?"choice":a instanceof AnswerScore?"score":"tag");if(a instanceof AnswerYesNo odds)row.put("probability",odds.probability());else row.put("probabilities",property(a,"probabilities"));}
        row.put("detail_inputs",List.of());
        if(value instanceof Find found){var candidates=found.candidates().state()==Presence.State.VALUE?found.candidates().value():List.<FindCandidate>of();var chosen=candidates.stream().filter(c->Objects.equals(optional(c.index()),optional(found.index()))).findFirst().orElse(null);row.put("input",chosen==null?null:optional(chosen.input()));row.put("probability",chosen==null?null:chosen.probability());row.put("answer_kind","find");row.put("probabilities",found.answer().probabilities());row.put("detail_inputs",candidates.stream().filter(c->c.index().state()==Presence.State.VALUE).map(c->{var result=new LinkedHashMap<String,Object>();result.put("input",optional(c.input()));location(result,optional(c.source()));return result;}).toList());}
        if(value instanceof Annotation annotation)row.put("member_authors",annotation.answers().values().stream().map(v->author(property(v,"question"))).toList());
        if(value instanceof AtomicNonZeroUsize rank && rank.members().state()==Presence.State.VALUE){rankFacts(row,meta);row.put("question_name",optional(rank.questionName()));row.put("members",rank.members().value().stream().map(member->{var child=row(member.result(),member.result().value(),call);child.put("name",member.name());child.put("author",property(member.result().question(),"name"));rankFacts(child,member.result().meta());return child;}).toList());}
        if(value instanceof Relation relation){var sources=new LinkedHashMap<Object,Object>();for(var item:relation.value())for(Object endpoint:List.of(item.source(),item.target()))if(property(endpoint,"ordinal")!=null)sources.putIfAbsent(property(endpoint,"ordinal"),endpoint);row.put("detail_inputs",sources.values().stream().map(source->{var result=new LinkedHashMap<String,Object>();result.put("input",property(source,"record"));location(result,source);return result;}).toList());}
        return row;
    }
    static void rankFacts(Map<String,Object> row,Meta meta){row.put("model",meta.model());row.put("context_digest",optional(meta.contextSha256()));var usage=new LinkedHashMap<String,Object>();if(optional(meta.usage()) instanceof Usage v){if(optional(v.inputTokens())!=null)usage.put("input_tokens",v.inputTokens().value());if(optional(v.outputTokens())!=null)usage.put("output_tokens",v.outputTokens().value());}row.put("usage",usage);row.put("source_batch_sizes",meta.questionSources().stream().map(s->optional(s.batchSize())).toList());}
    public static Map<String,Object> normalize(Engine.OwnedCall call,Map<String,Object> data) {
        var rows=new ArrayList<Map<String,Object>>();
        for(var packet:call.packets())switch(packet){
            case SessionPacketDecideRow p->rows.add(row(p.value(),p.value().value().value(),call));case SessionPacketChooseRow p->rows.add(row(p.value(),optional(p.value().value()),call));
            case SessionPacketTagRow p->rows.add(row(p.value(),p.value().value(),call));case SessionPacketScoreRow p->rows.add(row(p.value(),p.value().value(),call));case SessionPacketFilterRow p->rows.add(row(p.value(),p.value().value(),call));
            case SessionPacketAnnotateRow p->{var reading=new LinkedHashMap<String,Object>();p.value().value().value().forEach((key,value)->reading.put(key,annotated(value)));rows.add(row(p.value(),reading,call));}
            case SessionPacketRankAggregate p->p.value().forEach(v->rows.add(row(v,v.value(),call)));case SessionPacketFindAggregate p->rows.add(row(p.value(),optional(p.value().value()),call));
            case SessionPacketRecognizeAggregate p->p.value().forEach(v->rows.add(row(v,recognized(v.value()),call)));case SessionPacketRelateAggregate p->rows.add(row(p.value(),p.value().value().stream().map(v->edge(v,false)).toList(),call));default->{}
        }
        if(text(data,"verb").equals("filter")) {
            var all=new ArrayList<Map<String,Object>>();var details=questions(call);
            for(var packet:call.packets())if(packet instanceof SessionPacketObservation observation && observation.value() instanceof SessionObservationRow observed){
                SessionQuestionDetail detail=details.stream().filter(q->q.index().equals(observed.index())).reduce((a,b)->b).orElseThrow().detail();
                var selected=rows.stream().filter(r->Objects.equals(r.get("answer_id"),scalar(optional(detail.answerId())))).findFirst();
                if(selected.isPresent()){all.add(selected.get());continue;}
                var decision=(SessionJudgmentDecision)((SessionObservedRowJudgment)observed.value()).value();var source=detail.questionSources().isEmpty()?null:detail.questionSources().getFirst();
                var row=new LinkedHashMap<String,Object>();row.put("value",optional(decision.value()));row.put("index",observed.index());row.put("answer_id",scalar(optional(detail.answerId())));
                row.put("origin",source==null?null:switch(source.origin().value()){case "live"->1;case "cache"->2;case "replay"->3;default->throw new IllegalStateException("native origin");});row.put("answered_by",source==null?null:source.answeredBy());row.put("observations",detail.observations().size());row.put("sources",detail.questionSources().size());row.put("input",optional(detail.input()));
                row.put("observation_ids",detail.observations().stream().filter(v->v instanceof ObservationObservationId).map(v->((ObservationObservationId)v).observationId().value()).toList());row.put("detail_inputs",List.of());row.put("answer_kind","yes_no");location(row,optional(detail.inputSource()));row.putAll(author(detail.question()));
                if(optional(detail.probabilities()) instanceof SessionProbabilitiesYesNo odds)row.put("probability",odds.value());all.add(row);
            }
            rows.clear();rows.addAll(all);
        }
        var output=new LinkedHashMap<String,Object>();output.put("code",0);output.put("schema","thinkthen.result/2");output.put("observations",call.packets().stream().filter(p->p instanceof SessionPacketObservation).count());output.put("rows",rows);
        if(optional(call.terminal().facts()) instanceof Facts facts){output.put("call_id",facts.callId());output.put("requests_sent",facts.requestsSent());output.put("cache_answers",facts.cacheAnswers());output.put("records",facts.records());if(optional(facts.inputTokens())!=null)output.put("input_tokens",facts.inputTokens().value());if(optional(facts.outputTokens())!=null)output.put("output_tokens",facts.outputTokens().value());}
        if(optional(call.terminal().failure()) instanceof CallError error){output.put("code",code(error.error().kind().value()));output.put("message",error.error().message());if(optional(error.error().stopped().at())!=null)output.put("stopped_at",error.error().stopped().at().value());}
        if(yes(data,"incremental"))output.put("completed",rows);
        return Values.object(Values.freeze(output));
    }
    static int code(String kind){return switch(kind){case "usage"->1;case "backend"->2;case "deadline"->3;case "local"->4;case "cancelled"->5;default->6;};}
    public interface Feed extends AutoCloseable {
        int push(RequestSessionDescriptor descriptor); void finish(RequestReaderFailure failure); void cancel(); OwnedSession.Read read(); void close();
    }
    public static Request request(Map<String,Object> data,RequestQuestion question,RequestInput input,RequestOptions options) {
        var call=new LinkedHashMap<String,Object>();call.put("function",data.get("verb"));call.put("question",question.json());call.put("input",input.json());call.put("options",options.json());
        return Request.fromJson(Map.of("schema",RequestVersion.VALUE,"call",call));
    }
    public static boolean manual(Map<String,Object> data) { return yes(data,"incremental") || yes(data,"held_cancel") || text(map(data.get("operation")),"injection").equals("cancel_token"); }
    public static RequestInput sessionInput(Map<String,Object> data,RequestInput input) {return yes(data,"incremental")?new RequestInputFeed().name("input"):input;}
    public static Engine.OwnedCall drain(Feed feed,Map<String,Object> data,RequestInput input) throws Exception {
        var packets=new ArrayList<SessionPacket>();SessionPacketTerminal terminal=null;Thread signal=null;
        try {
            if(text(map(data.get("operation")),"injection").equals("cancel_token"))feed.cancel();
            if(yes(data,"held_cancel")){signal=Thread.ofPlatform().daemon(true).start(()->{try{if(System.in.read()!='!')throw new IllegalStateException("missing cancellation signal");feed.cancel();System.out.println("cancel-fired");System.out.flush();}catch(java.io.IOException error){throw new java.io.UncheckedIOException(error);}});}
            if(yes(data,"incremental")){
                if(yes(data,"owned_jsonl")){
                    String path=(String)list(data.get("paths")).getFirst();long line=0;
                    try(var reader=Files.newBufferedReader(Path.of(path))){String raw;while((raw=reader.readLine())!=null){line++;var descriptor=new RequestSessionDescriptor().item(RequestItem.fromJson(Map.of("original",original(Json.parse(raw),false)))).location(SessionSourceLocation.fromJson(Map.of("file",path,"first_line",line,"last_line",line)));if(!push(feed,descriptor,packets))break;}}
                }else for(Object record:list(Values.object(input.json()).get("items")))if(!push(feed,new RequestSessionDescriptor().item(RequestItem.fromJson(record)),packets))break;
            }
            feed.finish(null);
            while(true){var read=feed.read();if(read.packet()!=null){packets.add(read.packet());if(read.packet() instanceof SessionPacketTerminal done)terminal=done;}if(read.ended())break;if(read.packet()==null)Thread.sleep(1);}
            return new Engine.OwnedCall(packets,packets.stream().filter(p->p instanceof SessionPacketTerminal).map(p->(SessionPacketTerminal)p).reduce((a,b)->b).orElseThrow());
        }finally{feed.close();if(signal!=null){signal.interrupt();signal.join(1000);if(signal.isAlive())throw new IllegalStateException("fixture cancellation signal did not finish");}}
    }
    static boolean push(Feed feed,RequestSessionDescriptor descriptor,List<SessionPacket> packets) throws Exception {
        while(true){int status=feed.push(descriptor);if(status==0)return true;if(status==2)return false;var read=feed.read();if(read.packet()!=null)packets.add(read.packet());if(read.ended())return false;if(read.packet()==null)Thread.sleep(1);}
    }
    static Feed feed(OwnedSession session){return new Feed(){public int push(RequestSessionDescriptor d){return session.tryPush(d);}public void finish(RequestReaderFailure f){session.finish(f);}public void cancel(){session.cancel();}public OwnedSession.Read read(){return session.tryRead();}public void close(){session.close();}};}
    public static SessionPacket packet(Object value){return SessionPacket.read(value);}
    public static Engine.OwnedCall javaCall(List<?> packets,Object terminal){return new Engine.OwnedCall(packets.stream().map(Results.SessionPacket::read).toList(),new SessionPacketTerminal(terminal));}
    public static String failure(int code,String message){return Json.write(Map.of("code",code,"message",message));}
    public static void main(String[] args) throws Exception {
        var data=frame(args);try(var engine=engine(data,Engine.Surface.JAVA)){
            var question=question(data);var input=input(data);var options=options(data);
            if(manual(data)){System.out.println(Json.write(normalize(drain(feed(engine.startSession(request(data,question,sessionInput(data,input),options))),data,input),data)));return;}
            var future=named(engine,text(data,"verb"),question,input,options);
            try{System.out.println(Json.write(normalize(future.join(),data)));}catch(CompletionException failed){if(failed.getCause() instanceof Engine.SessionFailure failure)System.out.println(Json.write(normalize(failure.call(),data)));else if(failed.getCause() instanceof NativeFailure failure)System.out.println(failure(failure.code(),failure.getMessage()));else throw failed;}
        }catch(NativeFailure failure){System.out.println(failure(failure.code(),failure.getMessage()));}
    }
}
