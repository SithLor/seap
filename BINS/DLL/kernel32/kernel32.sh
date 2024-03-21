objdump -D  ./kernel32.dll  > kernel32.objdump.att.asm
objdump -x  ./kernel32.dll  > kernel32.objdump.header
objdump -f  ./kernel32.dll  > kernel32.objdump.fileheader
objdump -p  ./kernel32.dll  > kernel32.objdump.privatefileheader
objdump -t  ./kernel32.dll  > kernel32.objdump.symboltable
#objdump -T  ./kernel32.dll  > kernel32.objdump.dynamicsymboltable
#objdump -R  ./kernel32.dll  > kernel32.objdump.dynamicrelocationtable
objdump -r  ./kernel32.dll  > kernel32.objdump.relocationtable
