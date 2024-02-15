TIME=$(date +%s%3N)
javac ./com/sutil.java
mv ./com/sutil.class ./com/sutil.${TIME}.class
javap -c ./com/sutil.${TIME}.class > ./com/sutil.${TIME}.bytecode
mv ./com/sutil.${TIME}.bytecode ./bytecode/
mv ./com/sutil.${TIME}.class ./bytecode/
