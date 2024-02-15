JAVAFILE=$1
javac $JAVAFILE
javap -c $JAVAFILE > $JAVAFILE.c.bytecode
javap -s $JAVAFILE > $JAVAFILE.s.bytecode
javap -p $JAVAFILE > $JAVAFILE.p.bytecode
javap -l $JAVAFILE > $JAVAFILE.l.bytecode
javap -v $JAVAFILE > $JAVAFILE.v.bytecode
javap -c -p -s -l -v $JAVAFILE > $JAVAFILE.cpslv.bytecode
mv $JAVAFILE.c.bytecode ../bytecode/ 
mv $JAVAFILE.s.bytecode ../bytecode/
mv $JAVAFILE.p.bytecode ../bytecode/
mv $JAVAFILE.l.bytecode ../bytecode/
mv $JAVAFILE.v.bytecode ../bytecode/
mv $JAVAFILE.cpslv.bytecode ../bytecode/

 