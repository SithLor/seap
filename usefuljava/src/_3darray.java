package src;

import java.util.ArrayList;

//[1,1,1][1,1,1]
public class _3darray {

    public _3darray(Integer xInteger,Integer yInteger,Integer zInteger){
        ArrayList<Integer> _x = new ArrayList();
        ArrayList<Integer> _y = new ArrayList();
        ArrayList<Integer> _z = new ArrayList();
        for (int i = 0; i < xInteger.intValue(); i++) {
            _x.add(i, 0);
        }
        for (int i = 0; i < yInteger.intValue(); i++) {
            _y.add(i, 0);
        }
        for (int i = 0; i < zInteger.intValue(); i++) {
            _z.add(i, 0);
        }
    }
   // public String format(String Sformat, String[] AData){
   //     String replacedFormat = Sformat.replace("{}", "%s");
   //     return String.format(replacedFormat, (Object[]) AData);
   // }
    
}
