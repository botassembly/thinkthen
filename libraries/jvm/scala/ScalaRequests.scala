import thinkthen.Complete.CallControls
import thinkthen.Requests
import thinkthen.Requests.{QuestionInput, InputSource}
/** Prepared requests only; native complete execution remains with the Java door. */
object ScalaRequests {
  def decide(question: QuestionInput, source: InputSource, controls: CallControls): Requests.CompleteRequest = Requests.decide(question, source, controls)
  def choose(question: QuestionInput, source: InputSource, controls: CallControls): Requests.CompleteRequest = Requests.choose(question, source, controls)
  def tag(question: QuestionInput, source: InputSource, controls: CallControls): Requests.CompleteRequest = Requests.tag(question, source, controls)
  def score(question: QuestionInput, source: InputSource, controls: CallControls): Requests.CompleteRequest = Requests.score(question, source, controls)
  def filter(question: QuestionInput, source: InputSource, controls: CallControls): Requests.CompleteRequest = Requests.filter(question, source, controls)
  def rank(question: QuestionInput, source: InputSource, controls: CallControls): Requests.CompleteRequest = Requests.rank(question, source, controls)
  def find(question: QuestionInput, source: InputSource, controls: CallControls): Requests.CompleteRequest = Requests.find(question, source, controls)
  def annotate(question: QuestionInput, source: InputSource, controls: CallControls): Requests.CompleteRequest = Requests.annotate(question, source, controls)
  def recognize(question: QuestionInput, source: InputSource, controls: CallControls): Requests.CompleteRequest = Requests.recognize(question, source, controls)
  def relate(question: QuestionInput, source: InputSource, controls: CallControls): Requests.CompleteRequest = Requests.relate(question, source, controls)
}
