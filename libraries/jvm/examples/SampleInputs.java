package examples;
import thinkthen.Inputs;
import java.util.List;
import java.util.Map;
/** Typed questions and evidence shared by the three JVM examples. */
public final class SampleInputs {
    private SampleInputs() {}
    public static Inputs.RequestOptions options() { return new Inputs.RequestOptions().batch(new Inputs.RequestBatch(1)); }
    public static Inputs.RequestInput input(String text) { return new Inputs.RequestInputRecords().items(List.of(new Inputs.RequestItem().original(new Inputs.RequestOriginalText().text(text)))); }
    public static Inputs.RequestQuestion decide() { return new Inputs.RequestQuestionText().text("Is it?"); }
    private static Inputs.RequestQuestion authored(Inputs.RequestDefinition value) { return new Inputs.RequestQuestionDefinition().value(value); }
    public static Inputs.RequestQuestion choose() { return authored(new Inputs.RequestDefinitionAuthoredChoose().choose(new Inputs.AuthoredQuestionText("Which?")).options(new Inputs.AuthoredOptions(List.of("first", "second")))); }
    public static Inputs.RequestQuestion tag() { return authored(new Inputs.RequestDefinitionAuthoredTag().tag(new Inputs.AuthoredQuestionText("Which?")).labels(new Inputs.AuthoredLabels(List.of("first", "second")))); }
    public static Inputs.RequestQuestion score() { return authored(new Inputs.RequestDefinitionAuthoredScore().score(new Inputs.AuthoredQuestionText("Which?")).levels(new Inputs.AuthoredLevels(List.of("Low.", "High.")))); }
    public static Inputs.RequestQuestion find() { return authored(new Inputs.RequestDefinitionAuthoredFind().find(new Inputs.AuthoredQuestionText("Which?"))); }
    public static Inputs.RequestInput units() { return new Inputs.RequestInputUnits().items(List.of(new Inputs.RequestItem().original(new Inputs.RequestOriginalText().text("session-find")), new Inputs.RequestItem().original(new Inputs.RequestOriginalText().text("session-find-two")))); }
    public static Inputs.RequestQuestion annotate() { return authored(new Inputs.RequestDefinitionAnnotateDefinition().questions(Map.of("check", new Inputs.AnnotateQuestionDecide().decide(new Inputs.AuthoredQuestionText("Is it?"))))); }
    public static Inputs.RequestQuestion recognize() { return authored(new Inputs.RequestDefinitionRecognizeDefinition().recognize(new Inputs.RecognizeQuestion().kinds(Map.of("person", new Inputs.AuthoredDescription("A person's name."))))); }
    public static Inputs.RequestQuestion relate() { return authored(new Inputs.RequestDefinitionAuthoredRelate().relate(new Inputs.RelateQuestion().relations(List.of(new Inputs.AuthoredRelation().name("caused_by").source("alert").target("alert"))))); }
    public static Inputs.RequestInput entities() { return new Inputs.RequestInputEntities().items(List.of(new Inputs.RequestItem().original(new Inputs.RequestOriginalJson().value(Map.of("name", "First", "kind", "alert"))), new Inputs.RequestItem().original(new Inputs.RequestOriginalJson().value(Map.of("name", "Second", "kind", "alert"))))); }
}
