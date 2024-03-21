objdump -D  ./user32.dll  > user32.objdump.att.asm
objdump -x  ./user32.dll  > user32.objdump.header
objdump -f  ./user32.dll  > user32.objdump.fileheader
objdump -p  ./user32.dll  > user32.objdump.privatefileheader
objdump -t  ./user32.dll  > user32.objdump.symboltable
objdump -T  ./user32.dll  > user32.objdump.dynamicsymboltable
objdump -R  ./user32.dll  > user32.objdump.dynamicrelocationtable
objdump -r  ./user32.dll  > user32.objdump.relocationtable
