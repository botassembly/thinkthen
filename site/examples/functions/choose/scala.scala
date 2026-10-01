import scala.concurrent.ExecutionContext.Implicits.global
import scala.util.Using
import thinkthen.{Door, Json}

@main def choose(): Unit =
  Using.resource(Door()) { engine =>
    val tt = ScalaFacade(engine)
    val question = "Which team owns this?"
    val teams = """
      {"billing": "Invoices, fees, and refunds.",
       "shipping": "Parcels and delivery.",
       "account": "Logins and passwords."}"""
    val parcel =
      "My parcel went to the wrong address."
    val team = Json.parseObject(tt.call(
      s"""{"choose": ${Json.quote(question)},
          "options": $teams,
          "evidence": ${Json.quote(parcel)}}"""
    )).get("value")
    assert(team == "shipping")
  }
