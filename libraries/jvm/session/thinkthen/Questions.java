package thinkthen;

import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

/** Small authored questions; Rust validates grammar, limits and function matching. */
public final class Questions {
    private Questions() {}
    private static Inputs.RequestQuestion authored(Inputs.RequestDefinition definition) { return new Inputs.RequestQuestionDefinition().value(definition); }
    public static Inputs.RequestQuestion decide(String text) { return new Inputs.RequestQuestionText().text(text); }
    public static Inputs.RequestQuestion choose(String text, List<String> options) { return authored(new Inputs.RequestDefinitionAuthoredChoose().choose(new Inputs.AuthoredQuestionText(text)).options(new Inputs.AuthoredOptions(options))); }
    public static Inputs.RequestQuestion choose(String text, Options options) { return authored(new Inputs.RequestDefinitionAuthoredChoose().choose(new Inputs.AuthoredQuestionText(text)).options(new Inputs.AuthoredOptions(options.values))); }
    public static Inputs.RequestQuestion tag(String text, List<String> labels) { return authored(new Inputs.RequestDefinitionAuthoredTag().tag(new Inputs.AuthoredQuestionText(text)).labels(new Inputs.AuthoredLabels(labels))); }
    public static Inputs.RequestQuestion score(String text, List<String> levels) { return authored(new Inputs.RequestDefinitionAuthoredScore().score(new Inputs.AuthoredQuestionText(text)).levels(new Inputs.AuthoredLevels(levels))); }
    public static Inputs.RequestQuestion find(String text) { return authored(new Inputs.RequestDefinitionAuthoredFind().find(new Inputs.AuthoredQuestionText(text))); }
    public static Inputs.RequestQuestion annotate(Map<String,Inputs.AnnotateQuestion> questions) { return authored(new Inputs.RequestDefinitionAnnotateDefinition().questions(questions)); }
    public static Inputs.RequestQuestion recognize(Map<String,String> kinds) { var values = new LinkedHashMap<String,Inputs.AuthoredDescription>(); kinds.forEach((name, description) -> values.put(name, new Inputs.AuthoredDescription(description))); return authored(new Inputs.RequestDefinitionRecognizeDefinition().recognize(new Inputs.RecognizeQuestion().kinds(values))); }
    public static Inputs.RequestQuestion relate(List<Inputs.AuthoredRelation> relations) { return authored(new Inputs.RequestDefinitionAuthoredRelate().relate(new Inputs.RelateQuestion().relations(relations))); }
    /** Description options retain insertion order across all three JVM languages. */
    public static final class Options {
        private final Map<String,Inputs.AuthoredDescription> values = new LinkedHashMap<>();
        public Options option(String name, String description) { values.put(name, new Inputs.AuthoredDescription(description)); return this; }
    }
}
