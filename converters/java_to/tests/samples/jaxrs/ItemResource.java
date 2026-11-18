package demo;

import jakarta.ws.rs.*;
import jakarta.ws.rs.core.MediaType;

@Path("/items")
@Produces(MediaType.APPLICATION_JSON)
public class ItemResource {
    @GET
    @Path("{id}")
    public String one(@PathParam("id") int id, @QueryParam("q") @DefaultValue("none") String q) {
        return "item " + id + " q=" + q;
    }

    @POST
    public int count(String body) {
        return body.length();
    }
}
