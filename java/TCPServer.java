import java.io.*;
import java.net.*;
import java.util.*;
import java.util.concurrent.*;

public class TCPServer {
    private static final int PORT = 12345; // Port the server will listen on
    private static Set<String> availableIPs; // Set of available IP addresses
    private static Set<String> assignedIPs; // Set of currently assigned IP addresses
    private static final Object lock = new Object(); // Lock object for thread synchronization

    public static void main(String[] args) {
        // Initialize the sets for available and assigned IPs
        availableIPs = ConcurrentHashMap.newKeySet();
        assignedIPs = ConcurrentHashMap.newKeySet();

        // Populate the available IPs pool
        for (int i = 1; i <= 10; i++) {
            availableIPs.add("192.168.1." + i);
        }

        try (ServerSocket serverSocket = new ServerSocket(PORT)) {
            System.out.println("Server is listening on port " + PORT);

            while (true) {
                // Accept incoming client connections
                Socket socket = serverSocket.accept();
                System.out.println("New client connected");

                // Handle each client in a new thread
                new ClientHandler(socket).start();
            }
        } catch (IOException ex) {
            System.out.println("Server exception: " + ex.getMessage());
            ex.printStackTrace();
        }
    }

    // Inner class to handle client connections
    private static class ClientHandler extends Thread {
        private Socket socket;

        public ClientHandler(Socket socket) {
            this.socket = socket;
        }

        public void run() {
            try (InputStream input = socket.getInputStream();
                 OutputStream output = socket.getOutputStream();
                 BufferedReader reader = new BufferedReader(new InputStreamReader(input));
                 PrintWriter writer = new PrintWriter(output, true)) {

                String text;

                while ((text = reader.readLine()) != null) {
                    System.out.println("Received from client: " + text);

                    if (text.equalsIgnoreCase("request")) {
                        // Handle IP request
                        String ip = assignIP();
                        if (ip != null) {
                            writer.println("Assigned IP: " + ip);
                        } else {
                            writer.println("No IP available");
                        }
                    } else if (text.startsWith("release")) {
                        // Handle IP release
                        String[] parts = text.split(" ");
                        if (parts.length == 2) {
                            String ip = parts[1];
                            releaseIP(ip);
                            writer.println("Released IP: " + ip);
                        } else {
                            writer.println("Invalid release command");
                        }
                    } else {
                        // Handle unknown command
                        writer.println("Unknown command");
                    }
                }
            } catch (IOException ex) {
                System.out.println("Server exception: " + ex.getMessage());
                ex.printStackTrace();
            }
        }

        // Method to assign an IP address
        private String assignIP() {
            synchronized (lock) {
                Iterator<String> iterator = availableIPs.iterator();
                if (iterator.hasNext()) {
                    String ip = iterator.next();
                    availableIPs.remove(ip);
                    assignedIPs.add(ip);
                    return ip;
                }
                return null;
            }
        }

        // Method to release an IP address
        private void releaseIP(String ip) {
            synchronized (lock) {
                if (assignedIPs.remove(ip)) {
                    availableIPs.add(ip);
                }
            }
        }
    }
}
