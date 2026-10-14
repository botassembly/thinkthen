package examples;
import thinkthen.Inputs;
import thinkthen.Questions;
import java.util.List;
import java.util.Map;
/** Typed questions and evidence shared by the three JVM examples. */
public final class SampleInputs {
    private SampleInputs() {}
    public static Inputs.RequestOptions options() { return new Inputs.RequestOptions().batch(new Inputs.RequestBatch(1)); }
    public static Inputs.RequestInput input(String text) { return new Inputs.RequestInputRecords().items(List.of(new Inputs.RequestItem().original(new Inputs.RequestOriginalText().text(text)))); }
    public static Inputs.RequestQuestion decide() { return Questions.decide("Is it?"); }
    public static Inputs.RequestQuestion choose() { return Questions.choose("Which?", new Questions.Options().option("first", "First label.").option("second", "Second label.")); }
    public static Inputs.RequestQuestion tag() { return Questions.tag("Which?", List.of("first", "second")); }
    public static Inputs.RequestQuestion score() { return Questions.score("Which?", List.of("Low.", "High.")); }
    public static Inputs.RequestQuestion find() { return Questions.find("Which?"); }
    public static Inputs.RequestInput units() { return new Inputs.RequestInputUnits().items(List.of(new Inputs.RequestItem().original(new Inputs.RequestOriginalText().text("session-find")), new Inputs.RequestItem().original(new Inputs.RequestOriginalText().text("session-find-two")))); }
    public static Inputs.RequestQuestion annotate() { return Questions.annotate(new Inputs.Ordered<Inputs.AnnotateQuestion>().entry("check", new Inputs.AnnotateQuestionDecide().decide(new Inputs.AuthoredQuestionText("Is it?")))); }
    public static Inputs.RequestQuestion recognize() { return Questions.recognize(new Questions.Options().option("person", "A person's name.")); }
    public static Inputs.RequestQuestion relate() { return Questions.relate(List.of(new Inputs.AuthoredRelation().name("caused_by").source("alert").target("alert"))); }
    public static Inputs.RequestInput entities() { return new Inputs.RequestInputEntities().items(List.of(new Inputs.RequestItem().original(new Inputs.RequestOriginalJson().value(Map.of("name", "First", "kind", "alert"))), new Inputs.RequestItem().original(new Inputs.RequestOriginalJson().value(Map.of("name", "Second", "kind", "alert"))))); }
}
