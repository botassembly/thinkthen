import thinkthen.Complete.*
import thinkthen.CompleteEngine.CompleteCall
import thinkthen.Requests.{QuestionInput,InputSource}
import thinkthen.Door
/** Scala's named typed calls use the native engine with the closed scala token. */
final class ScalaComplete(settings: String = null) extends AutoCloseable, thinkthen.CompleteEngine {
  private val engine = Door.forScala(settings)
  def token(): Door#Token = engine.token()
  def decide(question: QuestionInput, source: InputSource, controls: CallControls, deadlineMs: Long = -1L, token: Door#Token = null): CompleteCall[DecideRow] = engine.decideComplete(question,source,controls,deadlineMs,token)
  def choose(question: QuestionInput, source: InputSource, controls: CallControls, deadlineMs: Long = -1L, token: Door#Token = null): CompleteCall[ChooseRow] = engine.chooseComplete(question,source,controls,deadlineMs,token)
  def tag(question: QuestionInput, source: InputSource, controls: CallControls, deadlineMs: Long = -1L, token: Door#Token = null): CompleteCall[TagRow] = engine.tagComplete(question,source,controls,deadlineMs,token)
  def score(question: QuestionInput, source: InputSource, controls: CallControls, deadlineMs: Long = -1L, token: Door#Token = null): CompleteCall[ScoreRow] = engine.scoreComplete(question,source,controls,deadlineMs,token)
  def filter(question: QuestionInput, source: InputSource, controls: CallControls, deadlineMs: Long = -1L, token: Door#Token = null): CompleteCall[FilterRow] = engine.filterComplete(question,source,controls,deadlineMs,token)
  def rank(question: QuestionInput, source: InputSource, controls: CallControls, deadlineMs: Long = -1L, token: Door#Token = null): CompleteCall[RankRow] = engine.rankComplete(question,source,controls,deadlineMs,token)
  def find(question: QuestionInput, source: InputSource, controls: CallControls, deadlineMs: Long = -1L, token: Door#Token = null): CompleteCall[FindRow] = engine.findComplete(question,source,controls,deadlineMs,token)
  def annotate(question: QuestionInput, source: InputSource, controls: CallControls, deadlineMs: Long = -1L, token: Door#Token = null): CompleteCall[AnnotateRow] = engine.annotateComplete(question,source,controls,deadlineMs,token)
  def recognize(question: QuestionInput, source: InputSource, controls: CallControls, deadlineMs: Long = -1L, token: Door#Token = null): CompleteCall[RecognizeRow] = engine.recognizeComplete(question,source,controls,deadlineMs,token)
  def relate(question: QuestionInput, source: InputSource, controls: CallControls, deadlineMs: Long = -1L, token: Door#Token = null): CompleteCall[RelateRow] = engine.relateComplete(question,source,controls,deadlineMs,token)
  override def close(): Unit = engine.close()
  def decideBatch(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long = -1L,token:Door#Token = null): thinkthen.NativeBatch[DecideRow] = engine.decideBatch(q,s,c,deadlineMs,token)

  def chooseBatch(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long = -1L,token:Door#Token = null): thinkthen.NativeBatch[ChooseRow] = engine.chooseBatch(q,s,c,deadlineMs,token)

  def tagBatch(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long = -1L,token:Door#Token = null): thinkthen.NativeBatch[TagRow] = engine.tagBatch(q,s,c,deadlineMs,token)

  def scoreBatch(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long = -1L,token:Door#Token = null): thinkthen.NativeBatch[ScoreRow] = engine.scoreBatch(q,s,c,deadlineMs,token)

  def filterBatch(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long = -1L,token:Door#Token = null): thinkthen.NativeBatch[FilterRow] = engine.filterBatch(q,s,c,deadlineMs,token)

  def annotateBatch(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long = -1L,token:Door#Token = null): thinkthen.NativeBatch[AnnotateRow] = engine.annotateBatch(q,s,c,deadlineMs,token)

  override def decideComplete(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long,token:Door#Token):CompleteCall[DecideRow] = decide(q,s,c,deadlineMs,token)

  override def chooseComplete(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long,token:Door#Token):CompleteCall[ChooseRow] = choose(q,s,c,deadlineMs,token)

  override def tagComplete(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long,token:Door#Token):CompleteCall[TagRow] = tag(q,s,c,deadlineMs,token)

  override def scoreComplete(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long,token:Door#Token):CompleteCall[ScoreRow] = score(q,s,c,deadlineMs,token)

  override def filterComplete(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long,token:Door#Token):CompleteCall[FilterRow] = filter(q,s,c,deadlineMs,token)

  override def rankComplete(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long,token:Door#Token):CompleteCall[RankRow] = rank(q,s,c,deadlineMs,token)

  override def findComplete(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long,token:Door#Token):CompleteCall[FindRow] = find(q,s,c,deadlineMs,token)

  override def annotateComplete(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long,token:Door#Token):CompleteCall[AnnotateRow] = annotate(q,s,c,deadlineMs,token)

  override def recognizeComplete(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long,token:Door#Token):CompleteCall[RecognizeRow] = recognize(q,s,c,deadlineMs,token)

  override def relateComplete(q:QuestionInput,s:InputSource,c:CallControls,deadlineMs:Long,token:Door#Token):CompleteCall[RelateRow] = relate(q,s,c,deadlineMs,token)

}
