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
    public static int bytesizeofString(String e){
        int data = e.length() * 2;
        return data;
    }
    public static double sizeof_primitive(Object a){
        //get the byte size of the object
        String type = a.getClass().getTypeName();
        System.out.println(type);

        double size_byte = 0;
        switch (type) {
            case "java.lang.Byte":
                size_byte = 1;
                break;
            case "java.lang.Short":
                size_byte = 2;
                break;
            case "java.lang.Integer":
                size_byte = 4;
                break;
            case "java.lang.Long":
                size_byte = 8;
                break;
            case "java.lang.Float":
                size_byte = 4;
                break;
            case "java.lang.Double":
                size_byte = 8;
                break;
            case "java.lang.Boolean":
                size_byte= 0.125;
                break;
            default:
                break;
        }
        return size_byte;
    }
    
}
public class main {
    
    public static void main(String[] args) {
        byte a_1 = 0;
        short a_2 = 0;
        int a_3 = 0;
        long a_4 = 0;
        float a_5 = 0;
        double a_6 = 0.0;
        boolean a_7 = false;
        System.out.println(Funcs.sizeof_primitive(a_1));        
    }
    
    private static void log(String d) {
        System.out.println(d);
    }
}

