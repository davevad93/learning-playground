import java.io.*;
import java.net.*;

public class ChatClient {
    private static final String SERVER_ADDRESS = "localhost"; // Server IP address
    private static final int SERVER_PORT = 12345; // Server port

    public static void main(String[] args) {
        try (Socket socket = new Socket(SERVER_ADDRESS, SERVER_PORT);
             BufferedReader consoleReader = new BufferedReader(new InputStreamReader(System.in));
             BufferedReader serverReader = new BufferedReader(new InputStreamReader(socket.getInputStream()));
             PrintWriter serverWriter = new PrintWriter(socket.getOutputStream(), true)) {

            System.out.println("Connected to the chat server");

            // Thread for reading messages from the server
            new Thread(new ServerMessageReader(serverReader)).start();

            String userInput;
            // Read input from console and send to the server
            while ((userInput = consoleReader.readLine()) != null) {
                serverWriter.println(userInput); // Send message to server
            }
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    // Runnable for reading server messages in a separate thread
    private static class ServerMessageReader implements Runnable {
        private BufferedReader serverReader;

        public ServerMessageReader(BufferedReader serverReader) {
            this.serverReader = serverReader;
        }

        @Override
        public void run() {
            try {
                String message;
                // Continuously read messages from the server
                while ((message = serverReader.readLine()) != null) {
                    System.out.println(message); // Print server messages to the console
                }
            } catch (IOException e) {
                e.printStackTrace();
            }
        }
    }
}
