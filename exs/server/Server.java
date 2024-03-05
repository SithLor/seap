package exs.server;

import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.net.ServerSocket;
import java.net.Socket;

import exs.client.data;



public class Server {
    public static final short PORT = 1025;
    public static void main(String[] args) {
        try {
            ServerSocket serverSocket = new ServerSocket(PORT);
            System.out.println("Server is listening on port "+PORT);
            
            while (true) {
                Socket socket = serverSocket.accept();
                System.out.println("New client connected");

                BufferedReader in = new BufferedReader(new InputStreamReader(socket.getInputStream()));
                String line;
                String DATA_FROM_CLIENT = "";
                while ((line = in.readLine()) != null) {
                    DATA_FROM_CLIENT = line;
                }
                System.out.println(DATA_FROM_CLIENT);
            }
        } catch (Exception e) {
            
            e.printStackTrace();
        }
        
    }
}