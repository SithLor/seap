package exs.client;

import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.net.HttpURLConnection;
import java.net.URL;

import exs.shared.StringBuilder;

import java.io.PrintWriter;
import java.net.Socket;
//Screen Shot Support:NO
class Funcs {
    public static String GetIpAddr() {
        final String Data = "0.0.0.0";
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
    public static int sizeof(Object a){
        //get the byte size of the object
        String type = Object.class.getTypeName();
        System.out.println(types);
    }
    
}
public class main {
    
    public static void main(String[] args) {
        short ARG_1 = 1;
        Funcs.sizeof(args)
        
    }
    
    private static void log(String d) {
        System.out.println(d);
    }
}

