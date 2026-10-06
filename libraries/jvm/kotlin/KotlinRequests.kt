import thinkthen.Complete.CallControls
import thinkthen.Requests
import thinkthen.Requests.QuestionInput
import thinkthen.Requests.InputSource
/** Prepared requests only; native complete execution remains with the Java door. */
object KotlinRequests {
    fun decide(question: QuestionInput, source: InputSource, controls: CallControls): Requests.CompleteRequest = Requests.decide(question, source, controls)
    fun choose(question: QuestionInput, source: InputSource, controls: CallControls): Requests.CompleteRequest = Requests.choose(question, source, controls)
    fun tag(question: QuestionInput, source: InputSource, controls: CallControls): Requests.CompleteRequest = Requests.tag(question, source, controls)
    fun score(question: QuestionInput, source: InputSource, controls: CallControls): Requests.CompleteRequest = Requests.score(question, source, controls)
    fun filter(question: QuestionInput, source: InputSource, controls: CallControls): Requests.CompleteRequest = Requests.filter(question, source, controls)
    fun rank(question: QuestionInput, source: InputSource, controls: CallControls): Requests.CompleteRequest = Requests.rank(question, source, controls)
    fun find(question: QuestionInput, source: InputSource, controls: CallControls): Requests.CompleteRequest = Requests.find(question, source, controls)
    fun annotate(question: QuestionInput, source: InputSource, controls: CallControls): Requests.CompleteRequest = Requests.annotate(question, source, controls)
    fun recognize(question: QuestionInput, source: InputSource, controls: CallControls): Requests.CompleteRequest = Requests.recognize(question, source, controls)
    fun relate(question: QuestionInput, source: InputSource, controls: CallControls): Requests.CompleteRequest = Requests.relate(question, source, controls)
}
