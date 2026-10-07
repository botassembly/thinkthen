import thinkthen.Complete.*
import thinkthen.CompleteEngine.CompleteCall
import thinkthen.Requests.QuestionInput
import thinkthen.Requests.InputSource
import thinkthen.Door
/** Kotlin's named typed calls use the native engine with the closed kotlin token. */
class KotlinComplete(settings: String? = null): AutoCloseable, thinkthen.CompleteEngine {
    private val engine = Door.forKotlin(settings)
    fun token(): Door.Token = engine.token()
    fun decide(question: QuestionInput, source: InputSource, controls: CallControls, deadlineMs: Long = -1, token: Door.Token? = null): CompleteCall<DecideRow> = engine.decideComplete(question,source,controls,deadlineMs,token)
    fun choose(question: QuestionInput, source: InputSource, controls: CallControls, deadlineMs: Long = -1, token: Door.Token? = null): CompleteCall<ChooseRow> = engine.chooseComplete(question,source,controls,deadlineMs,token)
    fun tag(question: QuestionInput, source: InputSource, controls: CallControls, deadlineMs: Long = -1, token: Door.Token? = null): CompleteCall<TagRow> = engine.tagComplete(question,source,controls,deadlineMs,token)
    fun score(question: QuestionInput, source: InputSource, controls: CallControls, deadlineMs: Long = -1, token: Door.Token? = null): CompleteCall<ScoreRow> = engine.scoreComplete(question,source,controls,deadlineMs,token)
    fun filter(question: QuestionInput, source: InputSource, controls: CallControls, deadlineMs: Long = -1, token: Door.Token? = null): CompleteCall<FilterRow> = engine.filterComplete(question,source,controls,deadlineMs,token)
    fun rank(question: QuestionInput, source: InputSource, controls: CallControls, deadlineMs: Long = -1, token: Door.Token? = null): CompleteCall<RankRow> = engine.rankComplete(question,source,controls,deadlineMs,token)
    fun find(question: QuestionInput, source: InputSource, controls: CallControls, deadlineMs: Long = -1, token: Door.Token? = null): CompleteCall<FindRow> = engine.findComplete(question,source,controls,deadlineMs,token)
    fun annotate(question: QuestionInput, source: InputSource, controls: CallControls, deadlineMs: Long = -1, token: Door.Token? = null): CompleteCall<AnnotateRow> = engine.annotateComplete(question,source,controls,deadlineMs,token)
    fun recognize(question: QuestionInput, source: InputSource, controls: CallControls, deadlineMs: Long = -1, token: Door.Token? = null): CompleteCall<RecognizeRow> = engine.recognizeComplete(question,source,controls,deadlineMs,token)
    fun relate(question: QuestionInput, source: InputSource, controls: CallControls, deadlineMs: Long = -1, token: Door.Token? = null): CompleteCall<RelateRow> = engine.relateComplete(question,source,controls,deadlineMs,token)
    override fun close() = engine.close()
    override fun decideBatch(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long,token:Door.Token?): thinkthen.NativeBatch<DecideRow> = engine.decideBatch(q,s,c,deadlineMs,token)

    override fun chooseBatch(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long,token:Door.Token?): thinkthen.NativeBatch<ChooseRow> = engine.chooseBatch(q,s,c,deadlineMs,token)

    override fun tagBatch(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long,token:Door.Token?): thinkthen.NativeBatch<TagRow> = engine.tagBatch(q,s,c,deadlineMs,token)

    override fun scoreBatch(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long,token:Door.Token?): thinkthen.NativeBatch<ScoreRow> = engine.scoreBatch(q,s,c,deadlineMs,token)

    override fun filterBatch(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long,token:Door.Token?): thinkthen.NativeBatch<FilterRow> = engine.filterBatch(q,s,c,deadlineMs,token)

    override fun annotateBatch(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long,token:Door.Token?): thinkthen.NativeBatch<AnnotateRow> = engine.annotateBatch(q,s,c,deadlineMs,token)

    override fun decideComplete(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long,token:Door.Token?):CompleteCall<DecideRow> = decide(q,s,c,deadlineMs,token)

    override fun chooseComplete(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long,token:Door.Token?):CompleteCall<ChooseRow> = choose(q,s,c,deadlineMs,token)

    override fun tagComplete(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long,token:Door.Token?):CompleteCall<TagRow> = tag(q,s,c,deadlineMs,token)

    override fun scoreComplete(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long,token:Door.Token?):CompleteCall<ScoreRow> = score(q,s,c,deadlineMs,token)

    override fun filterComplete(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long,token:Door.Token?):CompleteCall<FilterRow> = filter(q,s,c,deadlineMs,token)

    override fun rankComplete(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long,token:Door.Token?):CompleteCall<RankRow> = rank(q,s,c,deadlineMs,token)

    override fun findComplete(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long,token:Door.Token?):CompleteCall<FindRow> = find(q,s,c,deadlineMs,token)

    override fun annotateComplete(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long,token:Door.Token?):CompleteCall<AnnotateRow> = annotate(q,s,c,deadlineMs,token)

    override fun recognizeComplete(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long,token:Door.Token?):CompleteCall<RecognizeRow> = recognize(q,s,c,deadlineMs,token)

    override fun relateComplete(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long,token:Door.Token?):CompleteCall<RelateRow> = relate(q,s,c,deadlineMs,token)

}
