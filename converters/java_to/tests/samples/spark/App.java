import static spark.Spark.*;
import spark.Spark;

public class App {
    public static void main(String[] args) {
        get("/hi/:name", (req, res) -> "Hi " + req.params(":name"));
        get("/add", (req, res) -> {
            int a = Integer.parseInt(req.queryParams("a"));
            int b = Integer.parseInt(req.queryParams("b"));
            return "" + (a + b);
        });
    }
}
