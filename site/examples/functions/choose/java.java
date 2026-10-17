import thinkthen.Engine;
import thinkthen.Inputs.AuthoredDescription;
import thinkthen.Inputs.Ordered;
import thinkthen.Inputs.AuthoredOptions;
import thinkthen.Inputs.AuthoredQuestionText;
import thinkthen.Inputs.RequestDefinitionAuthoredChoose;
import thinkthen.Inputs.RequestInputText;
import thinkthen.Inputs.RequestQuestionDefinition;

public class Choose {
    public static void main(String[] args) {
        var question = "Which team owns this?";
        var teams = new Ordered<AuthoredDescription>();
        teams.entry("billing", new AuthoredDescription(
            "Invoices, fees, and refunds."));
        teams.entry("shipping", new AuthoredDescription(
            "Parcels and delivery."));
        teams.entry("account", new AuthoredDescription(
            "Logins and passwords."));
        var owner = new RequestQuestionDefinition().value(
            new RequestDefinitionAuthoredChoose()
                .choose(new AuthoredQuestionText(question))
                .options(new AuthoredOptions(teams)));
        var parcel = new RequestInputText().text(
            "My parcel went to the wrong address.");

        try (var tt = new Engine()) {
            var team = tt.choose(owner, parcel)
                .join().value();
            assert team.equals("shipping");
        }
    }
}
