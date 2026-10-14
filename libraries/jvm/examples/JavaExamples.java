package examples;
import thinkthen.Engine;
import java.util.List;
import java.util.ArrayList;
/** Read each named answer in one line. */
public final class JavaExamples {
    private JavaExamples() {}
    public static List<Object> run(Engine engine) {
        var answers = new ArrayList<Object>();
        answers.add(engine.decide(SampleInputs.decide(), SampleInputs.input("session-decide"), SampleInputs.options()).join().value());
        answers.add(engine.choose(SampleInputs.choose(), SampleInputs.input("session-choose"), SampleInputs.options()).join().value());
        answers.add(engine.tag(SampleInputs.tag(), SampleInputs.input("session-tag"), SampleInputs.options()).join().value());
        answers.add(engine.score(SampleInputs.score(), SampleInputs.input("session-score"), SampleInputs.options()).join().value());
        answers.add(engine.filter(SampleInputs.decide(), SampleInputs.input("session-filter"), SampleInputs.options()).join().value());
        answers.add(engine.rank(SampleInputs.decide(), SampleInputs.input("session-rank"), SampleInputs.options()).join().value());
        answers.add(engine.find(SampleInputs.find(), SampleInputs.units(), SampleInputs.options()).join().value());
        answers.add(engine.annotate(SampleInputs.annotate(), SampleInputs.input("session-annotate"), SampleInputs.options()).join().value());
        answers.add(engine.recognize(SampleInputs.recognize(), SampleInputs.input("Maria Chen"), SampleInputs.options()).join().value());
        var relation = engine.relate(SampleInputs.relate(), SampleInputs.entities(), SampleInputs.options()).join().value();
        var edge = relation.getFirst();
        System.out.println(edge.sourceName() + " -> " + edge.targetName());
        answers.add(relation);
        return java.util.Collections.unmodifiableList(answers);
    }
}
