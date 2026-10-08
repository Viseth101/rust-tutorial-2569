public class ErrorHandlingJava {
    public static void main(String[] args) {
        String input = "abc";
        try {
            int number = Integer.parseInt(input);
            number = number * 2;
            System.out.println("Success: " + number);
        } catch (NumberFormatException e) {
            System.out.println("Parsing Failure -> " + e);
        }
    }
}
