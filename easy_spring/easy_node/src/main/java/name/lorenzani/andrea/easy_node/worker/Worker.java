package name.lorenzani.andrea.easy_node.worker;

public class Worker {

    public static Integer[] sort(Integer[] input) {
        for (int i = 1; i < input.length; i++) {
            // Start comparing current element with every element before it
            for (int j = i - 1; j > -1; j--) {

                // Swap elements as required
                if (input[j + 1] < input[j]) {
                    Integer swap = input[j+1];
                    input[j+1] = input[j];
                    input[j] = swap;
                }
            }
        }
        return input;

    }
}
