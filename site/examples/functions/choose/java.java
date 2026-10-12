import java.util.LinkedHashMap;
import java.util.Map;
import thinkthen.Engine;
import thinkthen.Inputs.AuthoredDescription;
import thinkthen.Inputs.AuthoredOptions;
import thinkthen.Inputs.AuthoredQuestionText;
import thinkthen.Inputs.RequestDefinitionFieldsChoose;
import thinkthen.Inputs.RequestInputText;
import thinkthen.Inputs.RequestQuestionDefinition;
import thinkthen.Results.SessionPacketChooseRow;

public class Choose {
    public static void main(String[] args) {
        var question = "Which team owns this?";
        Map<String, AuthoredDescription> teams =
            new LinkedHashMap<>();
        teams.put("billing", new AuthoredDescription(
            "Invoices, fees, and refunds."));
        teams.put("shipping", new AuthoredDescription(
            "Parcels and delivery."));
        teams.put("account", new AuthoredDescription(
            "Logins and passwords."));
        var owner = new RequestQuestionDefinition().value(
            new RequestDefinitionFieldsChoose()
                .choose(new AuthoredQuestionText(question))
                .options(new AuthoredOptions(teams)));
        var parcel = new RequestInputText().text(
            "My parcel went to the wrong address.");
        var row = SessionPacketChooseRow.class;
        try (var tt = new Engine()) {
            var team = tt.choose(owner, parcel)
                .join().packets().stream()
                .filter(row::isInstance)
                .map(row::cast)
                .findFirst().orElseThrow()
                .value().value().value();
            assert team.equals("shipping");
        }
    }
}
