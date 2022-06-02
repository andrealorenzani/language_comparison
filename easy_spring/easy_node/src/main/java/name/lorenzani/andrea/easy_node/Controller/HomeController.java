package name.lorenzani.andrea.easy_node.Controller;

import name.lorenzani.andrea.easy_node.model.Input;
import name.lorenzani.andrea.easy_node.worker.Worker;
import org.springframework.http.MediaType;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RestController;

import java.util.concurrent.CompletableFuture;

@RestController
public class HomeController {

    @GetMapping("/ciao")
    public String index() {
        return "Greetings from Spring Boot!";
    }

    @PostMapping(path = "/", consumes = MediaType.ALL_VALUE)
    public Integer[] create(@RequestBody(required = true) Input data) {
        Integer[] original = data.getTest();
        long startExec = System.currentTimeMillis();
        Integer[] firstInput = original.clone();
        Integer[] result =  Worker.sort(firstInput);
        long endExec = System.currentTimeMillis();
        CompletableFuture[] allTheCompl = new CompletableFuture[200];
        for(int i=0; i<200; i++) {
            allTheCompl[i] = CompletableFuture.supplyAsync(() -> Worker.sort(original.clone()));
        }
        CompletableFuture.allOf(allTheCompl).thenRun(() -> {
            long endAll = System.currentTimeMillis();
            System.out.println("Total computation: "+(endAll-startExec));
            System.out.println("Single computation: "+(endExec-startExec));
            System.out.println("Thread computation: "+(endAll-endExec));
        });
        return result;
    }

}
