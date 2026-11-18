package demo;

import java.io.IOException;
import java.io.PrintWriter;
import javax.servlet.annotation.WebServlet;
import javax.servlet.http.*;

@WebServlet("/hello")
public class HelloServlet extends HttpServlet {
    protected void doGet(HttpServletRequest request, HttpServletResponse response) throws IOException {
        String name = request.getParameter("name");
        if (name == null) {
            name = "world";
        }
        response.setContentType("text/plain");
        PrintWriter out = response.getWriter();
        out.println("Hello, " + name + "!");
    }
}
