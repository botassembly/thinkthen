package thinkthen;
import java.util.List;
import java.util.Map;

/** Caller builders retain authored names across ordinary Java, Kotlin and Scala use. */
public final class AuthoredOrderCases {
    private AuthoredOrderCases() {}
    private static final List<String> NAMES = List.of("zebra", "mango", "apple", "tango", "berry");
    public static void check(Inputs.RequestQuestion question, String member) {
        var definition = Values.object(Values.object(Values.json(question)).get("value"));
        if (member.equals("kinds")) definition = Values.object(definition.get("recognize"));
        var authored = Values.object(definition.get(member));
        if (!List.copyOf(authored.keySet()).equals(NAMES)) throw new AssertionError("Authored name order: " + authored.keySet());
        String encoded = Json.write(Values.json(question));
        int previous = -1;
        for (String name : NAMES) {
            int position = encoded.indexOf(Json.quote(name));
            if (position <= previous) throw new AssertionError("Encoded authored order: " + encoded);
            previous = position;
        }
    }
    public static void javaCaller() {
        var descriptions = new Inputs.Ordered<Inputs.AuthoredDescription>()
            .entry("zebra", new Inputs.AuthoredDescription("Z"))
            .entry("mango", new Inputs.AuthoredDescription("M"))
            .entry("apple", new Inputs.AuthoredDescription("A"))
            .entry("tango", new Inputs.AuthoredDescription("T"))
            .entry("berry", new Inputs.AuthoredDescription("B"));
        check(new Inputs.RequestQuestionDefinition().value(new Inputs.RequestDefinitionAuthoredChoose()
            .choose(new Inputs.AuthoredQuestionText("Which?")).options(new Inputs.AuthoredOptions(descriptions))), "options");
        check(new Inputs.RequestQuestionDefinition().value(new Inputs.RequestDefinitionRecognizeDefinition()
            .recognize(new Inputs.RecognizeQuestion().kinds(descriptions))), "kinds");
        var options = new Questions.Options().option("zebra", "Z").option("mango", "M")
            .option("apple", "A").option("tango", "T").option("berry", "B");
        check(Questions.choose("Which?", options), "options");
        check(Questions.recognize(options), "kinds");
        try { descriptions.entry("zebra", new Inputs.AuthoredDescription("other")); }
        catch (IllegalArgumentException expected) { return; }
        throw new AssertionError("Duplicate authored name admitted");
    }
}
