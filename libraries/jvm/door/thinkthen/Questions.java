package thinkthen;
import java.util.List;
import thinkthen.Complete.*;
/** Typed defaults; validation and question-file parsing stay native. */
public final class Questions { private Questions() {}
public static Question create(Function kind, Content text) { return new Question(kind, text, OptionalValue.absent(), OptionalValue.absent(), List.of(), new Rule(RuleKind.DEFAULT, 0.0, 0.0), new Rule(RuleKind.DEFAULT, 0.0, 0.0), OptionalValue.absent(), OptionalValue.absent(), OptionalValue.absent(), false, false, List.of(), List.of(), List.of(), List.of(), OptionalValue.absent(), OptionalValue.absent()); }
public static Question decide(Content text) {return create(Function.DECIDE, text);}
public static Question choose(Content text) {return create(Function.CHOOSE, text);}
public static Question tag(Content text) {return create(Function.TAG, text);}
public static Question score(Content text) {return create(Function.SCORE, text);}
public static Question filter(Content text) {return create(Function.FILTER, text);}
public static Question rank(Content text) {return create(Function.RANK, text);}
public static Question find(Content text) {return create(Function.FIND, text);}
public static Question annotate(Content text) {return create(Function.ANNOTATE, text);}
public static Question recognize(Content text) {return create(Function.RECOGNIZE, text);}
public static Question relate(Content text) {return create(Function.RELATE, text);}
}
